mod b64;
mod csv;
mod genpass;
mod text;

use std::path::Path;

use clap::{Parser, Subcommand};

pub use self::csv::{CsvOpts, OutputFormat};
pub use self::text::{TextSignFormat, TextSubCommand};
pub use b64::{Base64Format, Base64SubCommand};
pub use genpass::GenPassOpts;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about=None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a random password")]
    GenPass(GenPassOpts),
    #[command(name = "base64", about = "Base64 encode or decode", subcommand)]
    Base64SubCommand(Base64SubCommand),
    #[command(name = "text", about = "Sign or verify a message", subcommand)]
    Text(TextSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("nonexistent"), Err("File does not exist"));
    }
}
