use crate::decoding::{compute_chunk, compute_decoded_size};
use crate::errors::VerificationError;

/// Verifies `content` is a valid G60 encoded string.
///
/// # Errors
/// An error will be thrown in the following cases:
/// - if `encoded` is not a valid G60 encoded string.
/// - if `encoded` is not canonical.
pub fn verify(encoded: &str) -> Result<(), VerificationError> {
    let bytes = encoded.as_bytes();

    // Check length.
    let last_group_length = bytes.len() - bytes.len() / 11 * 11;
    if let 1 | 4 | 8 = last_group_length {
        return Err(VerificationError::InvalidLength);
    }

    // Complete groups.
    let mut chunk_index = 0;
    for chunk in bytes.chunks_exact(11) {
        compute_chunk(chunk_index, chunk)?;
        chunk_index += 11;
    }

    // Last incomplete group.
    if last_group_length != 0 {
        let chunk = &bytes[bytes.len() - last_group_length..];
        let decoded = compute_chunk(chunk_index, chunk)?;
        let elements_to_write = compute_decoded_size(last_group_length);

        if decoded[elements_to_write..].iter().any(|v| *v != 0) {
            return Err(VerificationError::NotCanonical);
        }
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encode;

    #[test]
    fn test_verify_ok() {
        for length in 0..16 {
            for byte in 0..=255 {
                let bytes = vec![byte; length];
                let encoded = encode(&bytes);
                verify(&encoded).expect("The verification must succeed");
            }
        }
    }

    #[test]
    fn test_verify_invalid_length() {
        let test = "JKLMNPQRSTUx";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidLength,
            "Incorrect for '{}'",
            test
        );

        // --------------------------------------------------------------------

        let test = "JKLMNPQRSTUxxxx";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidLength,
            "Incorrect for '{}'",
            test
        );

        // --------------------------------------------------------------------

        let test = "JKLMNPQRSTUxxxxxxxx";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidLength,
            "Incorrect for '{}'",
            test
        );
    }

    #[test]
    fn test_verify_invalid_characters() {
        let test = "Hello, world!";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidByte {
                index: 5,
                byte: b',',
            },
            "Incorrect for '{}'",
            test
        );

        // --------------------------------------------------------------------

        let test = "THIS IS A TEST";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidByte {
                index: 2,
                byte: b'I',
            },
            "Incorrect for '{}'",
            test
        );

        // --------------------------------------------------------------------

        let test = "TESTONTEST";
        let error = verify(test).expect_err("The verification must fail");

        assert_eq!(
            error,
            VerificationError::InvalidByte {
                index: 4,
                byte: b'O',
            },
            "Incorrect for '{}'",
            test
        );
    }

    #[test]
    fn test_not_canonical() {
        for i in ["0f", "2F", "5y", "BU", "Gv", "Nr", "Xd"] {
            assert_eq!(
                verify(i),
                Err(VerificationError::NotCanonical),
                "Incorrect for '{}'",
                i
            );
        }
    }
}
