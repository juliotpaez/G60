use crate::errors::{DecodingError, VerificationError};
#[cfg(feature = "random")]
use crate::random;
use crate::{canonical, decoding, encoding, verification};
use std::borrow::Cow;
use std::io::Write;
use std::str::FromStr;

/// A correct G60 encoded string.
///
/// The correctness of this type implies that its content has a decodable string.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct G60String {
    value: String,
}

impl G60String {
    // CONSTRUCTORS -----------------------------------------------------------

    /// Creates a new `G60String` from an already G60 encoded string.
    pub fn new(string: String) -> Result<G60String, VerificationError> {
        Self::verify(string.as_str())?;

        unsafe { Ok(Self::new_unchecked(string)) }
    }

    /// Creates a new `G60String` from an already G60 encoded string.
    ///
    /// # Safety
    /// If `string` is not a valid G60 encoded string the future behaviour of this struct is undefined.
    pub unsafe fn new_unchecked(string: String) -> G60String {
        Self { value: string }
    }

    /// Creates a new `G60String` from an already G60 encoded string.
    pub fn new_str(string: &str) -> Result<G60String, VerificationError> {
        Self::verify(string)?;

        unsafe { Ok(Self::new_str_unchecked(string)) }
    }

    /// Creates a new `G60String` from an already G60 encoded string.
    ///
    /// # Safety
    /// If `string` is not a valid G60 encoded string the future behaviour of this struct is undefined.
    pub unsafe fn new_str_unchecked(string: &str) -> G60String {
        Self {
            value: string.to_string(),
        }
    }

    /// Encodes a list of bytes into a `G60String`.
    pub fn encode(content: &[u8]) -> G60String {
        G60String {
            value: encoding::encode(content),
        }
    }

    /// Generates a random `G60String` of `bytes` length using a non-deterministic PRNG.
    /// This method generates a canonical G60 string.
    #[cfg(feature = "random")]
    pub fn random_bytes(bytes: usize) -> String {
        random::random_bytes(bytes)
    }

    /// Generates a random `G60String` of `bytes` length using a basic but faster random generator.
    /// This method generates a canonical G60 string.
    #[cfg(feature = "random")]
    pub fn unsecure_random_bytes(bytes: usize) -> String {
        random::unsecure_random_bytes(bytes)
    }

    /// Generates a random `G60String` of `bytes` length using a custom random generator.
    /// This method generates a canonical G60 string.
    #[cfg(feature = "random")]
    pub fn custom_random_bytes<F>(bytes: usize, rng: F) -> String
    where
        F: FnMut(&mut [u8]),
    {
        random::custom_random_bytes(bytes, rng)
    }

    /// Generates a random `G60String` of at most `length` characters using a non-deterministic PRNG.
    /// This method generates a canonical G60 string.
    ///
    /// Note that if `length` is not valid for a G60 string, a string with `length - 1` will be generated instead.
    #[cfg(feature = "random")]
    pub fn random(length: usize) -> String {
        random::random_str(length)
    }

    /// Generates a random `G60String` of at most `length` characters using a basic but faster random generator.
    /// This method generates a canonical G60 string.
    ///
    /// Note that if `length` is not valid for a G60 string, a string with `length - 1` will be generated instead.
    #[cfg(feature = "random")]
    pub fn unsecure_random(length: usize) -> String {
        random::unsecure_random_str(length)
    }

    /// Generates a random `G60String` of at most `length` characters using a custom random generator.
    /// This method generates a canonical G60 string.
    ///
    /// Note that if `length` is not valid for a G60 string, a string with `length - 1` will be generated instead.
    #[cfg(feature = "random")]
    pub fn custom_random<F>(length: usize, rng: F) -> String
    where
        F: FnMut(&mut [u8]),
    {
        random::custom_random_str(length, rng)
    }

    // GETTERS ----------------------------------------------------------------

    /// The textual representation of the `G60String`.
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    /// Whether the `G60String` is in the canonical form or not.
    pub fn is_canonical(&self) -> bool {
        canonical::is_canonical(&self.value)
    }

    // METHODS ----------------------------------------------------------------

    /// Unwraps the inner content.
    pub fn unwrap_string(self) -> String {
        self.value
    }

    /// An alias for `decode`.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.decode()
    }

    /// Decodes the G60 string into a list of bytes.
    pub fn decode(&self) -> Vec<u8> {
        unsafe { decoding::decode_unchecked(self.value.as_str()) }
    }

    /// Decodes the `G60String` into a slice of bytes.
    /// The result is placed into `slice` and returns the number of elements written.
    ///
    /// # Errors
    /// An error will be thrown in the following cases:
    /// - if `slice` does not have at least `ceil(8 * encoded.len() / 11)` of size.
    pub fn decode_in_slice(&self, slice: &mut [u8]) -> Result<usize, DecodingError> {
        unsafe { decoding::decode_in_slice_unchecked(self.value.as_str(), slice) }
    }

    /// Decodes the `G60String` into a list of bytes.
    /// The result is writen in `writer` and returns the number of elements written.
    ///
    /// # Errors
    /// An error will be thrown in the following cases:
    /// - if the writing process fails.
    pub fn decode_in_writer<T: Write>(&self, slice: &mut T) -> Result<usize, DecodingError> {
        unsafe { decoding::decode_in_writer_unchecked(self.value.as_str(), slice) }
    }

    /// Get the canonical form of the `G60String`.
    /// Uses `Cow` to not allocate avoid unnecessary allocations.
    pub fn canonicalize(&self) -> Cow<G60String> {
        if canonical::is_canonical(&self.value) {
            Cow::Borrowed(self)
        } else {
            let mut encoded = self.value.clone();
            canonical::canonicalize_in_place(&mut encoded);

            Cow::Owned(G60String { value: encoded })
        }
    }

    /// Get the canonical form of the `G60String` replacing its current content
    /// and avoiding new allocations.
    pub fn canonicalize_in_place(&mut self) {
        canonical::canonicalize_in_place(&mut self.value)
    }

    // STATIC METHODS ---------------------------------------------------------

    /// Verifies whether `encoded` is a valid G60 encoded string or not.
    ///
    /// # Errors
    /// An error will be thrown in the following cases:
    /// - if `encoded` has an incorrect length.
    /// - if `encoded` contains an invalid G60 character.
    pub fn verify(encoded: &str) -> Result<(), VerificationError> {
        verification::verify(encoded)
    }
}

impl FromStr for G60String {
    type Err = VerificationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new_str(s)
    }
}

impl From<&[u8]> for G60String {
    fn from(v: &[u8]) -> Self {
        Self::encode(v)
    }
}

impl From<G60String> for String {
    fn from(v: G60String) -> Self {
        v.value
    }
}

impl AsRef<str> for G60String {
    fn as_ref(&self) -> &str {
        self.value.as_str()
    }
}

impl AsRef<String> for G60String {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl AsMut<String> for G60String {
    fn as_mut(&mut self) -> &mut String {
        &mut self.value
    }
}
