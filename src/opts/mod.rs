mod b64;
mod csv;
mod genpass;

use std::path::Path;

use clap::{Parser, Subcommand};

pub use self::csv::{CsvOpts, OutputFormat};
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
}

fn validate_input_file(filename: &str) -> Result<String, &'static str> {
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
    fn test_validate_input_file() {
        assert_eq!(validate_input_file("-"), Ok("-".into()));
        assert_eq!(validate_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            validate_input_file("nonexistent"),
            Err("File does not exist")
        );
    }
}
