use clap::{Args, Parser};

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 encode a string")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "Base64 decode a string")]
    Decode(DecodeOpts),
}

#[derive(Debug, Args)]
pub struct EncodeOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(long)]
    pub url_safe: bool,
    #[arg(long)]
    pub no_padding: bool,
}

#[derive(Debug, Args)]
pub struct DecodeOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(short, long)]
    pub output: Option<String>,
    #[arg(short, long)]
    pub url_safe: bool,
    #[arg(short, long)]
    pub no_padding: bool,
}
