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
    let content = match format {
        OutputFormat::Json => {
            let mut ret = Vec::with_capacity(128);
            for result in reader.records() {
                let record = result?;
                let iter = headers.iter().zip(record.iter());
                let json_record = iter.collect::<serde_json::Value>();
                ret.push(json_record);
            }
            serde_json::to_string_pretty(&ret)?
        }
        OutputFormat::Yaml => {
            let mut ret = Vec::with_capacity(128);
            for result in reader.records() {
                let record = result?;
                let iter = headers.iter().zip(record.iter());
                let yaml_record = serde_yaml::to_value(iter.collect::<serde_json::Value>())?;
                ret.push(yaml_record);
            }
            serde_yaml::to_string(&ret)?
        }
        OutputFormat::Toml => {
            let mut ret = Vec::with_capacity(128);
            for result in reader.records() {
                let record = result?;
                let iter = headers.iter().zip(record.iter());
                let toml_record = iter.collect::<HashMap<_, _>>();
                let toml_record: toml::Value = toml_record.into();
                ret.push(toml_record)
            }
            let mut result = HashMap::new();

            let file: Vec<_> = input.split(std::path::MAIN_SEPARATOR).collect();
            let input = file[file.len() - 1];
            let input = input.split('.').collect::<Vec<&str>>();
            let filename = input[0];
            result.insert(filename, ret);
            toml::to_string_pretty(&result)?
        }
    };
    fs::write(output, content)?;
    Ok(())
}
