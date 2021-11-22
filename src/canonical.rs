use crate::{decode, encode, DecodingError};

/// Get the canonical form of an encoded string.
/// Uses `Cow` to not allocate unless necessary.
pub fn canonicalize(encoded: &str) -> Result<String, DecodingError> {
    let decoded = decode(encoded)?;
    Ok(encode(&decoded))
}

/// Return whether a representation is canonical or not.
pub fn is_canonical(encoded: &str) -> bool {
    let canonical = match canonicalize(encoded) {
        Ok(v) => v,
        Err(_) => return false,
    };

    encoded == canonical
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_canonical() {
        // Yes
        for encoded in ["010", "34564657567"] {
            assert!(is_canonical(encoded), "Incorrect yes value: {}", encoded)
        }

        // No
        for encoded in ["001", "012", "0Co00000000"] {
            assert!(!is_canonical(encoded), "Incorrect no value: {}", encoded)
        }
    }

    #[test]
    fn test_canonical() {
        // Yes
        for encoded in ["010", "34564657567"] {
            let result = canonicalize(encoded)
                .unwrap_or_else(|_| panic!("Canonicalize must succeed: {}", encoded));
            assert_eq!(result, encoded, "Incorrect yes value: {}", encoded)
        }

        // No
        let result =
            canonicalize("001").unwrap_or_else(|_| panic!("Canonicalize must succeed: 001"));
        assert_eq!(result, "000", "Incorrect yes value: 001");

        let result = canonicalize("0Co00000000")
            .unwrap_or_else(|_| panic!("Canonicalize must succeed: 001"));
        assert_eq!(result, "00000000000", "Incorrect yes value: 001")
    }
}
