# Roadmap

## v1 — Local GitHub Preview (Current)

**Goal:** Working CLI tool that previews any `.md` file or directory in the browser with GitHub-like rendering and live reload.

**Features:**
- [x] Project initialized
- [x] MDPV: Markdown Preview feature
  - [x] CLI entry point (`clap`)
  - [x] Directory scanner (`walkdir`)
  - [x] GFM markdown renderer (`comrak` + `syntect`)
  - [x] HTTP server with sidebar navigation (`axum 0.8`)
  - [x] File watcher (`notify`)
  - [x] WebSocket live reload
  - [x] Browser auto-open (`webbrowser`)
- [x] Mermaid diagram rendering (client-side via Mermaid.js CDN)
- [x] Relative link navigation — inline links open files inside the previewer
- [x] Sidebar auto-expands to reveal the active file on navigation
- [x] CHANGELOG

**Definition of done:** `cargo install --path .` then `mdpreview ~/my-docs/` opens browser at `http://localhost:3000`, shows all `.md` files in sidebar, renders selected file with GitHub styling, auto-reloads on save, and renders Mermaid diagrams.

---

## v2 — Ideas (not committed)

- **URL-based navigation** — reflect the active file in the browser URL (e.g. `?path=docs/api.md`) so that F5 / hard-refresh restores the file you were viewing instead of resetting to the default; requires `history.pushState` on navigate and reading the query param on page load
- Search across files
- Front matter display (YAML/TOML)
- Custom port flag (`--port`)
- Image serving from relative paths
- Anchor link scrolling (`file.md#section`)
