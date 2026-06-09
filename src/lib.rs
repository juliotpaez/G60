//! A G60 format (de)encoder for rust.
//!
//! [![](https://!img.shields.io/crates/v/g60.svg)](https://crates.io/crates/g60)
//! [![Docs](https://docs.rs/g60/badge.svg)](https://docs.rs/g60)
//!
//! ## Examples
//!
//! ```rust
//! # fn main() {
//!     let origin = "Hello, world!";
//!     let encoded = "Gt4CGFiHehzRzjCF16";
//!
//!     assert_eq!(g60::encode(origin.as_bytes()), encoded);
//!     assert_eq!(origin.as_bytes(), g60::decode(encoded).unwrap());
//! # }
//! ```

pub use decoding::decode;
pub use decoding::decode_in_slice;
pub use decoding::decode_in_writer;
pub use encoding::encode;
pub use encoding::encode_in_slice;
pub use encoding::encode_in_writer;
pub use verification::verify;

mod constants;
mod decoding;
mod encoding;
pub mod errors;
mod utils;
mod verification;

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn roundtrip(bytes: Vec<u8>) {
            let encoded = crate::encode(&bytes);
            let decoded = crate::decode(&encoded).unwrap();
            prop_assert_eq!(bytes, decoded);
        }

        #[test]
        fn valid_encoding_verifies(bytes: Vec<u8>) {
            let encoded = crate::encode(&bytes);
            prop_assert!(crate::verify(&encoded).is_ok());
        }
    }
}
