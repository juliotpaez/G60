use crate::errors::VerificationError;
use crate::verification::{CHAR_RANGE_LOWERCASE, CHAR_RANGE_NUMBERS, CHAR_RANGE_UPPERCASE};

pub fn verify(content: &str) -> Result<bool, VerificationError> {
    let bytes = content.as_bytes();

    // Check chars.
    for (index, c) in bytes.iter().enumerate() {
        if CHAR_RANGE_UPPERCASE.contains(c) {
            if let b'O' | b'I' = *c {
                return Err(VerificationError::InvalidByte { index, byte: *c });
            }
        } else if !CHAR_RANGE_NUMBERS.contains(c) && !CHAR_RANGE_LOWERCASE.contains(c) {
            return Err(VerificationError::InvalidByte { index, byte: *c });
        }
    }

    // Check length.
    let remaining_bytes = bytes.len() - bytes.len() / 11 * 11;
    Ok(matches!(remaining_bytes, 1 | 4 | 8))
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_ok() {
        // Correct
        for test in [
            "0123456789ABCDEFGH",
            "JKLMNPQRSTUVWXYZab",
            "cdefghijklmnopqrst",
            "uvwxyz0123456789AB",
        ] {
            let result =
                verify(test).unwrap_or_else(|_| panic!("Verify must succeed for '{}'", test));
            assert!(!result, "Incorrect for '{}'", test);
        }

        // Incorrect
        for test in ["1", "1234", "12345678"] {
            let result =
                verify(test).unwrap_or_else(|_| panic!("Verify must succeed for '{}'", test));
            assert!(result, "Incorrect for '{}'", test);
        }
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
}
