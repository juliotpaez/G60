use std::fmt::{Debug, Display, Formatter};

/// A wrapping error of all possible errors of the G60 encoding library.
#[derive(Debug)]
pub enum Error {
    Encoding(EncodingError),
    Decoding(DecodingError),
    Verification(VerificationError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Encoding(e) => Display::fmt(&e, f),
            Error::Decoding(e) => Display::fmt(&e, f),
            Error::Verification(e) => Display::fmt(&e, f),
        }
    }
}

impl std::error::Error for Error {}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// An error returned during the encoding process.
#[derive(Debug, Eq, PartialEq)]
pub enum EncodingError {
    /// The result buffer has not enough space to held the encoding result.
    NotEnoughSpaceInBuffer { actual: usize, required: usize },
}

impl Display for EncodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EncodingError {}

impl From<EncodingError> for Error {
    fn from(v: EncodingError) -> Self {
        Self::Encoding(v)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// An error returned during the decoding process.
#[derive(Debug, Eq, PartialEq)]
pub enum DecodingError {
    /// Error during verification of the input.
    InputVerification(VerificationError),
    /// The decoded bytes are not a valid UTF8 string.
    InvalidUTF8String { bytes: Vec<u8> },
    /// The result buffer has not enough space to held the decoding result.
    NotEnoughSpaceInBuffer { actual: usize, required: usize },
}

impl Display for DecodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DecodingError {}

impl From<DecodingError> for Error {
    fn from(v: DecodingError) -> Self {
        Self::Decoding(v)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// An error returned during the verification process.
#[derive(Debug, Eq, PartialEq)]
pub enum VerificationError {
    /// The length of the encoded string is incorrect.
    InvalidLength,
    /// Invalid byte in the encoded string.
    InvalidByte { index: usize, byte: u8 },
}

impl Display for VerificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for VerificationError {}

impl From<VerificationError> for Error {
    fn from(v: VerificationError) -> Self {
        Self::Verification(v)
    }
}

impl From<VerificationError> for DecodingError {
    fn from(v: VerificationError) -> Self {
        Self::InputVerification(v)
    }
}
