use std::path::Path;

// Tree types ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DirNode {
    pub name: String,
    pub rel_path: String,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone)]
pub struct FileNode {
    pub name: String,
    pub rel_path: String,
}

#[derive(Debug, Clone)]
pub enum TreeNode {
    Dir(DirNode),
    File(FileNode),
}

impl TreeNode {
    fn name(&self) -> &str {
        match self {
            TreeNode::Dir(d) => &d.name,
            TreeNode::File(f) => &f.name,
        }
    }
}

const IGNORED_DIRS: &[&str] = &[
    // VCS internals — always huge, never useful to browse
    ".git", ".hg", ".svn",
    // Build output
    "target",        // Rust
    "node_modules",  // Node.js
    "__pycache__",   // Python
    ".venv", "venv", // Python virtualenvs
    "dist", "build", "out", ".next", ".nuxt", ".svelte-kit",
];

fn is_ignored_dir(name: &str, is_dir: bool) -> bool {
    is_dir && IGNORED_DIRS.contains(&name)
}

/// Scan `root` recursively, returning all visible (non-hidden) entries as a
/// tree. Directories come before files at each level; both are sorted
/// alphabetically.
pub fn scan_tree(root: &Path) -> Vec<TreeNode> {
    read_dir(root, root)
}

fn read_dir(dir: &Path, root: &Path) -> Vec<TreeNode> {
    let mut dirs: Vec<TreeNode> = Vec::new();
    let mut files: Vec<TreeNode> = Vec::new();

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if is_ignored_dir(&name, path.is_dir()) {
            continue;
        }

        let rel = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");

        if path.is_dir() {
            let children = read_dir(&path, root);
            dirs.push(TreeNode::Dir(DirNode { name, rel_path: rel, children }));
        } else {
            files.push(TreeNode::File(FileNode { name, rel_path: rel }));
        }
    }

    dirs.sort_by(|a, b| a.name().cmp(b.name()));
    files.sort_by(|a, b| a.name().cmp(b.name()));
    dirs.extend(files);
    dirs
}

/// Find the best default file to display on startup (README.md first, then
/// any .md file, then any file).
pub fn find_default(tree: &[TreeNode]) -> Option<String> {
    // Prefer root-level README.md
    for node in tree {
        if let TreeNode::File(f) = node {
            if f.name.eq_ignore_ascii_case("readme.md") {
                return Some(f.rel_path.clone());
            }
        }
    }
    // Any .md file (depth-first)
    find_first_ext(tree, "md")
        .or_else(|| find_any_file(tree))
}

fn find_first_ext(tree: &[TreeNode], ext: &str) -> Option<String> {
    for node in tree {
        match node {
            TreeNode::File(f) => {
                if f.name.rsplit('.').next().map(|e| e.eq_ignore_ascii_case(ext)).unwrap_or(false) {
                    return Some(f.rel_path.clone());
                }
            }
            TreeNode::Dir(d) => {
                if let Some(p) = find_first_ext(&d.children, ext) {
                    return Some(p);
                }
            }
        }
    }
    None
}

fn find_any_file(tree: &[TreeNode]) -> Option<String> {
    for node in tree {
        match node {
            TreeNode::File(f) => return Some(f.rel_path.clone()),
            TreeNode::Dir(d) => {
                if let Some(p) = find_any_file(&d.children) {
                    return Some(p);
                }
            }
        }
    }
    None
}

// ── Legacy flat scan (kept for unit tests) ───────────────────────────────────

#[cfg(test)]
pub fn scan(root: &Path) -> Vec<std::path::PathBuf> {
    use walkdir::WalkDir;
    let mut files: Vec<std::path::PathBuf> = WalkDir::new(root)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path()
                    .extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.eq_ignore_ascii_case("md"))
                    .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect();
    files.sort_by(|a, b| {
        a.components().count().cmp(&b.components().count()).then(a.cmp(b))
    });
    files
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn tmpdir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn finds_md_files_recursively() {
        let dir = tmpdir();
        fs::write(dir.path().join("README.md"), "# Hello").unwrap();
        fs::create_dir(dir.path().join("docs")).unwrap();
        fs::write(dir.path().join("docs").join("setup.md"), "setup").unwrap();
        fs::write(dir.path().join("docs").join("notes.txt"), "included now").unwrap();

        // scan() still returns only .md files
        let files = scan(dir.path());
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn returns_empty_for_no_md_files() {
        let dir = tmpdir();
        fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
        assert!(scan(dir.path()).is_empty());
    }

    #[test]
    fn root_files_before_nested() {
        let dir = tmpdir();
        fs::write(dir.path().join("z.md"), "z").unwrap();
        fs::create_dir(dir.path().join("sub")).unwrap();
        fs::write(dir.path().join("sub").join("a.md"), "a").unwrap();

        let files = scan(dir.path());
        assert_eq!(files.len(), 2);
        let depths: Vec<usize> = files.iter().map(|f| f.components().count()).collect();
        assert!(depths[0] <= depths[1]);
    }

    #[test]
    fn scan_tree_shows_all_files() {
        let dir = tmpdir();
        fs::write(dir.path().join("README.md"), "# hi").unwrap();
        fs::write(dir.path().join("main.rs"), "fn main(){}").unwrap();
        fs::create_dir(dir.path().join("src")).unwrap();
        fs::write(dir.path().join("src").join("lib.rs"), "").unwrap();

        let tree = scan_tree(dir.path());
        // Should see: src/ dir + README.md + main.rs
        assert_eq!(tree.len(), 3);
        assert!(matches!(&tree[0], TreeNode::Dir(_))); // dir comes first
    }

    #[test]
    fn hidden_files_are_visible_but_git_is_excluded() {
        let dir = tmpdir();
        fs::write(dir.path().join("visible.md"), "hi").unwrap();
        fs::write(dir.path().join(".hidden.md"), "shown now").unwrap();
        fs::create_dir(dir.path().join(".git")).unwrap();
        fs::write(dir.path().join(".git").join("config"), "").unwrap();
        fs::create_dir(dir.path().join(".github")).unwrap();
        fs::write(dir.path().join(".github").join("CODEOWNERS"), "").unwrap();

        let tree = scan_tree(dir.path());
        // .github/ dir + visible.md + .hidden.md = 3; .git/ excluded
        assert_eq!(tree.len(), 3);
        let names: Vec<&str> = tree.iter().map(|n| n.name()).collect();
        assert!(names.contains(&".github"));
        assert!(names.contains(&".hidden.md"));
        assert!(!names.contains(&".git"));
    }

    #[test]
    fn find_default_prefers_readme() {
        let dir = tmpdir();
        fs::write(dir.path().join("alpha.md"), "").unwrap();
        fs::write(dir.path().join("README.md"), "").unwrap();

        let tree = scan_tree(dir.path());
        let default = find_default(&tree).unwrap();
        assert_eq!(default, "README.md");
    }
}
