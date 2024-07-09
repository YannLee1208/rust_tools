use clap::Parser;

#[derive(Parser, Debug)]
#[command(name="password", version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = 8)]
    pub length: u8,

    #[arg(short, long, default_value_t = true)]
    pub upper_case: bool,

    #[arg(short, long, default_value_t = true)]
    pub lower_case: bool,

    #[arg(short, long, default_value_t = true)]
    pub number: bool,

    #[arg(short, long, default_value_t = true)]
    pub symbol: bool,
}
