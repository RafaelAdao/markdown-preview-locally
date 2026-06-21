use comrak::plugins::syntect::SyntectAdapterBuilder;
use comrak::{markdown_to_html_with_plugins, ExtensionOptions, Options, Plugins, RenderOptions};
use std::{path::Path, sync::OnceLock};
use syntect::{
    easy::HighlightLines,
    highlighting::ThemeSet,
    html::{styled_line_to_highlighted_html, IncludeBackground},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

// Syntax set and theme set are heavy — load once.
static SS: OnceLock<SyntaxSet> = OnceLock::new();
static TS: OnceLock<ThemeSet> = OnceLock::new();

fn ss() -> &'static SyntaxSet {
    SS.get_or_init(SyntaxSet::load_defaults_newlines)
}
fn ts() -> &'static ThemeSet {
    TS.get_or_init(ThemeSet::load_defaults)
}

// ── Theme ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    pub fn syntect_name(&self) -> &'static str {
        match self {
            Theme::Light => "InspiredGitHub",
            Theme::Dark => "base16-ocean.dark",
        }
    }

    pub fn from_str(s: &str) -> Self {
        if s == "dark" { Theme::Dark } else { Theme::Light }
    }
}

// ── GFM markdown rendering ───────────────────────────────────────────────────

pub fn render_to_html(markdown: &str, theme: Theme) -> String {
    let adapter = SyntectAdapterBuilder::new()
        .theme(theme.syntect_name())
        .build();

    let mut plugins = Plugins::default();
    plugins.render.codefence_syntax_highlighter = Some(&adapter);

    let options = Options {
        extension: ExtensionOptions {
            autolink: true,
            table: true,
            tasklist: true,
            strikethrough: true,
            footnotes: true,
            ..Default::default()
        },
        render: RenderOptions {
            unsafe_: true,
            ..Default::default()
        },
        ..Default::default()
    };

    markdown_to_html_with_plugins(markdown, &options, &plugins)
}

// ── Generic file rendering ───────────────────────────────────────────────────

/// Render any file for browser display.
/// `rel_path` is used to construct image src URLs; `abs_path` is where the
/// file is read from. `theme` controls syntax highlighting colours.
pub fn render_for_web(rel_path: &str, abs_path: &Path, theme: Theme) -> String {
    let ext = abs_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    // Markdown → GFM render
    if ext == "md" {
        return match std::fs::read_to_string(abs_path) {
            Ok(md) => render_to_html(&md, theme),
            Err(e) => error_html(&format!("Cannot read file: {e}")),
        };
    }

    // Images → inline img tag pointing at the /file route
    if is_image(&ext) {
        let encoded = url_encode(rel_path);
        let alt = html_escape(abs_path.file_name().and_then(|n| n.to_str()).unwrap_or("image"));
        return format!(
            "<div style=\"text-align:center;padding:32px\">\
             <img src=\"/file?path={encoded}\" alt=\"{alt}\" \
             style=\"max-width:100%;height:auto;border-radius:6px;border:1px solid var(--color-border)\">\
             </div>"
        );
    }

    // Text / source code → syntax highlighted
    match std::fs::read_to_string(abs_path) {
        Ok(content) => {
            let name = abs_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(rel_path);
            let highlighted = highlight_code(&content, &ext, theme);
            format!(
                "<div class=\"code-file-header\">{}</div>{}",
                html_escape(name),
                highlighted
            )
        }
        Err(_) => match std::fs::metadata(abs_path) {
            Ok(m) => format!(
                "<div class=\"binary-info\">\
                 <p>Binary file · {} bytes</p>\
                 <p><a href=\"/file?path={}\">Download</a></p>\
                 </div>",
                m.len(),
                url_encode(rel_path)
            ),
            Err(_) => error_html("Cannot read file"),
        },
    }
}

// ── Syntax highlighting ──────────────────────────────────────────────────────

fn highlight_code(code: &str, ext: &str, theme: Theme) -> String {
    let ss = ss();
    let ts = ts();
    let syntect_theme = &ts.themes[theme.syntect_name()];

    let syntax = ss
        .find_syntax_by_token(ext)
        .unwrap_or_else(|| ss.find_syntax_plain_text());

    let mut hl = HighlightLines::new(syntax, syntect_theme);
    let mut out = String::from("<pre class=\"code-block\"><code>");

    for line in LinesWithEndings::from(code) {
        match hl.highlight_line(line, ss) {
            Ok(ranges) => match styled_line_to_highlighted_html(&ranges, IncludeBackground::No) {
                Ok(html) => out.push_str(&html),
                Err(_) => out.push_str(&html_escape(line)),
            },
            Err(_) => out.push_str(&html_escape(line)),
        }
    }

    out.push_str("</code></pre>");
    out
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn is_image(ext: &str) -> bool {
    matches!(ext, "png" | "jpg" | "jpeg" | "gif" | "webp" | "bmp" | "ico" | "svg")
}

pub fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for byte in s.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'/' | b'~') {
            out.push(byte as char);
        } else {
            out.push_str(&format!("%{byte:02X}"));
        }
    }
    out
}

fn error_html(msg: &str) -> String {
    format!("<p style=\"color:var(--color-danger,#cf222e)\">{}</p>", html_escape(msg))
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_table() {
        let html = render_to_html("| a | b |\n|---|---|\n| 1 | 2 |", Theme::Light);
        assert!(html.contains("<table>"));
    }

    #[test]
    fn renders_task_list() {
        let html = render_to_html("- [ ] todo\n- [x] done", Theme::Light);
        assert!(html.contains(r#"type="checkbox""#));
    }

    #[test]
    fn renders_strikethrough() {
        let html = render_to_html("~~deleted~~", Theme::Light);
        assert!(html.contains("<del>"));
    }

    #[test]
    fn renders_code_light() {
        let html = render_to_html("```rust\nfn main() {}\n```", Theme::Light);
        assert!(html.contains("<span"));
    }

    #[test]
    fn renders_code_dark() {
        let html = render_to_html("```rust\nfn main() {}\n```", Theme::Dark);
        assert!(html.contains("<span"));
    }

    #[test]
    fn highlight_code_plain_fallback() {
        let html = highlight_code("hello world", "unknown_lang_xyz", Theme::Light);
        assert!(html.contains("hello world"));
        assert!(html.contains("<pre"));
    }

    #[test]
    fn url_encode_safe_chars() {
        assert_eq!(url_encode("docs/setup.md"), "docs/setup.md");
        assert_eq!(url_encode("path with spaces"), "path%20with%20spaces");
    }
}
