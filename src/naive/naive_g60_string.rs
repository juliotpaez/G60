use crate::errors::VerificationError;
use crate::{encoding, naive, G60String};
use std::convert::TryFrom;
use std::str::FromStr;

/// A G60 encoded string.
///
/// This type is different from `G60String` in that this does not imply correctness, i.e. the content
/// is decodable.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NaiveG60String {
    value: String,
}

impl NaiveG60String {
    // CONSTRUCTORS -----------------------------------------------------------

    /// Creates a new `NaiveG60String` from an already G60 encoded string.
    pub fn new(string: String) -> Result<NaiveG60String, VerificationError> {
        Self::verify(string.as_str())?;

        unsafe { Ok(Self::new_unchecked(string)) }
    }

    /// Creates a new `NaiveG60String` from an already G60 encoded string.
    ///
    /// # Safety
    /// If `string` is not a valid G60 encoded string the future behaviour of this struct is undefined.
    pub unsafe fn new_unchecked(string: String) -> NaiveG60String {
        Self { value: string }
    }

    /// Creates a new `NaiveG60String` from an already G60 encoded string.
    pub fn new_str(string: &str) -> Result<NaiveG60String, VerificationError> {
        Self::verify(string)?;

        unsafe { Ok(Self::new_str_unchecked(string)) }
    }

    /// Creates a new `NaiveG60String` from an already G60 encoded string.
    ///
    /// # Safety
    /// If `string` is not a valid G60 encoded string the future behaviour of this struct is undefined.
    pub unsafe fn new_str_unchecked(string: &str) -> NaiveG60String {
        Self {
            value: string.to_string(),
        }
    }

    /// Encodes a list of bytes into a `NaiveG60String`.
    pub fn encode(content: &[u8]) -> NaiveG60String {
        NaiveG60String {
            value: encoding::encode(content),
        }
    }

    /// Generates a random `NaiveG60String` of exactly `length` characters using a non-deterministic PRNG.
    /// Note: depending on the length it can result in a valid G60 encoded string or not.
    #[cfg(feature = "random")]
    pub fn random(length: usize) -> String {
        naive::random::exactly_random(length)
    }

    /// Generates a random `NaiveG60String` of exactly `length` characters using a basic but faster random generator.
    /// Note: depending on the length it can result in a valid G60 encoded string or not.
    #[cfg(feature = "random")]
    pub fn unsecure_random(length: usize) -> String {
        naive::random::exactly_unsecure_random(length)
    }

    /// Generates a random `NaiveG60String` of exactly `length` characters using a custom random generator.
    /// Note: depending on the length it can result in a valid G60 encoded string or not.
    #[cfg(feature = "random")]
    pub fn custom_random<F>(length: usize, rng: F) -> String
    where
        F: FnMut(&mut [u8]),
    {
        naive::random::exactly_custom_random(length, rng)
    }

    // GETTERS ----------------------------------------------------------------

    /// The textual representation of the `NaiveG60String`.
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    // METHODS ----------------------------------------------------------------

    /// Unwraps the inner content.
    pub fn unwrap_string(self) -> String {
        self.value
    }

    // STATIC METHODS ---------------------------------------------------------

    /// Verifies whether `encoded` is a valid naive G60 encoded string or not.
    ///
    /// # Errors
    /// An error will be thrown in the following cases:
    /// - if `encoded` contains an invalid G60 character.
    pub fn verify(encoded: &str) -> Result<bool, VerificationError> {
        naive::verification::verify(encoded)
    }
}

impl FromStr for NaiveG60String {
    type Err = VerificationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new_str(s)
    }
}

impl From<NaiveG60String> for String {
    fn from(v: NaiveG60String) -> Self {
        v.value
    }
}

impl AsRef<str> for NaiveG60String {
    fn as_ref(&self) -> &str {
        self.value.as_str()
    }
}

impl AsRef<String> for NaiveG60String {
    fn as_ref(&self) -> &String {
        &self.value
    }
}

impl AsMut<String> for NaiveG60String {
    fn as_mut(&mut self) -> &mut String {
        &mut self.value
    }
}

impl From<G60String> for NaiveG60String {
    fn from(v: G60String) -> Self {
        Self {
            value: v.unwrap_string(),
        }
    }
}

impl TryFrom<NaiveG60String> for G60String {
    type Error = VerificationError;

    fn try_from(value: NaiveG60String) -> Result<Self, Self::Error> {
        Self::new(value.value)
    }
}
