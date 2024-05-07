use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::opts::OutputFormat;

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

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let headers = reader.headers()?.clone();
    let mut ret = Vec::with_capacity(128);
    for result in reader.records() {
        let record = result?;
        let record = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(record);
    }
    let content = match format {
        OutputFormat::Toml => {
            let file: Vec<_> = input.split(std::path::MAIN_SEPARATOR).collect();
            let input = file[file.len() - 1];
            let filename: Vec<_> = input.split('.').collect();
            let filename = filename[0];
            let mut result = HashMap::new();
            result.insert(filename, ret);
            toml::to_string(&result)?
        }
        // because serde_yaml::to_string() only requires Serialize and Sized trait, so we can use serde_json::Value here
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        // because serde_yaml::to_string() only requires Serialize and Sized trait, so we can use serde_json::Value here
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
    };
    println!("content: {}", content);
    fs::write(output, content)?;
    Ok(())
}
