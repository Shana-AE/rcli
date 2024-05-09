mod b64;
mod csv;
mod genpass;

use clap::{Parser, Subcommand};

pub use self::csv::{CsvOpts, OutputFormat};
pub use b64::Base64SubCommand;
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
