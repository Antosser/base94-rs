# Base94 Encoding Library

Convert binary data to a compact text-based format using Base94 encoding. Effortlessly encode and decode data for a wide range of use cases. 🔐🔍

## Features

- Encode binary data into a Base94-encoded string.
- Decode Base94-encoded strings back to their original binary form.

## Usage
### As a library

To use this library, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
base94 = "0.1.0"
```
Then, in your Rust code:
```rust
use base94::{encode, decode};

fn main() {
    let data = b"Hello, World!";
    let base = 94;

    // Encode data
    let encoded = encode(data, base);
    println!("Encoded: {}", encoded);

    // Decode data
    let decoded = decode(&encoded, base).unwrap();
    println!("Decoded: {:?}", decoded);
}
```

### As a CLI
To install the CLI, run `cargo install base94`. This will install the `base94cli` binary to your system.
```
Base94 encoding/decoding library

Usage: base94cli.exe [OPTIONS] <OPERATION> <INPUT> <OUTPUT>

Arguments:
  <OPERATION>  Whether to encode or decode the input [possible values: encode, decode]
  <INPUT>      The input file to encode or decode
  <OUTPUT>     The output file to write the result to

Options:
  -b, --base <BASE>  The base to use for encoding or decoding. Must be between 2 and 94 (inclusive) [default: 94]
  -h, --help         Print help
  -V, --version      Print version
```

## Supported Bases

The encoding and decoding functions support various bases within the range of 2 to 94. The specified base must be consistent between encoding and decoding operations.

## Examples

Encoding and decoding example with a base of 50:
```rust
use base94::{encode, decode};

let data = b"Example data for encoding.";
let base = 50;

let encoded = encode(data, base);
let decoded = decode(&encoded, base).unwrap();

assert_eq!(decoded, data);
```