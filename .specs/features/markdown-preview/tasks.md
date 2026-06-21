# Markdown Preview — Tasks

**Feature:** markdown-preview  
**Status:** Ready to implement  
**Total tasks:** 9

---

## T01 — Rust toolchain upgrade + Cargo scaffold

**What:** Update Rust to latest stable via asdf, then create `Cargo.toml` and the project skeleton.

**Where:** Project root

**Depends on:** Nothing

**Steps:**
1. `asdf install rust latest && asdf global rust latest` (need ≥ 1.70 for clap 4 + axum 0.6)
2. `cargo init --name mdpreview` in `/home/RAFAEL.ADAO/src/markdown-preview-localy`
3. Write `Cargo.toml` with all dependencies from design.md
4. Create `src/` module stubs (empty `mod` declarations in `main.rs`)
5. Create `assets/` directory with placeholder files

**Done when:**
- `cargo build` compiles (even if binary does nothing)
- `rustc --version` shows ≥ 1.70

**Gate:** `cargo build`

**Reqs:** MDPV-01 (prerequisite)

---

## T02 — CLI argument parsing (`src/cli.rs`)

**What:** Implement `Cli` struct with clap derive; parse optional path argument.

**Where:** `src/cli.rs`, `src/main.rs`

**Depends on:** T01

**Steps:**
1. Write `src/cli.rs` with `Cli` struct (optional `path: Option<PathBuf>`)
2. Add path resolution logic: if None → cwd; resolve to absolute; validate exists
3. Detect mode: `Mode::Dir` if path is directory, `Mode::File` if `.md` file, else error
4. In `main.rs`: parse CLI, print resolved path to stdout, exit 0

**Done when:**
- `mdpreview --help` shows usage
- `mdpreview /nonexistent` exits with error message
- `mdpreview ./` prints resolved path

**Gate:** `cargo test cli_tests` (write unit tests for path resolution and mode detection)

**Reqs:** MDPV-01, MDPV-04, MDPV-12

---

## T03 — Directory scanner (`src/scanner.rs`)

**What:** Recursive `.md` file discovery returning sorted `Vec<PathBuf>`.

**Where:** `src/scanner.rs`

**Depends on:** T01

**Steps:**
1. Write `pub fn scan(root: &Path) -> Vec<PathBuf>` using `walkdir`
2. Filter: only `.md` extension (case-insensitive)
3. Sort: alphabetical, with entries closer to root first (by component count, then name)
4. Handle empty directory: return empty vec (caller shows "no files" message)
5. Write unit tests with a temp directory containing nested `.md` files

**Done when:**
- Given dir with `README.md`, `docs/setup.md`, `docs/api.md` → returns all three sorted
- Given empty dir → returns `[]`

**Gate:** `cargo test scanner_tests`

**Reqs:** MDPV-03, MDPV-11

---

## T04 — Markdown renderer (`src/renderer.rs`)

**What:** GFM markdown string → HTML fragment (inner content only, not full page).

**Where:** `src/renderer.rs`, `assets/github-markdown.css`

**Depends on:** T01

**Steps [P] (can run in parallel with T02, T03):**

1. Implement `pub fn render_to_html(markdown: &str) -> String` using comrak:
   - Enable all GFM extensions: `autolink`, `table`, `tasklist`, `strikethrough`, `footnotes`
   - Enable `unsafe_` rendering
   - Wire syntect adapter for fenced code blocks

2. Implement `SyntaxHighlighterAdapter` for comrak using syntect:
   - Use `syntect::parsing::SyntaxSet::load_defaults_newlines()`
   - Use `syntect::highlighting::ThemeSet::load_defaults()`
   - Theme: `InspiredGitHub`
   - Output: HTML with inline styles (no external CSS needed for highlighting)

3. Download `github-markdown.css` from `https://raw.githubusercontent.com/sindresorhus/github-markdown-css/main/github-markdown-light.css` and save to `assets/github-markdown.css`
   - Scope: content is already scoped to `.markdown-body`

4. Write unit tests:
   - Table renders as `<table>`
   - `- [ ]` renders as `<input type="checkbox" disabled>`
   - ~~strike~~ renders as `<del>`
   - Fenced rust block renders with `<span style=...>` syntax highlighting

**Done when:** All renderer unit tests pass

**Gate:** `cargo test renderer_tests`

**Reqs:** MDPV-05, MDPV-06, MDPV-07

---

## T05 — Embedded assets (`src/assets.rs`)

**What:** `include_str!` constants for CSS + JS; `page_template()` function for full HTML page.

**Where:** `src/assets.rs`, `assets/app.js`

**Depends on:** T04 (needs `github-markdown.css` to exist)

**Steps:**
1. Write `assets/app.js` — WebSocket client (see design.md for code sketch):
   - Connect to `ws://${location.host}/ws`
   - On message: parse JSON; if `type === 'reload'` and path matches current → fetch `/render?path=...` and replace `.markdown-body` innerHTML
   - On close: `setTimeout(() => location.reload(), 1500)` for auto-reconnect
   - Track `currentPath` from `<meta name="current-path">` tag

2. Write `src/assets.rs`:
   ```rust
   pub const GITHUB_CSS: &str = include_str!("../assets/github-markdown.css");
   pub const APP_JS: &str = include_str!("../assets/app.js");
   
   pub fn full_page(title: &str, sidebar: &str, content: &str, current_path: &str) -> String
   pub fn content_fragment(html: &str) -> String  // for /render endpoint (no shell)
   ```

3. `full_page()` generates:
   - `<!DOCTYPE html>` with `<meta charset="utf-8">`, `<meta name="current-path" content="...">`
   - `<style>` with layout CSS (sidebar + main panel) + `GITHUB_CSS`
   - `<nav id="sidebar">` with file list links
   - `<main><article class="markdown-body">` with content
   - `<script>` with `APP_JS`

**Done when:** `full_page("README", "<li>...</li>", "<h1>Hello</h1>", "README.md")` returns valid HTML with all sections present

**Gate:** `cargo test assets_tests` (test HTML structure)

**Reqs:** MDPV-02, MDPV-07

---

## T06 — File watcher (`src/watcher.rs`)

**What:** Async task that watches the filesystem and sends reload events via broadcast channel.

**Where:** `src/watcher.rs`

**Depends on:** T01

**Steps [P]:**
1. Write `pub async fn watch(root: PathBuf, tx: broadcast::Sender<String>)`
2. Bridge `notify` (sync) to tokio via `std::sync::mpsc::channel` + `tokio::task::spawn_blocking`
3. Watch mode: `RecursiveMode::Recursive` for dirs, `NonRecursive` for files
4. Filter: only care about `.md` file events (`Modify`, `Create`, `Remove`)
5. Debounce: skip events for the same path within 100ms of the last event
6. Serialize: `serde_json::to_string(&json!({"type": "reload", "path": relative_path}))`
7. Handle `tx.send()` error gracefully (all receivers dropped → just log and continue)

**Done when:**
- Modifying a `.md` file causes one broadcast message within 200ms
- Modifying a non-`.md` file produces no broadcast message

**Gate:** Integration test: spawn watcher on temp dir, write to file, assert broadcast received within 500ms

**Reqs:** MDPV-08

---

## T07 — HTTP server + routes (`src/server.rs`)

**What:** axum 0.6 server with all routes wired up.

**Where:** `src/server.rs`

**Depends on:** T03, T04, T05, T06 (needs scanner, renderer, assets, reload channel)

**Steps:**
1. Define `AppState` with `root: Arc<PathBuf>`, `mode: Arc<Mode>`, `reload_tx: broadcast::Sender<String>`
2. Implement `GET /` handler:
   - Scan files, pick first `.md` (prefer `README.md`)
   - Render it, build sidebar HTML from file list
   - Return `Html(full_page(...))`
3. Implement `GET /render` handler (`?path=relative/path.md`):
   - Validate path is within root (prevent path traversal)
   - Read file, render with `renderer::render_to_html`
   - Return `Html(content_fragment(...))`
4. Implement `GET /ws` WebSocket handler (see design.md for code)
5. Implement `GET /static/app.js` → return `APP_JS` with `Content-Type: application/javascript`
6. Build router with `.layer(Extension(state))`

**Security note:** In `/render`, validate that the resolved path starts with `root`. Reject with 403 if not (prevents `?path=../../etc/passwd`).

**Done when:**
- `GET /` returns 200 with HTML containing `.markdown-body`
- `GET /render?path=README.md` returns HTML fragment
- `GET /render?path=../../etc/passwd` returns 403
- `GET /ws` upgrades to WebSocket

**Gate:** `cargo test server_tests` (use `axum::test` helpers)

**Reqs:** MDPV-01, MDPV-02, MDPV-09, MDPV-11

---

## T08 — Main wiring + port binding (`src/main.rs`)

**What:** Connect all components; handle port conflicts; open browser.

**Where:** `src/main.rs`

**Depends on:** T02, T06, T07

**Steps:**
1. Parse CLI → resolve path → detect mode
2. Create `broadcast::channel::<String>(64)`
3. Try binding `TcpListener` on ports 3000–3009; use first available. If all taken, exit with error
4. Print `Listening on http://localhost:{port}` to stderr
5. `tokio::spawn(watcher::watch(root.clone(), tx.clone()))`
6. `tokio::spawn(server::run(listener, state))`
7. `webbrowser::open(&url)` — non-fatal if it fails (just log warning)
8. `tokio::signal::ctrl_c().await` → exit cleanly

**Done when:**
- `cargo run -- .` starts server, prints URL, opens browser
- Running twice: second invocation uses port 3001
- Ctrl+C exits cleanly with no error output

**Gate:** Manual smoke test (see T09)

**Reqs:** MDPV-01, MDPV-11, MDPV-12

---

## T09 — End-to-end smoke test + install

**What:** Manual verification against a real markdown directory, then `cargo install`.

**Where:** Test with `/home/RAFAEL.ADAO/src/markdown-preview-localy` itself (add test .md files)

**Depends on:** T08

**Steps:**
1. Create `test-docs/` with:
   - `README.md` — contains table, task list `- [ ]`, fenced rust block, ~~strikethrough~~
   - `docs/setup.md` — simple prose
   - `docs/api.md` — code-heavy
2. Run `cargo run -- test-docs/`
3. Verify in browser:
   - Sidebar shows 3 files
   - README.md renders with correct GFM features
   - Clicking sidebar links changes content
   - Editing + saving a file refreshes content within 1s
4. `cargo install --path .`
5. Run `mdpreview test-docs/` from anywhere
6. Verify clean exit on Ctrl+C

**Done when:** All items in spec.md "Success Criteria" are checked off

**Gate:** All spec acceptance criteria verified manually

**Reqs:** All MDPV-*

---

## Execution Order

```
T01 (scaffold)
  ├── T02 (CLI)         ─┐
  ├── T03 (scanner)     ─┤─ all parallel
  ├── T04 (renderer)    ─┤
  └── T06 (watcher)     ─┘
           │
           ├── T05 (assets) — needs T04 for CSS
           │
           └── T07 (server) — needs T02, T03, T04, T05, T06
                    │
                    T08 (main wiring) — needs T02, T06, T07
                         │
                         T09 (smoke test + install)
```

**Parallel opportunities:** T02, T03, T04, T06 can all be implemented simultaneously after T01.

---

## Status Tracking

| Task | Status | Notes |
|------|--------|-------|
| T01 | Pending | |
| T02 | Pending | |
| T03 | Pending | |
| T04 | Pending | |
| T05 | Pending | |
| T06 | Pending | |
| T07 | Pending | |
| T08 | Pending | |
| T09 | Pending | |
