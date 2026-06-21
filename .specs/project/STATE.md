# Project State

## Decisions

| Date | Decision | Reason |
|------|----------|--------|
| 2026-06-21 | Use `axum 0.6.x` not 0.7.x | MSRV: current Rust is 1.67.1; axum 0.7 requires 1.75+ |
| 2026-06-21 | Use `comrak` over `pulldown-cmark` | comrak has native GFM extensions (tables, task lists, autolinks, strikethrough) matching GitHub's cmark-gfm |
| 2026-06-21 | Server-side syntax highlighting (syntect) | No client-side JS required; keeps the binary self-contained |
| 2026-06-21 | Embed GitHub Primer CSS as string literal | Zero external deps at runtime; no npm or CDN |
| 2026-06-21 | Directory navigation scope (v1) | User confirmed: wants all .md files in a directory with sidebar, not just single file |

## Blockers

_None_

## Lessons Learned

_None yet_

## Deferred Ideas

- Mermaid diagram rendering (would need a JS runtime or WASM)
- Search across files (full-text index with tantivy?)
- Custom port via `--port` flag (trivial addition in v2)

## Preferences

_None recorded_
