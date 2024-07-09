use anyhow::Result;
use clap::Parser;
use password::{process_password, Args};

fn main() -> Result<()> {
    let args = Args::parse();
    let generated_password = process_password(
        args.length,
        args.upper_case,
        args.lower_case,
        args.number,
        args.symbol,
    )?;

    println!("{}", generated_password);

    Ok(())
}
