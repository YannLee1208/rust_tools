use std::str::FromStr;

use anyhow::{anyhow, Ok};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="b64", version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: B64Command,
}

#[derive(Parser, Debug, Clone)]

pub enum B64Command {
    Encode(EncodeOpts),
    Decode(DecodeOpts),
}

#[derive(Debug, Parser, Clone)]
pub struct EncodeOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,

    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Parser, Clone)]
pub struct DecodeOpts {
    #[arg(short, long, default_value = "-")]
    pub input: String,

    #[arg(short, long, default_value = "standard")]
    pub format: Base64Format,
}

#[derive(Debug, Clone)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

impl FromStr for Base64Format {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlSafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow!("Not valid format")),
        }
    }
}
