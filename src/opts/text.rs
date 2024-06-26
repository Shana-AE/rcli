use std::{fmt, path::PathBuf, str::FromStr};

use crate::{process_text_generate, process_text_sign, process_text_verify, CmdExecutor};

use enum_dispatch::enum_dispatch;
use tokio::fs;

use super::{verify_file, verify_path};
use clap::{Args, Parser};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private/shared key")]
    Sign(TextSignOpts),
    #[command(about = "Verify a message")]
    Verify(TextVerifyOpts),
    #[command(about = "Generate a random blake3 key or Ed25519 key pair")]
    Generate(KeyGenerateOpts),
}

#[derive(Debug, Args)]
pub struct TextSignOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format )]
    pub format: TextSignFormat,
}

#[derive(Debug, Args)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(short, long)]
    pub sig: String,
    #[arg(long, default_value = "blake3", value_parser = parse_text_sign_format )]
    pub format: TextSignFormat,
}

#[derive(Debug, Args)]
pub struct KeyGenerateOpts {
    #[arg(long, value_parser = parse_text_sign_format, default_value = "blake3")]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_path)]
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

fn parse_text_sign_format(format: &str) -> anyhow::Result<TextSignFormat> {
    format.parse()
}

impl From<TextSignFormat> for &str {
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3 => "blake3",
            TextSignFormat::Ed25519 => "ed25519",
        }
    }
}

impl FromStr for TextSignFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "ed25519" => Ok(TextSignFormat::Ed25519),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

impl CmdExecutor for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_text_sign(&self.input, &self.key, self.format)?;
        Ok(())
    }
}

impl CmdExecutor for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_text_verify(&self.input, &self.key, &self.sig, self.format)?;
        Ok(())
    }
}

impl CmdExecutor for KeyGenerateOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let keys = process_text_generate(self.format)?;
        for (k, v) in keys {
            fs::write(self.output_path.join(k), v).await?
        }
        Ok(())
    }
}
