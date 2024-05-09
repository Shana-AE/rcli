use clap::{Args, Parser};
use std::{fmt, str::FromStr};

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
    #[arg(long, value_parser = parse_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Args)]
pub struct DecodeOpts {
    #[arg(short, long)]
    pub input: String,
    #[arg(long, value_parser = parse_format, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

fn parse_format(format: &str) -> anyhow::Result<Base64Format> {
    format.parse()
}

impl From<Base64Format> for &str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url_safe_strs = ["urlsafe", "url_safe", "url safe"];
        match s.to_lowercase().as_str() {
            "standard" => Ok(Base64Format::Standard),
            s if url_safe_strs.contains(&s) => Ok(Base64Format::UrlSafe),
            v => anyhow::bail!("Unsupported format: {}", v),
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}