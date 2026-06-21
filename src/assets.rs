pub const GITHUB_CSS_LIGHT: &str = include_str!("../assets/github-markdown.css");
pub const GITHUB_CSS_DARK: &str = include_str!("../assets/github-markdown-dark.css");
pub const APP_JS: &str = include_str!("../assets/app.js");


pub fn full_page(title: &str, sidebar_html: &str, content_html: &str, current_path: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en" data-theme="light">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="current-path" content="{current_path}">
<title>{title} — mdpreview</title>
<!-- Apply saved theme BEFORE stylesheets render to avoid flash -->
<script>!function(){{var t=localStorage.getItem("mdpreview-theme")||(matchMedia("(prefers-color-scheme:dark)").matches?"dark":"light");document.documentElement.dataset.theme=t}}();</script>
<style id="css-light" media="all">{GITHUB_CSS_LIGHT}</style>
<style id="css-dark"  media="not all">{GITHUB_CSS_DARK}</style>
<!-- Activate the right stylesheet immediately (before first paint) -->
<script>!function(){{var d=document.documentElement.dataset.theme==="dark";document.getElementById("css-light").media=d?"not all":"all";document.getElementById("css-dark").media=d?"all":"not all";}}();</script>
<style>
*,*::before,*::after{{box-sizing:border-box}}
html,body{{height:100%;margin:0}}

/* ── CSS custom properties (theme tokens) ─────────────────────────────────── */
:root{{
  --bg:#ffffff;
  --bg-sidebar:#f6f8fa;
  --bg-sidebar-hover:#eaeef2;
  --bg-active:#dce0e7;
  --bg-code:#ffffff;
  --bg-code-header:#f6f8fa;
  --color-text:#24292f;
  --color-text-muted:#57606a;
  --color-link:#0969da;
  --color-border:#d0d7de;
  --color-active:#0969da;
  --color-danger:#cf222e;
  --color-warn-bg:#9a6700;
  --color-icon:#57606a;
}}
[data-theme="dark"]{{
  --bg:#0d1117;
  --bg-sidebar:#161b22;
  --bg-sidebar-hover:#21262d;
  --bg-active:#1f2d3d;
  --bg-code:#0d1117;
  --bg-code-header:#161b22;
  --color-text:#e6edf3;
  --color-text-muted:#8b949e;
  --color-link:#58a6ff;
  --color-border:#30363d;
  --color-active:#58a6ff;
  --color-danger:#ff7b72;
  --color-warn-bg:#9e6a03;
  --color-icon:#8b949e;
}}

/* ── Layout ─────────────────────────────────────────────────────────────────── */
body{{
  display:flex;background:var(--bg);color:var(--color-text);overflow:hidden;
  font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif;
  transition:background .15s,color .15s;
}}

/* ── Sidebar ─────────────────────────────────────────────────────────────────── */
#sidebar{{
  width:260px;min-width:180px;flex-shrink:0;
  border-right:1px solid var(--color-border);overflow-y:auto;
  background:var(--bg-sidebar);display:flex;flex-direction:column;
  transition:background .15s,border-color .15s;
}}
#sidebar-header{{
  display:flex;align-items:center;justify-content:space-between;
  flex-shrink:0;padding:10px 12px 8px 16px;
  border-bottom:1px solid var(--color-border);
}}
#sidebar-header span{{
  font-size:11px;font-weight:600;color:var(--color-text-muted);
  text-transform:uppercase;letter-spacing:.06em;
}}
#theme-btn{{
  background:none;border:none;padding:4px 6px;border-radius:6px;
  cursor:pointer;color:var(--color-icon);line-height:0;
  transition:background .1s,color .1s;
}}
#theme-btn:hover{{background:var(--bg-sidebar-hover);color:var(--color-text)}}
#theme-btn svg{{display:block;width:15px;height:15px;fill:currentColor}}
/* Show the right icon per theme */
[data-theme="light"] #icon-moon{{display:block}}
[data-theme="light"] #icon-sun{{display:none}}
[data-theme="dark"]  #icon-sun{{display:block}}
[data-theme="dark"]  #icon-moon{{display:none}}

/* ── File tree ───────────────────────────────────────────────────────────────── */
.tree{{list-style:none;margin:0;padding:4px 0 16px}}
.tree ul{{
  list-style:none;margin:0;padding-left:12px;
  border-left:1px solid var(--color-border);margin-left:18px;
}}
.tree-dir>.tree-dir-label{{
  display:flex;align-items:center;gap:5px;
  padding:5px 8px 5px 12px;cursor:pointer;user-select:none;
  color:var(--color-text-muted);font-size:12px;font-weight:600;
  white-space:nowrap;transition:background .1s;
}}
.tree-dir>.tree-dir-label:hover{{background:var(--bg-sidebar-hover);color:var(--color-text)}}
.tree-arrow{{
  display:inline-block;width:12px;text-align:center;
  font-size:9px;transition:transform .15s;flex-shrink:0;
  color:var(--color-text-muted);
}}
.tree-dir.collapsed>.tree-dir-label .tree-arrow{{transform:rotate(-90deg)}}
.tree-dir.collapsed>ul{{display:none}}
.tree-file>a{{
  display:block;padding:5px 8px 5px 14px;
  font-size:13px;color:var(--color-text);text-decoration:none;
  white-space:nowrap;overflow:hidden;text-overflow:ellipsis;line-height:1.4;
  transition:background .1s,color .1s;
}}
.tree-file>a:hover{{background:var(--bg-sidebar-hover);color:var(--color-link)}}
.tree-file>a.active{{background:var(--bg-active);font-weight:600;color:var(--color-active)}}

/* ── Content area ────────────────────────────────────────────────────────────── */
#content{{flex:1;overflow-y:auto;padding:32px 48px;background:var(--bg);transition:background .15s}}
.markdown-body{{max-width:860px;margin:0 auto}}

/* ── Code file rendering ─────────────────────────────────────────────────────── */
.code-file-header{{
  max-width:860px;margin:0 auto;padding:8px 16px;
  font-size:12px;font-weight:600;color:var(--color-text-muted);
  background:var(--bg-code-header);border:1px solid var(--color-border);
  border-bottom:none;border-radius:6px 6px 0 0;
  font-family:ui-monospace,SFMono-Regular,"SF Mono",Menlo,Consolas,monospace;
  transition:background .15s,border-color .15s;
}}
.code-block{{
  max-width:860px;margin:0 auto;padding:16px;overflow-x:auto;
  background:var(--bg-code);border:1px solid var(--color-border);
  border-radius:0 0 6px 6px;
  font-family:ui-monospace,SFMono-Regular,"SF Mono",Menlo,Consolas,monospace;
  font-size:13px;line-height:1.5;transition:background .15s,border-color .15s;
}}
.code-block code{{font-family:inherit;font-size:inherit;background:none;padding:0;color:var(--color-text)}}
.binary-info{{
  max-width:860px;margin:0 auto;padding:32px;text-align:center;
  color:var(--color-text-muted);border:1px dashed var(--color-border);border-radius:6px;
}}
.binary-info a{{color:var(--color-link)}}

/* ── Copy button ─────────────────────────────────────────────────────────────── */
.copy-btn{{
  position:absolute;top:8px;right:8px;
  padding:3px 10px;font-size:11px;line-height:1.4;
  font-family:-apple-system,BlinkMacSystemFont,"Segoe UI",Helvetica,Arial,sans-serif;
  background:var(--bg);border:1px solid var(--color-border);border-radius:4px;
  cursor:pointer;color:var(--color-text-muted);
  opacity:0;transition:opacity .15s,color .1s,background .1s,border-color .1s;
  user-select:none;z-index:1;
}}
pre:hover .copy-btn,.copy-btn:focus{{opacity:1}}
.copy-btn:hover{{background:var(--bg-sidebar-hover);color:var(--color-text)}}
.copy-btn.copied{{color:#1a7f37;border-color:#1a7f37;opacity:1}}
[data-theme="dark"] .copy-btn.copied{{color:#3fb950;border-color:#3fb950}}

/* ── Reconnect banner ────────────────────────────────────────────────────────── */
#reconnect-banner{{
  display:none;position:fixed;top:0;left:0;right:0;
  background:var(--color-warn-bg);color:#fff;text-align:center;
  padding:8px 12px;font-size:13px;z-index:1000;
}}
</style>
</head>
<body>
<div id="reconnect-banner">Reconnecting to live reload server…</div>
<nav id="sidebar">
  <div id="sidebar-header">
    <span>Explorer</span>
    <button id="theme-btn" onclick="toggleTheme()" title="Toggle light / dark theme" aria-label="Toggle theme">
      <!-- Moon: shown in light mode (click → switch to dark) -->
      <svg id="icon-moon" viewBox="0 0 16 16"><path d="M9.598 1.591a.749.749 0 0 1 .785-.175 7 7 0 1 1-8.967 8.967.75.75 0 0 1 .961-.96 5.5 5.5 0 0 0 7.046-7.046.749.749 0 0 1 .175-.786zm1.616 1.945a7 7 0 0 1-7.678 7.678 5.5 5.5 0 1 0 7.678-7.678z"/></svg>
      <!-- Sun: shown in dark mode (click → switch to light) -->
      <svg id="icon-sun"  viewBox="0 0 16 16"><path d="M8 12a4 4 0 1 1 0-8 4 4 0 0 1 0 8zm0 1a5 5 0 1 0 0-10A5 5 0 0 0 8 13zm-.75-9.25a.75.75 0 0 1 1.5 0v1.5a.75.75 0 0 1-1.5 0v-1.5zm0 9a.75.75 0 0 1 1.5 0v1.5a.75.75 0 0 1-1.5 0v-1.5zM2.166 3.227a.75.75 0 0 1 1.06 0l1.061 1.06a.75.75 0 0 1-1.06 1.061L2.165 4.288a.75.75 0 0 1 0-1.061zm8.508 8.507a.75.75 0 0 1 1.06 0l1.061 1.061a.75.75 0 0 1-1.06 1.06l-1.061-1.06a.75.75 0 0 1 0-1.061zM.75 7.25a.75.75 0 0 1 0 1.5H-.75a.75.75 0 0 1 0-1.5H.75zm14 0a.75.75 0 0 1 0 1.5h-1.5a.75.75 0 0 1 0-1.5h1.5zM3.227 13.834a.75.75 0 0 1 0-1.06l1.06-1.061a.75.75 0 0 1 1.061 1.06l-1.06 1.061a.75.75 0 0 1-1.061 0zm8.507-8.507a.75.75 0 0 1 0-1.061l1.06-1.06a.75.75 0 1 1 1.061 1.06l-1.06 1.061a.75.75 0 0 1-1.061 0z"/></svg>
    </button>
  </div>
  {sidebar_html}
</nav>
<main id="content">
  <article class="markdown-body">{content_html}</article>
</main>
<script>{APP_JS}</script>
</body>
</html>"#
    )
}
