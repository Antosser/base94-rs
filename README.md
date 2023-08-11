# Base94 Encoding Library

Convert binary data to a compact text-based format using Base94 encoding. Effortlessly encode and decode data for a wide range of use cases. 🔐🔍

## Features

- Encode binary data into a Base94-encoded string.
- Decode Base94-encoded strings back to their original binary form.

## Usage

To use this library, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
base94 = "0.1.0"
```
Then, in your Rust code:
```rust
use base94::{base94_encode, base94_decode};

fn main() {
    let data = b"Hello, World!";
    let base = 94;

    // Encode data
    let encoded = base94_encode(data, base);
    println!("Encoded: {}", encoded);

    // Decode data
    let decoded = base94_decode(&encoded, base).unwrap();
    println!("Decoded: {:?}", decoded);
}
```
## Supported Bases

The encoding and decoding functions support various bases within the range of 2 to 94. The specified base must be consistent between encoding and decoding operations.

## Examples

Encoding and decoding example with a base of 50:
```rust
use base94::{base94_encode, base94_decode};

let data = b"Example data for encoding.";
let base = 50;

let encoded = base94_encode(data, base);
let decoded = base94_decode(&encoded, base).unwrap();

assert_eq!(decoded, data);
```
