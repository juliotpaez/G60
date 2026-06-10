use std::fmt::{Display, Formatter};

/// A wrapping error of all possible errors of the G60 encoding library.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Encoding(EncodingError),
    Decoding(DecodingError),
    Verification(VerificationError),
}

impl std::error::Error for Error {
    // GETTERS ----------------------------------------------------------------

    /// Gets the source of the error.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Encoding(error) => Some(error),
            Error::Decoding(error) => Some(error),
            Error::Verification(error) => Some(error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Encoding(error) => Display::fmt(&error, f),
            Error::Decoding(error) => Display::fmt(&error, f),
            Error::Verification(error) => Display::fmt(&error, f),
        }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// An error returned during the encoding process.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum EncodingError {
    /// The result buffer has not enough space to held the encoding result.
    NotEnoughSpaceInSlice { actual: usize, required: usize },
    /// A writer error.
    WritingError(std::io::ErrorKind),
}

impl Display for EncodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodingError::NotEnoughSpaceInSlice { actual, required } => {
                write!(
                    f,
                    "not enough space in slice: actual {actual}, required {required}"
                )
            }
            EncodingError::WritingError(kind) => write!(f, "write error: {kind}"),
        }
    }
}

impl std::error::Error for EncodingError {}

impl From<EncodingError> for Error {
    fn from(error: EncodingError) -> Self {
        Self::Encoding(error)
    }
}

impl From<std::io::Error> for EncodingError {
    fn from(error: std::io::Error) -> Self {
        Self::WritingError(error.kind())
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// An error returned during the decoding process.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DecodingError {
    /// A verification error over the encoded string.
    Verification(VerificationError),

    /// The result buffer has not enough space to held the decoding result.
    NotEnoughSpaceInSlice { actual: usize, required: usize },

    /// A writer error.
    WritingError(std::io::ErrorKind),
}

impl Display for DecodingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DecodingError::Verification(error) => Display::fmt(error, f),
            DecodingError::NotEnoughSpaceInSlice { actual, required } => {
                write!(
                    f,
                    "not enough space in slice: actual {actual}, required {required}"
                )
            }
            DecodingError::WritingError(kind) => write!(f, "write error: {kind}"),
        }
    }
}

impl std::error::Error for DecodingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DecodingError::Verification(error) => Some(error),
            _ => None,
        }
    }
}

impl From<VerificationError> for DecodingError {
    fn from(error: VerificationError) -> Self {
        Self::Verification(error)
    }
}

impl From<std::io::Error> for DecodingError {
    fn from(error: std::io::Error) -> Self {
        Self::WritingError(error.kind())
    }
}

impl From<DecodingError> for Error {
    fn from(error: DecodingError) -> Self {
        Self::Decoding(error)
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

/// An error returned during the verification process.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VerificationError {
    /// The length of the encoded string is incorrect.
    InvalidLength,
    /// Invalid byte in the encoded string.
    InvalidByte { index: usize, byte: u8 },
    /// The encoded string is not canonical.
    NotCanonical,
}

impl Display for VerificationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VerificationError::InvalidLength => write!(f, "invalid encoded length"),
            VerificationError::InvalidByte { index, byte } => {
                write!(f, "invalid byte 0x{byte:02x} at index {index}")
            }
            VerificationError::NotCanonical => write!(f, "encoded string is not canonical"),
        }
    }
}

impl std::error::Error for VerificationError {}

impl From<VerificationError> for Error {
    fn from(error: VerificationError) -> Self {
        Self::Verification(error)
    }
}
