mod assets;
mod cli;
mod renderer;
mod scanner;
mod server;
mod watcher;

use clap::Parser;
use std::{net::TcpListener, sync::Arc};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let args = cli::Cli::parse();

    let raw_path = args
        .path
        .unwrap_or_else(|| std::env::current_dir().expect("Cannot determine current directory"));

    if !raw_path.exists() {
        eprintln!("Error: path '{}' does not exist", raw_path.display());
        std::process::exit(1);
    }

    let path = raw_path
        .canonicalize()
        .expect("Failed to resolve path");

    let is_single_file = path.is_file();
    if is_single_file {
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if !ext.eq_ignore_ascii_case("md") {
            eprintln!("Error: '{}' is not a .md file", path.display());
            std::process::exit(1);
        }
    }

    let root = if is_single_file {
        path.parent()
            .unwrap_or(path.as_path())
            .to_path_buf()
    } else {
        path.clone()
    };

    let (reload_tx, _) = broadcast::channel::<String>(64);

    let (listener, port) = find_port(3000, 3009);
    let url = format!("http://localhost:{port}");

    eprintln!("  mdpreview  {url}");
    eprintln!("  watching   {}", root.display());
    eprintln!("  Ctrl+C to stop\n");

    let state = server::AppState {
        root: Arc::new(root.clone()),
        reload_tx: reload_tx.clone(),
    };

    let router = server::make_router(state);
    listener.set_nonblocking(true).expect("Failed to set non-blocking");
    let tokio_listener = tokio::net::TcpListener::from_std(listener)
        .expect("Failed to convert listener");

    tokio::spawn(watcher::watch(root, reload_tx));

    if let Err(e) = webbrowser::open(&url) {
        eprintln!("[warn] Could not open browser: {e}");
    }

    tokio::select! {
        result = axum::serve(tokio_listener, router) => {
            if let Err(e) = result {
                eprintln!("Server error: {e}");
            }
        }
        _ = tokio::signal::ctrl_c() => {}
    }

    eprintln!("Stopped.");
}

fn find_port(start: u16, end: u16) -> (TcpListener, u16) {
    for port in start..=end {
        if let Ok(listener) = TcpListener::bind(format!("127.0.0.1:{port}")) {
            return (listener, port);
        }
    }
    eprintln!("Error: no available port in range {start}-{end}");
    std::process::exit(1);
}
