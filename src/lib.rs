//! # Base94 Encoding Library
//!
//! This crate provides functions for encoding and decoding data using Base94 encoding.
//! Base94 encoding is a method of converting binary data into a text-based format using a larger
//! character set than Base64. The encoding and decoding functions in this crate allow you to
//! convert data between its original binary form and a Base94-encoded string representation.
//!
//! ## Usage
//!
//! To use this library, you can include it as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! base94 = {current_version}
//! ```
//!
//! Then, you can use the provided functions in your Rust code:
//!
//! ```rust
//! use base94::{encode, decode};
//!
//! let data = b"Hello, World!";
//! let base = 94;
//! let encoded = encode(data, base);
//! let decoded = decode(&encoded, base).unwrap();
//!
//! assert_eq!(decoded, data);
//! ```
//!
//! ## Supported Bases
//!
//! The encoding and decoding functions support various bases within the range of 2 to 94.
//! The specified base must be consistent between encoding and decoding operations.
//!
//! ## Examples
//!
//! ```
//! use base94::{encode, decode};
//!
//! let data = b"Example data for encoding.";
//! let base = 50;
//!
//! let encoded = encode(data, base);
//! let decoded = decode(&encoded, base).unwrap();
//!
//! assert_eq!(decoded, data);
//! ```
//!

use num::BigUint;
use num::Integer;
use num::ToPrimitive;
use thiserror::Error;

pub static CHARACTERS: &[u8; 94] = include_bytes!("characters.txt");

/// Encodes a slice of bytes into a Base94-encoded string using the specified base.
///
/// Base94 encoding is a method of converting binary data into a text-based format using
/// a larger character set than Base64. The provided base specifies the number of unique
/// characters used in the encoding, ranging from 2 to 94. The encoded string can later be
/// decoded back to the original data using the `decode` function.
///
/// # Arguments
///
/// * `data` - A slice of bytes representing the data to be encoded.
/// * `base` - The base used for encoding. Must be between 2 and 94 (inclusive).
///
/// # Panics
///
/// This function panics if the specified base is outside the valid range (2 to 94).
///
/// # Returns
///
/// A Base94-encoded string representation of the input data.
///
/// # Examples
///
/// ```
/// use base94::encode;
///
/// let data = b"Hello, World!";
/// let base = 94;
/// let encoded = encode(data, base);
/// println!("Encoded: {}", encoded);
/// ```
pub fn encode(data: &[u8], base: u8) -> String {
    assert!(base <= 94, "Base must be less than or equal to 94");
    assert!(base >= 2, "Base must be greater than or equal to 2");

    let mut num = BigUint::from_bytes_le(data);
    let mut out = String::new();

    while num > BigUint::from(0u8) {
        let (div, rem) = num.div_rem(&BigUint::from(base));
        num = div;
        out.push(CHARACTERS[rem.to_usize().unwrap()] as char);
    }

    out
}

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Invalid character '{c}' at position {position}")]
    InvalidCharacter { c: u8, position: usize },
}

/// Decodes a Base94-encoded string back to its original byte representation using the specified base.
///
/// Base94 encoding is a method of converting binary data into a text-based format using
/// a larger character set than Base64. The provided base specifies the number of unique
/// characters used in the encoding, ranging from 2 to 94. The encoded string should have
/// been generated using the `encode` function with the same base.
///
/// # Arguments
///
/// * `encoded` - A Base94-encoded string to be decoded.
/// * `base` - The base used for decoding. Must match the base used for encoding.
///
/// # Returns
///
/// An optional vector of bytes representing the decoded original data. Returns `None`
/// if the decoding process encounters invalid characters.
///
/// # Examples
///
/// ```
/// use base94::decode;
///
/// let encoded = "A@#D9e@D9n9RRb6^";
/// let base = 94;
/// let decoded = decode(encoded, base).unwrap();
/// println!("Decoded: {:?}", decoded);
/// ```
pub fn decode(encoded: &str, base: u8) -> Result<Vec<u8>, DecodeError> {
    let mut num = BigUint::from(0u8);
    let mut power = BigUint::from(1u8);

    for (i, c) in encoded.chars().enumerate() {
        let index =
            CHARACTERS
                .iter()
                .position(|&x| x == c as u8)
                .ok_or(DecodeError::InvalidCharacter {
                    c: c as u8,
                    position: i,
                })?;
        num += BigUint::from(index) * &power;
        power *= BigUint::from(base);
    }

    let out = num.to_bytes_le();
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAX_BASE: u8 = 94;

    #[test]
    fn test_encode_decode_empty() {
        for base in 2..=MAX_BASE {
            let data = [0];
            let encoded = encode(&data, base);
            let decoded = decode(&encoded, base).unwrap();
            assert_eq!(decoded, data);
        }
    }

    #[test]
    fn test_encode_decode_single_byte() {
        for base in 2..=MAX_BASE {
            let data = [65]; // ASCII value of 'A'
            let encoded = encode(&data, base);
            let decoded = decode(&encoded, base).unwrap();
            assert_eq!(decoded, data);
        }
    }

    #[test]
    fn test_encode_decode_hello_world() {
        for base in 2..=MAX_BASE {
            let data = b"Hello, World!";
            let encoded = encode(data, base);
            let decoded = decode(&encoded, base).unwrap();
            assert_eq!(decoded, data);
        }
    }

    #[test]
    fn test_encode_decode_max_value() {
        for base in 2..=MAX_BASE {
            let data = [255, 255, 255]; // Three bytes of value 255
            let encoded = encode(&data, base);
            let decoded = decode(&encoded, base).unwrap();
            assert_eq!(decoded, data);
        }
    }

    #[test]
    fn test_encode_decode_large_data() {
        for base in 2..=MAX_BASE {
            let data = vec![42; 1000]; // 1000 bytes of value 42 ('*')
            let encoded = encode(&data, base);
            let decoded = decode(&encoded, base).unwrap();
            assert_eq!(decoded, data);
        }
    }
}
