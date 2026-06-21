# mdpreview

A local markdown previewer that renders files exactly like GitHub — served in your browser with live reload.

## Features

- **GitHub-flavored markdown** — tables, task lists, strikethrough, footnotes, autolinks
- **Syntax highlighting** — code fences highlighted server-side via syntect
- **Live reload** — edits in your editor appear instantly in the browser via WebSocket
- **File tree sidebar** — browse all files and directories; click any file to view it
- **Non-markdown files** — source code is syntax-highlighted, images are displayed, binaries show size info
- **Copy button** — hover over any code block to copy its contents to clipboard
- **Light / dark theme** — toggle with the moon/sun button; preference is saved across sessions
- **Auto-open** — browser opens automatically on startup

## Install

Requires [Rust](https://www.rust-lang.org/tools/install) (1.70+).

```sh
git clone <this-repo>
cd markdown-preview-localy
cargo install --path .
```

## Usage

```sh
# Preview the current directory
mdpreview

# Preview a specific directory
mdpreview path/to/project

# Preview a single file
mdpreview README.md
```

The server starts on port 3000 (falls back to 3001–3009 if busy) and opens your default browser automatically.

## Keyboard / UI

| Action | How |
|---|---|
| Browse files | Click any entry in the left sidebar |
| Toggle theme | Moon / sun button at the top of the sidebar |
| Copy code | Hover over a code block → click **Copy** |
| Collapse directory | Click the directory label |
| Stop server | `Ctrl+C` |

## Stack

| Crate | Role |
|---|---|
| `axum 0.8` | HTTP server + WebSocket |
| `comrak 0.39` | GitHub Flavored Markdown parser |
| `syntect 5` | Server-side syntax highlighting |
| `notify 7` | File system watcher |
| `clap 4` | CLI argument parsing |
| `tokio 1` | Async runtime |

## Development

```sh
cargo run              # dev build, auto-reloads on file changes
cargo test             # run unit tests
cargo build --release  # optimised binary → target/release/mdpreview
```
