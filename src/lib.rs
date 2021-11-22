//! A G60 format (de)encoder for rust.
//!
//! [![](https://!img.shields.io/crates/v/g60.svg)](https://crates.io/crates/g60)
//! [![Docs](https://docs.rs/g60/badge.svg)](https://docs.rs/g60)
//!
//! ## Examples
//!
//! ```rust
//! # use g60::{G60String};
//!
//! # fn main() {
//!     let origin = "Hello, world!";
//!     let encoded = "Gt4CGFiHehzRzjCF16";
//!
//!     assert_eq!(G60String::encode(origin.as_bytes()).as_str(), encoded);
//!     assert_eq!(origin.as_bytes(), G60String::new_str(encoded).unwrap().decode());
//! # }
//! ```

pub use encoding::encode_in_slice;
pub use encoding::encode_in_writer;
pub use g60_string::*;

mod canonical;
mod constants;
mod decoding;
mod encoding;
pub mod errors;
mod g60_string;
#[cfg(feature = "naive")]
mod naive;
#[cfg(feature = "random")]
mod random;
mod utils;
mod verification;
