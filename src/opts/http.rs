use std::path::PathBuf;

use super::verify_path;

use clap::{Args, Subcommand};

#[derive(Debug, Subcommand)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Args)]
pub struct HttpServeOpts {
    #[arg(value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
}