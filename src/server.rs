use axum::{
    body::Body,
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::{header, Response, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::broadcast;

use crate::scanner::{TreeNode, DirNode, FileNode};

#[derive(Clone)]
pub struct AppState {
    pub root: Arc<PathBuf>,
    pub reload_tx: broadcast::Sender<String>,
}

#[derive(serde::Deserialize)]
pub struct PathQuery {
    path: String,
    #[serde(default)]
    theme: Option<String>,
}

#[derive(serde::Deserialize, Default)]
struct IndexQuery {
    path: Option<String>,
}

pub fn make_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/render", get(render_handler))
        .route("/file", get(file_handler))
        .route("/ws", get(ws_handler))
        .route("/static/app.js", get(js_handler))
        .with_state(state)
}

// ── Route handlers ───────────────────────────────────────────────────────────

async fn index_handler(
    State(state): State<AppState>,
    Query(params): Query<IndexQuery>,
) -> Html<String> {
    let tree = crate::scanner::scan_tree(&state.root);

    let (content, current_rel) = if let Some(ref path) = params.path {
        // Honour ?path= so that F5 / deep-links restore the correct file.
        match resolve_safe(&state.root, path) {
            Some((abs, rel)) => {
                let html = crate::renderer::render_for_web(&rel, &abs, crate::renderer::Theme::Light);
                (html, rel)
            }
            None => default_content(&tree, &state.root),
        }
    } else {
        default_content(&tree, &state.root)
    };

    let sidebar = build_tree_html(&tree, &current_rel, 0);
    let title = current_rel
        .rsplit('/')
        .next()
        .unwrap_or("mdpreview");

    Html(crate::assets::full_page(
        title,
        &format!("<ul class=\"tree\">{sidebar}</ul>"),
        &content,
        &current_rel,
    ))
}

async fn render_handler(
    State(state): State<AppState>,
    Query(params): Query<PathQuery>,
) -> axum::response::Response {
    let (canonical, rel) = match resolve_safe(&state.root, &params.path) {
        Some(pair) => pair,
        None => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    };

    let theme = crate::renderer::Theme::from_str(params.theme.as_deref().unwrap_or("light"));
    Html(crate::renderer::render_for_web(&rel, &canonical, theme)).into_response()
}

async fn file_handler(
    State(state): State<AppState>,
    Query(params): Query<PathQuery>,
) -> axum::response::Response {
    let (canonical, _) = match resolve_safe(&state.root, &params.path) {
        Some(pair) => pair,
        None => return (StatusCode::NOT_FOUND, "File not found").into_response(),
    };

    let ext = canonical
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let content_type = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "bmp" => "image/bmp",
        _ => "application/octet-stream",
    };

    match std::fs::read(&canonical) {
        Ok(bytes) => Response::builder()
            .header(header::CONTENT_TYPE, content_type)
            .body(Body::from(bytes))
            .unwrap()
            .into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, state.reload_tx.subscribe()))
}

async fn handle_ws(mut socket: WebSocket, mut rx: broadcast::Receiver<String>) {
    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg.into())).await.is_err() {
            break;
        }
    }
}

async fn js_handler() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "application/javascript; charset=utf-8")],
        crate::assets::APP_JS,
    )
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Resolve a relative path against root, check it stays within root.
/// Returns `(canonical_abs_path, relative_forward_slash_path)` or None.
fn resolve_safe(root: &Arc<PathBuf>, rel: &str) -> Option<(PathBuf, String)> {
    let canonical = root.join(rel).canonicalize().ok()?;
    let root_canonical = root.canonicalize().unwrap_or_else(|_| root.as_ref().clone());
    if !canonical.starts_with(&root_canonical) {
        return None; // path traversal attempt
    }
    let clean_rel = canonical
        .strip_prefix(&root_canonical)
        .unwrap_or(&canonical)
        .to_string_lossy()
        .replace('\\', "/");
    Some((canonical, clean_rel))
}

fn default_content(tree: &[TreeNode], root: &Arc<PathBuf>) -> (String, String) {
    match crate::scanner::find_default(tree) {
        Some(ref rel) => {
            let abs = root.join(rel);
            let html = crate::renderer::render_for_web(rel, &abs, crate::renderer::Theme::Light);
            (html, rel.clone())
        }
        None => (
            "<p style=\"color:#57606a\">No files found in this directory.</p>".to_string(),
            String::new(),
        ),
    }
}

fn html_escape(s: &str) -> String {
    crate::renderer::html_escape(s)
}

fn dir_contains(d: &DirNode, current: &str) -> bool {
    d.children.iter().any(|n| match n {
        TreeNode::File(f) => f.rel_path == current,
        TreeNode::Dir(sub) => dir_contains(sub, current),
    })
}

/// Build `<li>` rows for the sidebar tree recursively.
/// `depth` drives the left padding on nested `<ul>`.
fn build_tree_html(nodes: &[TreeNode], current: &str, _depth: usize) -> String {
    nodes.iter().map(|node| match node {
        TreeNode::Dir(d) => build_dir_html(d, current),
        TreeNode::File(f) => build_file_html(f, current),
    }).collect()
}

fn build_dir_html(d: &DirNode, current: &str) -> String {
    // Expand directories that contain the active file so F5 / deep-links don't
    // leave the user with a collapsed sidebar hiding the current file.
    let collapsed = if dir_contains(d, current) { "" } else { " collapsed" };
    let children_html = build_tree_html(&d.children, current, 0);
    format!(
        "<li class=\"tree-dir{collapsed}\">\
          <div class=\"tree-dir-label\" onclick=\"toggleDir(this)\">\
            <span class=\"tree-arrow\">&#9660;</span>\
            {}\
          </div>\
          <ul>{}</ul>\
        </li>",
        html_escape(&d.name),
        children_html
    )
}

fn build_file_html(f: &FileNode, current: &str) -> String {
    let active = if f.rel_path == current { " class=\"active\"" } else { "" };
    format!(
        "<li class=\"tree-file\">\
          <a href=\"#\" data-path=\"{}\" onclick=\"loadFile(this);return false;\"{} title=\"{}\">{}</a>\
        </li>",
        html_escape(&f.rel_path),
        active,
        html_escape(&f.rel_path),
        html_escape(&f.name),
    )
}
