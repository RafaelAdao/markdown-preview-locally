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
- [x] Language label on code blocks
- [x] URL-based navigation — active file in URL, F5 restores file, back/forward works
- [x] CHANGELOG

**Definition of done:** `cargo install --path .` then `mdpreview ~/my-docs/` opens browser at `http://localhost:3000`, shows all `.md` files in sidebar, renders selected file with GitHub styling, auto-reloads on save, and renders Mermaid diagrams.

---

## v2 — Ideas (not committed)

- Anchor link scrolling (`file.md#section`)
- Search across files
- Front matter display (YAML/TOML)
- Custom port flag (`--port`)
- Image serving from relative paths
