# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Mermaid diagram rendering — fenced code blocks tagged `mermaid` are rendered as interactive diagrams via Mermaid.js; diagrams update on live reload and respect the active light/dark theme

### Fixed

- Relative links in rendered markdown (e.g. `[doc](docs/file.md)`) now open the target file inside the previewer instead of causing a full browser navigation
- Navigating to a file inside a directory via an inline link now expands that directory in the sidebar

## [0.1.0] - 2026-06-21

### Added

- GitHub-flavored markdown rendering via `comrak` (tables, task lists, strikethrough, footnotes, autolinks)
- Server-side syntax highlighting for fenced code blocks using `syntect` with the InspiredGitHub theme
- WebSocket-based live reload — edits appear in the browser within ~1 second of saving
- File tree sidebar listing all `.md` files and directories; click any entry to navigate
- Non-markdown file support — source files are syntax-highlighted, images are displayed, binaries show size info
- Copy button on hover for all code blocks
- Light / dark theme toggle with preference saved across sessions
- Auto-open browser on server startup
- Automatic port selection — starts on 3000, falls back to 3001–3009 if the port is busy
- Single-file mode — `mdpreview README.md` previews a single file directly
- Zero-config operation — `mdpreview` with no arguments previews the current directory

[Unreleased]: https://github.com/RafaelAdao/markdown-preview-locally/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/RafaelAdao/markdown-preview-locally/releases/tag/v0.1.0
