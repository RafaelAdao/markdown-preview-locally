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

### Navigation & UX
- Anchor link scrolling (`file.md#section`) — scroll to the heading when a `#fragment` is in the URL or link
- Keyboard navigation — `j`/`k` to move between files, `Ctrl+K` quick-file-switcher
- Table of contents — auto-generated from headings, shown as a floating panel or injected above content
- Print / export to PDF — `window.print()` with a dedicated print stylesheet; zero backend work

### Editor experience
- Line numbers on code blocks — toggle-able gutter next to syntax-highlighted code
- Broken link highlight — dim or mark relative links that point to missing files
- Word count / reading time — subtle status line displayed below the file title

### Rendering
- GitHub Alerts — render `> [!NOTE]`, `> [!WARNING]`, `> [!TIP]` etc. with coloured callout boxes
- Search across files — full-text search with results linking directly to the matching file

### Server & CLI
- Sidebar auto-refresh — detect newly added or deleted `.md` files and update the sidebar without restarting (file watcher already runs)
- Custom port flag (`--port`) — let the user pin a specific port instead of auto-selecting
