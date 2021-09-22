use std::io::Write;

use crate::constants::UTF8_TO_ENCODED_MAP;
use crate::utils::div_rem;
use crate::{verify, DecodingError};

/// Encodes a string into a G60 encoding format.
///
/// # Errors
/// An error will be arise in the following cases:
/// - if `buffer` does not have at least `ceil(8 * encoded.len() / 11)` of size.
/// - if `encoded` is not a valid G60 encoded string.
/// - if `encoded` is not a valid UTF8 string.
pub fn decode_to_string(encoded: &str) -> Result<String, DecodingError> {
    let buffer = decode(encoded)?;

    match String::from_utf8(buffer) {
        Ok(v) => Ok(v),
        Err(e) => Err(DecodingError::InvalidUTF8String {
            bytes: e.into_bytes(),
        }),
    }
}

/// Decodes a G60 encoding string to a list of bytes.
///
/// # Errors
/// An error will be arise in the following cases:
/// - if `buffer` does not have at least `ceil(8 * encoded.len() / 11)` of size.
/// - if `encoded` is not a valid G60 encoded string.
pub fn decode(encoded: &str) -> Result<Vec<u8>, DecodingError> {
    verify(encoded)?;

    let mut buffer = vec![0; compute_buffer_size(encoded.len())];

    unsafe {
        decode_in_buffer_unchecked(encoded, &mut buffer)?;
    }

    Ok(buffer)
}

/// Decodes a G60 encoding string to a list of bytes.
/// The result is placed into `buffer` and returns the number of elements written.
///
/// # Errors
/// An error will be arise in the following cases:
/// - if `buffer` does not have at least `ceil(8 * encoded.len() / 11)` of size.
/// - if `encoded` is not a valid G60 encoded string.
pub fn decode_in_buffer(encoded: &str, buffer: &mut [u8]) -> Result<usize, DecodingError> {
    verify(encoded)?;

    unsafe { decode_in_buffer_unchecked(encoded, buffer) }
}

/// Decodes a G60 encoding string to a list of bytes without checking for its validity.
///
/// # Safety
/// This method will panic or return an undefined value if `encoded` is not a valid G60 string.
///
/// # Errors
/// An error will be arise if `buffer` does not have at least `ceil(8 * encoded.len() / 11)` of size.
pub unsafe fn decode_unchecked(encoded: &str) -> Result<Vec<u8>, DecodingError> {
    let mut buffer = vec![0; compute_buffer_size(encoded.len())];

    decode_in_buffer_unchecked(encoded, &mut buffer)?;

    Ok(buffer)
}

/// Decodes a G60 encoding string to a list of bytes without checking for its validity.
/// The result is placed into `buffer` and returns the number of elements written.
///
/// # Safety
/// This method will panic or return an undefined value if `encoded` is not a valid G60 string.
///
/// # Errors
/// An error will be arise if `buffer` does not have at least `ceil(8 * encoded.len() / 11)` of size.
pub unsafe fn decode_in_buffer_unchecked(
    encoded: &str,
    mut buffer: &mut [u8],
) -> Result<usize, DecodingError> {
    let bytes = encoded.as_bytes();
    let required_buffer_size = compute_buffer_size(bytes.len());

    if buffer.len() < required_buffer_size {
        return Err(DecodingError::NotEnoughSpaceInBuffer {
            actual: buffer.len(),
            required: required_buffer_size,
        });
    }

    // Complete groups.
    for chunk in bytes.chunks_exact(11) {
        let c0 = UTF8_TO_ENCODED_MAP[chunk[0] as usize] as usize;
        let c1 = match chunk.get(1) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c2 = match chunk.get(2) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c3 = match chunk.get(3) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c4 = match chunk.get(4) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c5 = match chunk.get(5) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c6 = match chunk.get(6) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c7 = match chunk.get(7) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c8 = match chunk.get(8) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c9 = match chunk.get(9) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c10 = match chunk.get(10) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };

        let (b1, r1) = div_rem(60 * c0 + c1, 14);
        let (b2, r2) = div_rem(c2, 3);
        let (b3, r3) = div_rem(c4, 20);
        let aux = 3 * c3 + b3;
        let b3_bis = aux >> 1;
        let r3_bis = aux & 0x1;
        let (b4, r4) = div_rem(60 * r3 + c5, 9);
        let b5 = c6 >> 1;
        let r5 = c6 & 0x1;
        let (b6, r6) = div_rem(60 * c7 + c8, 24);
        let (b7, r7) = div_rem(c9, 5);

        let decoded = [
            b1 as u8,
            (r1 * 20 + b2) as u8,
            (r2 * 90 + b3_bis) as u8,
            (128 * r3_bis + b4) as u8,
            (r4 * 30 + b5) as u8,
            (r5 * 150 + b6) as u8,
            (r6 * 12 + b7) as u8,
            (60 * r7 + c10) as u8,
        ];

        buffer.write_all(&decoded).unwrap();
    }

    // Last incomplete group.
    let last_group_length = bytes.len() - (bytes.len() / 11 * 11);
    if last_group_length != 0 {
        let chunk = &bytes[bytes.len() - last_group_length..];

        let c0 = UTF8_TO_ENCODED_MAP[chunk[0] as usize] as usize;
        let c1 = match chunk.get(1) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c2 = match chunk.get(2) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c3 = match chunk.get(3) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c4 = match chunk.get(4) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c5 = match chunk.get(5) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c6 = match chunk.get(6) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c7 = match chunk.get(7) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c8 = match chunk.get(8) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c9 = match chunk.get(9) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };
        let c10 = match chunk.get(10) {
            Some(v) => UTF8_TO_ENCODED_MAP[*v as usize] as usize,
            None => 0,
        };

        let (b1, r1) = div_rem(60 * c0 + c1, 14);
        let (b2, r2) = div_rem(c2, 3);
        let (b3, r3) = div_rem(c4, 20);
        let aux = 3 * c3 + b3;
        let b3_bis = aux >> 1;
        let r3_bis = aux & 0x1;
        let (b4, r4) = div_rem(60 * r3 + c5, 9);
        let b5 = c6 >> 1;
        let r5 = c6 & 0x1;
        let (b6, r6) = div_rem(60 * c7 + c8, 24);
        let (b7, r7) = div_rem(c9, 5);

        let decoded = [
            b1 as u8,
            (r1 * 20 + b2) as u8,
            (r2 * 90 + b3_bis) as u8,
            (128 * r3_bis + b4) as u8,
            (r4 * 30 + b5) as u8,
            (r5 * 150 + b6) as u8,
            (r6 * 12 + b7) as u8,
            (60 * r7 + c10) as u8,
        ];

        let elements_to_write = compute_buffer_size(last_group_length);
        buffer.write_all(&decoded[..elements_to_write]).unwrap();
    }

    Ok(required_buffer_size)
}

// ----------------------------------------------------------------------------
// AUX METHODS ----------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Computes `ceil(8 * encoded_length / 11)` faster using only integers.
fn compute_buffer_size(encoded_length: usize) -> usize {
    (encoded_length << 3) / 11
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::VerificationError;

    use super::*;

    #[test]
    fn test_compute_buffer_size() {
        for encoded_length in 0usize..100 {
            let real_value = (8.0 * encoded_length as f64 / 11.0).floor() as usize;
            let computed_value = compute_buffer_size(encoded_length);

            assert_eq!(
                computed_value, real_value,
                "Incorrect for {}",
                encoded_length
            );
        }
    }

    /// This will test also `decode`, `decode_in_buffer` and `decode_in_buffer_unchecked`.
    #[test]
    fn test_decode_to_string_ok() {
        let test = "Gt4CGFiHehzRzjCF16";
        let decoded = decode_to_string(test).expect("The decoding must succeed");

        assert_eq!(decoded, "Hello, world!", "Incorrect for '{}'", test);

        // --------------------------------------------------------------------

        let test = "Gt4CGFEHehzRzsCF26RHF";
        let decoded = decode_to_string(test).expect("The decoding must succeed");

        assert_eq!(decoded, "Hella, would???", "Incorrect for '{}'", test);
    }

    /// This will test also `decode`, `decode_in_buffer` and `decode_in_buffer_unchecked`.
    #[test]
    fn test_decode_to_string_err() {
        let test = "Gt4CGFiHehzRzjCF16x";
        let error = decode_to_string(test).expect_err("The decoding cannot succeed");

        assert_eq!(
            error,
            DecodingError::InputVerification(VerificationError::InvalidLength),
            "Incorrect for '{}'",
            test
        );

        // --------------------------------------------------------------------

        let test = "xxxxxxxxxxx";
        let error = decode_to_string(test).expect_err("The decoding cannot succeed");

        assert_eq!(
            error,
            DecodingError::InvalidUTF8String {
                bytes: vec![248, 119, 86, 247, 208, 38, 7, 177]
            },
            "Incorrect for '{}'",
            test
        );
    }

    /// This will test also `decode_in_buffer` and `decode_in_buffer_unchecked`.
    #[test]
    fn test_decode_in_buffer_exact_buffer() {
        let test = "Gt4CGFiHehzRzjCF16";
        let mut result_buffer = vec![0; 13];
        let decoded_chars =
            decode_in_buffer(test, &mut result_buffer).expect("The decoding must succeed");

        let result = b"Hello, world!".to_vec();

        assert_eq!(decoded_chars, 13, "Incorrect chars");
        assert_eq!(result_buffer, result, "Incorrect buffer result");
    }

    /// This will test also `decode_in_buffer` and `decode_in_buffer_unchecked`.
    #[test]
    fn test_decode_in_buffer_bigger_buffer() {
        let test = "Gt4CGFiHehzRzjCF16";
        let mut result_buffer = vec![0; 15];
        let decoded_chars =
            decode_in_buffer(test, &mut result_buffer).expect("The decoding must succeed");

        let mut result = b"Hello, world!".to_vec();
        result.push(0);
        result.push(0);

        assert_eq!(decoded_chars, 13, "Incorrect chars");
        assert_eq!(result_buffer, result, "Incorrect buffer result");
    }

    /// This will test also `decode_in_buffer` and `decode_in_buffer_unchecked`.
    #[test]
    fn test_decode_in_buffer_shorter_buffer() {
        let test = "Gt4CGFiHehzRzjCF16";
        let mut result_buffer = vec![0; 10];
        let error =
            decode_in_buffer(test, &mut result_buffer).expect_err("The decoding cannot succeed");

        assert_eq!(
            error,
            DecodingError::NotEnoughSpaceInBuffer {
                actual: 10,
                required: 13,
            },
            "Incorrect for '{}'",
            test
        );
    }
}
