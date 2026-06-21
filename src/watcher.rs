use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{collections::HashMap, path::PathBuf, time::Instant};
use tokio::sync::broadcast;

pub async fn watch(root: PathBuf, tx: broadcast::Sender<String>) {
    let (event_tx, mut event_rx) =
        tokio::sync::mpsc::channel::<notify::Result<notify::Event>>(64);

    let mut watcher: RecommendedWatcher =
        notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
            let _ = event_tx.blocking_send(res);
        })
        .expect("Failed to create file watcher");

    let mode = if root.is_dir() {
        RecursiveMode::Recursive
    } else {
        RecursiveMode::NonRecursive
    };

    if let Err(e) = watcher.watch(&root, mode) {
        eprintln!("[watcher] Failed to watch '{}': {e}", root.display());
        return;
    }

    let mut last_seen: HashMap<PathBuf, Instant> = HashMap::new();

    while let Some(result) = event_rx.recv().await {
        match result {
            Ok(event) => {
                let relevant = matches!(
                    event.kind,
                    EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
                );
                if !relevant {
                    continue;
                }

                for path in &event.paths {
                    // Skip hidden files and directories
                    if path.components().any(|c| {
                        c.as_os_str().to_str().map(|s| s.starts_with('.')).unwrap_or(false)
                    }) {
                        continue;
                    }

                    // Debounce: skip if same path fired within 150ms
                    let now = Instant::now();
                    if let Some(&last) = last_seen.get(path) {
                        if now.duration_since(last).as_millis() < 150 {
                            continue;
                        }
                    }
                    last_seen.insert(path.clone(), now);

                    // Compute relative path so the browser can decide whether
                    // to reload based on which file it's currently viewing.
                    let rel = path
                        .strip_prefix(&root)
                        .unwrap_or(path)
                        .to_string_lossy()
                        .replace('\\', "/");

                    let msg = serde_json::json!({"type": "reload", "path": rel}).to_string();
                    if tx.send(msg).is_err() {
                        return;
                    }
                }
            }
            Err(e) => eprintln!("[watcher] error: {e:?}"),
        }
    }
}
