use anyhow::Result;
use clap::Parser;
use password::process_password;

#[derive(Parser, Debug)]
#[command(name="password", version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = 8)]
    length: u8,

    #[arg(short, long, default_value_t = true)]
    upper_case: bool,

    #[arg(short, long, default_value_t = true)]
    lower_case: bool,

    #[arg(short, long, default_value_t = true)]
    number: bool,

    #[arg(short, long, default_value_t = true)]
    symbol: bool,
}

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
