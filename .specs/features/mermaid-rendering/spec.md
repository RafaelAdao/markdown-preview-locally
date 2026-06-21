# Mermaid Diagram Rendering — Spec

**Scope:** Medium  
**Status:** Implementing

## Requirements

| ID | Requirement |
|----|-------------|
| MERM-01 | Fenced code blocks with info string `mermaid` (` ```mermaid `) are rendered as Mermaid diagrams, not as code |
| MERM-02 | Diagrams update correctly on live reload (same mechanism as markdown content) |
| MERM-03 | Diagrams respect the active light/dark theme — use Mermaid's `default` theme for light, `dark` for dark |
| MERM-04 | Non-mermaid code blocks are unaffected |
| MERM-05 | If Mermaid.js fails to load (offline), the raw diagram source is visible as fallback text |

## Approach

- **Server-side pre-processing:** Before comrak parses the markdown, scan for ` ```mermaid ` fences and replace them with `<div class="mermaid">HTML-escaped-source</div>`. comrak passes raw HTML through (unsafe_ rendering is already enabled).
- **Client-side rendering:** Load Mermaid.js from CDN (`mermaid@11`). Call `mermaid.run()` after content loads or updates.
- **CDN vs bundle:** CDN chosen — Mermaid.js is ~2.5 MB minified; bundling would significantly increase binary size. Acceptable for a local dev tool (usually has internet).

## Files changed

- `src/renderer.rs` — add `preprocess_mermaid()`, call it in `render_to_html()`
- `src/assets.rs` — add Mermaid.js CDN `<script>` + CSS for `.mermaid` containers
- `assets/app.js` — add `renderMermaid()`, call from `setContent()` and init
- `CHANGELOG.md` — add to [Unreleased]
