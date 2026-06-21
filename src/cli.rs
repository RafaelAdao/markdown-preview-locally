use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mdpreview",
    about = "GitHub-style local markdown preview with live reload"
)]
pub struct Cli {
    /// Path to a .md file or directory (defaults to current directory)
    pub path: Option<PathBuf>,
}
