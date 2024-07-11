use clap::{command, Parser};
use csv_transformer::process_csv_transformer;

#[derive(Parser, Debug)]
#[command(name="csv_transformer",version, about="transform csv file to other format", long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    // default_value can only be &str, and transform to the corresponding type. default_value_t should be the same type as value type
    #[arg(short, long, default_value = "output.json")]
    output: String,
}

fn main() {
    let arg = Args::parse();
    match process_csv_transformer(&arg.input, &arg.output) {
        Err(e) => println!("Error : {:?}", e),
        Ok(()) => println!("success"),
    };
}
