// rcli csv -i input.csv -o output.json --header -d ','
use clap::{Args, Parser, Subcommand};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Args)]
struct CsvOpts {
    #[arg(short, long, value_parser = validate_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(long, default_value_t = false)]
    header: bool,
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input).unwrap();
            for result in reader.deserialize() {
                let record: Player = result.unwrap();
                println!("{:?}", record);
            }
        }
    }
}

fn validate_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}
