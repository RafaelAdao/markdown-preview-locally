# Markdown Preview — Feature Specification

## Problem Statement

Developers writing markdown locally have no reliable way to preview how it will look on GitHub without pushing to a remote repository. The existing ecosystem workarounds (VS Code extensions, online tools, browser extensions) either don't match GitHub's rendering faithfully or require internet access. We need a zero-config, self-contained CLI that serves a GitHub-identical preview locally.

## Goals

- [ ] Render any `.md` file with GitHub Flavored Markdown fidelity (GFM extensions + syntax highlighting)
- [ ] Navigate a directory of markdown files from the browser with zero config
- [ ] Live reload in under 500ms from file save to browser update

## Out of Scope

| Feature | Reason |
|---------|--------|
| Editing markdown in browser | Out of scope; use your editor |
| Search across files | v2 candidate |
| Custom themes | v2 candidate |
| Non-markdown file rendering | Unnecessary complexity for v1 |
| Authentication | Local dev tool only |

---

## User Stories

### P1: Serve directory as local docs site ⭐ MVP

**User Story**: As a developer, I want to run `mdpreview .` in my docs directory so that I can browse all my markdown files in a GitHub-like interface without leaving my machine.

**Why P1**: This is the core value proposition — everything else supports it.

**Acceptance Criteria**:

1. WHEN user runs `mdpreview <dir>` THEN system SHALL start an HTTP server on port 3000 and print the URL
2. WHEN server starts THEN system SHALL auto-open `http://localhost:3000` in the default browser
3. WHEN browser opens THEN system SHALL display a two-column layout: left sidebar with all `.md` files found recursively, right panel with the rendered content of the root `README.md` (or first `.md` file found)
4. WHEN user clicks a file in the sidebar THEN system SHALL navigate to that file's rendered HTML without full page reload (or with fast reload)
5. WHEN no path argument is given THEN system SHALL default to the current working directory
6. WHEN path is a single `.md` file THEN system SHALL serve just that file (no sidebar needed)

**Independent Test**: Run `mdpreview /tmp/test-docs` where the directory contains 3 `.md` files; browser opens showing sidebar with 3 entries and renders the first file.

---

### P1: GitHub Flavored Markdown rendering ⭐ MVP

**User Story**: As a developer, I want the rendered markdown to look and work exactly like GitHub so that what I see locally matches what reviewers will see.

**Why P1**: Without GFM fidelity, the tool is not useful — plain markdown renderers already exist.

**Acceptance Criteria**:

1. WHEN a `.md` file contains a GFM table THEN system SHALL render it as an HTML table with GitHub-style borders and striped rows
2. WHEN a `.md` file contains `- [ ]` or `- [x]` THEN system SHALL render them as disabled HTML checkboxes (checked or unchecked)
3. WHEN a `.md` file contains ~~strikethrough~~ THEN system SHALL render it with `<del>` tags
4. WHEN a `.md` file contains a fenced code block with a language tag THEN system SHALL apply syntax highlighting matching GitHub's color scheme
5. WHEN a `.md` file contains a bare URL THEN system SHALL autolink it
6. WHEN the page loads THEN system SHALL use embedded GitHub Primer CSS so the typography, spacing, and colors match GitHub's markdown rendering

**Independent Test**: Render a test file containing a table, a task list, a code block with `rust` language tag, and strikethrough — all render correctly styled.

---

### P1: Live reload on file change ⭐ MVP

**User Story**: As a developer editing a markdown file, I want the browser preview to automatically refresh when I save so that I get instant visual feedback.

**Why P1**: Without live reload, the user must manually refresh; the tool's main advantage over pushing to GitHub is lost.

**Acceptance Criteria**:

1. WHEN server starts THEN system SHALL watch the target directory (or file) for filesystem changes
2. WHEN any `.md` file in the watched directory is saved/modified THEN system SHALL notify all connected browser clients within 500ms
3. WHEN browser receives reload signal THEN system SHALL refresh only the content panel (not full page reload) if the currently viewed file changed, OR do a full refresh if the sidebar needs updating (new/deleted file)
4. WHEN WebSocket connection is lost (e.g., server restarted) THEN browser SHALL display a "Reconnecting..." indicator and automatically reconnect
5. WHEN a new `.md` file is created in the directory THEN system SHALL add it to the sidebar on next reload

**Independent Test**: Open a `.md` file in editor, make a change, save — browser refreshes and shows the change within 1 second.

---

## Edge Cases

- WHEN the specified path does not exist THEN system SHALL exit with a clear error message and non-zero exit code
- WHEN port 3000 is already in use THEN system SHALL try 3001, 3002... up to 3009, then exit with error
- WHEN a `.md` file contains invalid UTF-8 THEN system SHALL display an error page for that file, not crash
- WHEN the directory contains 0 `.md` files THEN system SHALL display a "No markdown files found" message instead of blank sidebar
- WHEN user presses Ctrl+C THEN system SHALL exit cleanly (no zombie processes)
- WHEN a `.md` file is deleted while being viewed THEN system SHALL show a "File not found" message on next reload

---

## Requirement Traceability

| Requirement ID | Story | Phase | Status |
|----------------|-------|-------|--------|
| MDPV-01 | P1: Serve directory — start server + open browser | Design | Pending |
| MDPV-02 | P1: Serve directory — two-column layout with sidebar | Design | Pending |
| MDPV-03 | P1: Serve directory — recursive `.md` file discovery | Design | Pending |
| MDPV-04 | P1: Serve directory — single file mode | Design | Pending |
| MDPV-05 | P1: GFM — tables, task lists, strikethrough, autolinks | Design | Pending |
| MDPV-06 | P1: GFM — syntax highlighting (GitHub theme) | Design | Pending |
| MDPV-07 | P1: GFM — embedded Primer CSS | Design | Pending |
| MDPV-08 | P1: Live reload — file watcher | Design | Pending |
| MDPV-09 | P1: Live reload — WebSocket notification | Design | Pending |
| MDPV-10 | P1: Live reload — reconnect on disconnect | Design | Pending |
| MDPV-11 | Edge: port conflict fallback | Design | Pending |
| MDPV-12 | Edge: invalid path error handling | Design | Pending |

**Coverage:** 12 requirements, 0 mapped to tasks, 12 unmapped ⚠️

---

## Success Criteria

- [ ] `cargo install --path .` completes and `mdpreview --help` prints usage
- [ ] `mdpreview ~/my-repo` opens browser, shows all `.md` files in sidebar, renders README.md
- [ ] Editing and saving a `.md` file updates the browser in under 1 second
- [ ] A file with GFM table, task list, fenced code block, and strikethrough all render correctly
- [ ] Binary has no runtime dependencies (no Node, no Python, no CDN calls)
