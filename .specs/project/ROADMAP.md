# Roadmap

## v1 — Local GitHub Preview (Current)

**Goal:** Working CLI tool that previews any `.md` file or directory in the browser with GitHub-like rendering and live reload.

**Features:**
- [x] Project initialized
- [x] MDPV: Markdown Preview feature (single deliverable — see spec)
  - [x] CLI entry point (`clap`)
  - [x] Directory scanner (`walkdir`)
  - [x] GFM markdown renderer (`comrak` + `syntect`)
  - [x] HTTP server with sidebar navigation (`axum 0.8`)
  - [x] File watcher (`notify`)
  - [x] WebSocket live reload
  - [x] Browser auto-open (`webbrowser`)

**Definition of done:** `cargo install --path .` then `mdpreview ~/my-docs/` opens browser at `http://localhost:3000`, shows all `.md` files in sidebar, renders selected file with GitHub styling, and auto-reloads on save.

---

## v2 — Ideas (not committed)

- Search across files
- Front matter display (YAML/TOML)
- Custom port flag (`--port`)
- Mermaid diagram rendering
- Image serving from relative paths
