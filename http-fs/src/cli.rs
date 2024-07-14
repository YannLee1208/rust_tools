use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long)]
    pub dir_path: PathBuf,

    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
