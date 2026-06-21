# Markdown Preview — Design

## Architecture Overview

```
┌─────────────┐
│  CLI (clap) │  mdpreview [path]
└──────┬──────┘
       │
       ▼
┌──────────────────────────────────────────────────────────┐
│                      App Core (main.rs)                  │
│  1. Resolve path → Mode::File | Mode::Dir                │
│  2. Scan .md files (scanner)                             │
│  3. Create broadcast channel (watcher → WS clients)      │
│  4. Spawn watcher task                                   │
│  5. Bind port (3000..3009)                               │
│  6. Spawn axum server                                    │
│  7. Open browser                                         │
│  8. Await Ctrl+C                                         │
└──────────────────────────────────────────────────────────┘
          │                          │
          ▼                          ▼
┌──────────────────┐      ┌──────────────────────────────────┐
│  File Watcher    │      │    HTTP Server (axum 0.6)        │
│  (notify 6.x)   │      │                                  │
│                  │      │  GET /            → index page   │
│  Watches dir or  │─────►│  GET /render?p=.. → render .md  │
│  file for events │      │  GET /ws          → WebSocket    │
│  → broadcast::Tx │      │  GET /static/..   → CSS/JS       │
└──────────────────┘      └──────────────────────────────────┘
                                       │
                                       ▼
                           ┌──────────────────────┐
                           │  Renderer (renderer) │
                           │  comrak (GFM) +      │
                           │  syntect (highlight) │
                           └──────────────────────┘
```

## Component Breakdown

### `src/main.rs` — Entry & wiring

- Parse CLI args via clap
- Resolve absolute path; detect file vs directory
- Create `tokio::sync::broadcast::channel::<String>(64)` for reload events
- Spawn `watcher::watch(path, tx)` as a tokio task
- Try binding `TcpListener` on ports 3000–3009
- Build axum router with shared state (`AppState`)
- Spawn `axum::Server::from_tcp(listener).serve(router)` as tokio task
- Call `webbrowser::open(url)` 
- `tokio::signal::ctrl_c().await` to block until exit

### `src/cli.rs` — CLI arguments

```rust
#[derive(Parser)]
#[command(name = "mdpreview", about = "GitHub-style markdown preview")]
pub struct Cli {
    /// Path to a .md file or directory (default: current directory)
    pub path: Option<PathBuf>,
}
```

### `src/scanner.rs` — File discovery

- `pub fn scan(root: &Path) -> Vec<PathBuf>` 
- Uses `walkdir` to find all `*.md` files recursively
- Returns sorted list (alphabetical, directories before files)
- For single-file mode, returns `vec![file_path]`

### `src/renderer.rs` — Markdown → HTML

```rust
pub fn render(markdown: &str) -> String
```

**Pipeline:**
1. Pass raw markdown to comrak with all GFM extensions enabled:
   - `autolink`, `table`, `tasklist`, `strikethrough`, `footnotes`
   - `unsafe_` rendering enabled (to allow raw HTML pass-through)
   - Syntax highlighting: use comrak's `SyntaxHighlighterAdapter` trait with syntect

2. Wrap output in full HTML page using `assets::page_template(title, sidebar_html, content_html)`

**comrak + syntect integration:**
Implement `comrak::adapters::SyntaxHighlighterAdapter` using `syntect::easy::HighlightLines` with the `InspiredGitHub` theme (closest to GitHub's syntax colors).

### `src/server.rs` — axum routes

**Shared state:**
```rust
#[derive(Clone)]
pub struct AppState {
    pub root: Arc<PathBuf>,
    pub mode: Arc<Mode>,         // File | Dir
    pub reload_tx: broadcast::Sender<String>,
}
```

**Routes:**
| Route | Handler | Description |
|-------|---------|-------------|
| `GET /` | `index_handler` | Scan files, render first .md, return full page |
| `GET /render` | `render_handler` | `?path=relative/path.md` → render that file |
| `GET /ws` | `ws_handler` | WebSocket upgrade; subscribe to `reload_tx` |
| `GET /static/app.js` | inline | Serve embedded JS |

**WebSocket handler (axum 0.6):**
```rust
async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state.reload_tx.subscribe()))
}

async fn handle_ws(socket: WebSocket, mut rx: broadcast::Receiver<String>) {
    let (mut sender, _) = socket.split();
    while let Ok(msg) = rx.recv().await {
        if sender.send(Message::Text(msg)).await.is_err() { break; }
    }
}
```

### `src/watcher.rs` — File watching

```rust
pub async fn watch(root: PathBuf, tx: broadcast::Sender<String>)
```

- Uses `notify::recommended_watcher` with a tokio channel bridge
- Watches `RecursiveMode::Recursive` for directories, `NonRecursive` for single files
- On `EventKind::Modify | Create | Remove`: serialize event as JSON `{"type":"reload","path":"<relative>"}` and `tx.send()`
- Debounce: ignore events within 50ms of a previous event for the same file

### `src/assets.rs` — Embedded static files

All embedded at compile time via `include_str!`:

```rust
pub const GITHUB_CSS: &str = include_str!("../assets/github-markdown.css");
pub const APP_JS: &str = include_str!("../assets/app.js");
```

**`assets/github-markdown.css`:** GitHub Primer markdown CSS (from `github-markdown-css` project, MIT license). Scoped under `.markdown-body` class.

**`assets/app.js`:** ~30 lines — WebSocket client + live reload logic:
```js
const ws = new WebSocket(`ws://${location.host}/ws`);
ws.onmessage = (e) => {
  const msg = JSON.parse(e.data);
  if (msg.type === 'reload') {
    // Fetch new content and replace .markdown-body innerHTML
    fetch(`/render?path=${encodeURIComponent(msg.path)}`)
      .then(r => r.text())
      .then(html => { document.querySelector('.markdown-body').innerHTML = html; });
  }
};
ws.onclose = () => setTimeout(() => location.reload(), 1000); // reconnect
```

**`assets/page_template`:** Function that builds the full HTML shell:
- Two-column flex layout: `#sidebar` (20%) + `#content` (80%)
- Sidebar links: `<a href="/render?path=...">filename</a>` → fetched via JS
- Content: `<article class="markdown-body">...</article>`
- Loads `/static/app.js` and inline `<style>` for layout chrome

## Data Flow: Live Reload

```
User saves file.md
      │
      ▼
notify watcher fires Modify event
      │
      ▼
watcher.rs → tx.send(r#"{"type":"reload","path":"docs/file.md"}"#)
      │
      ▼ (broadcast)
All WebSocket clients receive the JSON message
      │
      ▼
app.js: if msg.path == currentPath → fetch /render?path=... → swap .markdown-body
        else → location.reload() (sidebar may need update)
```

## Key Technical Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Markdown parser | `comrak` | Implements GFM spec; supports SyntaxHighlighterAdapter for syntect integration |
| Syntax highlighting | `syntect` server-side | Zero JS needed; InspiredGitHub theme matches GitHub closely |
| HTTP framework | `axum 0.6.x` | Built-in WebSocket; ergonomic; MSRV 1.63 |
| Live reload transport | WebSocket | Persistent connection; lower latency than polling |
| CSS approach | Embedded `github-markdown-css` | Self-contained binary; exact GitHub appearance |
| JS approach | Inline partial-reload | Avoids full page flash on content-only changes |
| Rust version | **Update to latest stable via asdf** | Current 1.67.1 is old; newer crates have better MSRV support. `asdf install rust latest` before starting |

## Crate Versions

```toml
[dependencies]
axum = { version = "0.6", features = ["ws"] }
tokio = { version = "1", features = ["full"] }
comrak = { version = "0.18", features = ["syntect"] }
syntect = "5"
notify = "6"
walkdir = "2"
clap = { version = "4", features = ["derive"] }
webbrowser = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tower-http = { version = "0.3", features = ["fs"] }
```

> **Note:** `clap 4` and `axum 0.6` require Rust ≥ 1.70. Run `asdf install rust latest && asdf global rust latest` before building.

## File Structure

```
markdown-preview-locally/
├── Cargo.toml
├── assets/
│   ├── github-markdown.css   # Embedded at compile time
│   └── app.js                # Embedded at compile time
└── src/
    ├── main.rs               # Entry point + wiring
    ├── cli.rs                # clap argument struct
    ├── scanner.rs            # walkdir file discovery
    ├── renderer.rs           # comrak + syntect
    ├── server.rs             # axum routes + WebSocket
    ├── watcher.rs            # notify file watching
    └── assets.rs             # include_str! constants + HTML template
```
