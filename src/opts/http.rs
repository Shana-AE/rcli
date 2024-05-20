use std::path::PathBuf;

use crate::{process_http_serve, CmdExecutor};

use super::verify_path;

use clap::{Args, Subcommand};
use enum_dispatch::enum_dispatch;

#[derive(Debug, Subcommand)]
#[enum_dispatch(CmdExecutor)]
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

impl CmdExecutor for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_http_serve(self.dir, self.port).await?;
        Ok(())
    }
}
