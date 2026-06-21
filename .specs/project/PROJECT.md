# markdown-preview-localy

**Vision:** A fast, zero-config Rust CLI that serves any directory of markdown files as a local GitHub-style documentation site with live reload.
**For:** Developers who write markdown and want to preview it exactly as it appears on GitHub — without pushing to a remote.
**Solves:** The friction of checking GitHub rendering for README files, wikis, or docs during local editing.

## Goals

- Render markdown identically to GitHub (GFM: tables, task lists, strikethrough, code blocks with syntax highlighting)
- Zero-config: `mdpreview .` should work out of the box — no setup, no config files
- Live reload: changes to any `.md` file in the watched directory reflect in the browser instantly

## Tech Stack

**Core:**

- Language: Rust (via asdf, currently 1.67.1 stable)
- HTTP server: axum 0.6.x (MSRV-compatible)
- Async runtime: tokio 1.x
- Markdown parser: comrak (GFM-compliant, closest to GitHub's cmark-gfm)

**Key dependencies:**

- `comrak` — GFM markdown parser with extensions (tables, task lists, strikethrough, autolinks)
- `axum 0.6` — HTTP server + built-in WebSocket support for live reload
- `tokio` — async runtime
- `notify` — cross-platform file system watching
- `syntect` — server-side syntax highlighting for code blocks
- `walkdir` — recursive directory traversal for `.md` file discovery
- `clap` — CLI argument parsing
- `webbrowser` — open default browser on startup

## Scope

**v1 includes:**

- CLI: `mdpreview [path]` where path defaults to current directory
- Directory navigation sidebar listing all `.md` files (recursive)
- GitHub-flavored markdown rendering (comrak with all GFM extensions)
- Server-side syntax highlighting via syntect (GitHub theme)
- WebSocket-based live reload when any `.md` file changes
- Auto-open browser when server starts
- Embedded GitHub Primer CSS (no external dependencies)
- Single `.md` file as input also supported

**Explicitly out of scope:**

- Plugins or custom themes
- Editing markdown in the browser
- Authentication or access control
- Deployment / publishing output
- Non-markdown file rendering (images served as static assets only)
- Search across files

## Constraints

- Technical: Must work with Rust 1.67.1 (asdf); axum 0.6.x, not 0.7.x
- Resources: Single binary, no runtime dependencies, no npm
