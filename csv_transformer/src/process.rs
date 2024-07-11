use std::{fs, path::Path};

use anyhow::{anyhow, Error, Result};

use crate::OutputFormat;

fn check_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    if path.exists() && path.is_file() {
        return true;
    }
    false
}
fn get_output_format(output: &str) -> Result<OutputFormat, Error> {
    let parts: Vec<&str> = output.split('.').collect();
    if let Some(ext) = parts.last() {
        ext.parse::<OutputFormat>().map_err(|e| e.into())
    } else {
        Err(anyhow!("No file extension found"))
    }
}

pub fn process_csv_transformer(input: &str, output: &str) -> anyhow::Result<(), Error> {
    let output_format = get_output_format(output)?;
    if !check_file(input) {
        return Err(anyhow!("File doesn't exist"));
    }
    let mut rdr = csv::Reader::from_path(input)?;
    let mut res = Vec::with_capacity(128);
    let header = rdr.headers()?.clone();
    for result in rdr.records() {
        let record = result?;
        let json_record = record
            .iter()
            .zip(header.iter())
            .collect::<serde_json::Value>();
        res.push(json_record);
    }

    let content = match output_format {
        OutputFormat::Json => serde_json::to_string_pretty(&res)?,
        OutputFormat::Yaml => serde_yaml::to_string(&res)?,
    };

    fs::write(output, content)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::process_csv_transformer;

    #[test]
    fn test_process_csv_transformer() {
        let input = "data/test.csv";
        let output = "data/output.json";
        if let Err(err) = process_csv_transformer(input, &output) {
            println!("error running example: {}", err);
        }
    }
}
