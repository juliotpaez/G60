use crate::{decoding, encoding};

/// This method assumes `encoded` is correct.
pub fn canonicalize_in_place(encoded: &mut String) {
    let bytes = unsafe { encoded.as_bytes_mut() };
    let bytes_length = bytes.len();

    // Complete groups.
    for chunk_id in 0..(bytes_length / 11) {
        let position = chunk_id * 11;
        let chunk = &mut bytes[position..position + 11];
        let decoded = decoding::compute_chunk(chunk);
        let encoded = encoding::compute_chunk(&decoded);

        bytes[position..(position + 11)].clone_from_slice(&encoded[position..(position + 11)]);
    }

    // Last incomplete group.
    let last_group_length = bytes_length - (bytes_length / 11 * 11);
    if last_group_length != 0 {
        let chunk = &mut bytes[bytes_length - last_group_length..];
        let decoded = decoding::compute_chunk(chunk);
        let elements_to_write = decoding::compute_decoded_size(last_group_length);
        let encoded = encoding::compute_chunk(&decoded[..elements_to_write]);

        bytes[(bytes_length - last_group_length)..bytes_length]
            .clone_from_slice(&encoded[(bytes_length - last_group_length)..bytes_length]);
    }
}

pub fn is_canonical(encoded: &str) -> bool {
    let bytes = encoded.as_bytes();
    let bytes_length = bytes.len();

    // Complete groups.
    for chunk_id in 0..(bytes_length / 11) {
        let position = chunk_id * 11;
        let chunk = &bytes[position..position + 11];
        let decoded = decoding::compute_chunk(chunk);
        let encoded = encoding::compute_chunk(&decoded);

        for p in position..position + 11 {
            if bytes[p] != encoded[p] {
                return false;
            }
        }
    }

    // Last incomplete group.
    let last_group_length = bytes_length - (bytes_length / 11 * 11);
    if last_group_length != 0 {
        let chunk = &bytes[bytes_length - last_group_length..];
        let decoded = decoding::compute_chunk(chunk);
        let elements_to_write = decoding::compute_decoded_size(last_group_length);
        let encoded = encoding::compute_chunk(&decoded[..elements_to_write]);

        for p in bytes_length - last_group_length..bytes_length {
            if bytes[p] != encoded[p] {
                return false;
            }
        }
    }

    true
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
            let mut encoded_string = encoded.to_string();
            canonicalize_in_place(&mut encoded_string);
            assert_eq!(encoded_string, encoded, "Incorrect yes value: {}", encoded)
        }

        // No
        let mut encoded_string = "001".to_string();
        canonicalize_in_place(&mut encoded_string);
        assert_eq!(encoded_string, "000", "Incorrect yes value: 001");

        let mut encoded_string = "0Co00000000".to_string();
        canonicalize_in_place(&mut encoded_string);
        assert_eq!(encoded_string, "00000000000", "Incorrect yes value: 001")
    }
}
