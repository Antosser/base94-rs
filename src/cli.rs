use std::fs;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[clap(version, author, about)]
struct Cli {
    operation: Operation,

    input: String,

    output: String,

    #[clap(short, long, default_value = "94")]
    base: u8,
}

#[derive(Clone, ValueEnum)]
enum Operation {
    Encode,
    Decode,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let input = fs::read(&args.input)?;

    let output = match args.operation {
        Operation::Encode => base94::encode(&input, args.base).as_bytes().to_vec(),
        Operation::Decode => base94::decode(std::str::from_utf8(&input)?, args.base)?,
    };

    fs::write(&args.output, output)?;

    Ok(())
}
