use anyhow::{Error, Result};
use b64::{process_decode, process_encode, Args};
use clap::Parser;

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let res = match args.command {
        b64::B64Command::Encode(opts) => process_encode(&opts.input, opts.format)?,
        b64::B64Command::Decode(opts) => process_decode(&opts.input, opts.format)?,
    };
    println!("{:?}", res);
    Ok(())
}
