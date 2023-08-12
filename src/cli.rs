use std::fs;

use clap::{CommandFactory, Parser, ValueEnum};

#[derive(Parser)]
#[clap(version, author, about)]
struct Cli {
    /// Whether to encode or decode the input.
    operation: Operation,

    /// The input file to encode or decode.
    input: String,

    /// The output file to write the result to.
    output: String,

    /// The base to use for encoding or decoding. Must be between 2 and 94 (inclusive).
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
    if args.base < 2 || args.base > 94 {
        Cli::command()
            .error(
                clap::error::ErrorKind::InvalidValue,
                "Base must be between 2 and 94 (inclusive)",
            )
            .exit();
    }

    let input = fs::read(&args.input)?;

    let output = match args.operation {
        Operation::Encode => base94::encode(&input, args.base).as_bytes().to_vec(),
        Operation::Decode => base94::decode(std::str::from_utf8(&input)?, args.base)?,
    };

    fs::write(&args.output, output)?;

    Ok(())
}
