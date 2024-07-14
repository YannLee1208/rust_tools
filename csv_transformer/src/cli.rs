use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        match s.as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow!("Choose between json and yaml")),
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OutputFormat::Json => f.write_str("json"),
            OutputFormat::Yaml => f.write_str("yaml"),
        }
    }
}

pub fn parse_output_format(format: &str) -> anyhow::Result<OutputFormat> {
    OutputFormat::from_str(format)
}
