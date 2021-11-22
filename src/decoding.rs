use std::io::Write;

use crate::constants::UTF8_TO_ENCODED_MAP;
use crate::utils::div_rem;
use crate::{verify, DecodingError};

/// Encodes a string into a G60 encoding format.
///
/// # Errors
/// An error will be arise in the following cases:
/// - if `encoded` is not a valid G60 encoded string.
/// - if result is not a valid UTF8 string.
pub fn decode_to_string(encoded: &str) -> Result<String, DecodingError> {
    let slice = decode(encoded)?;

    match String::from_utf8(slice) {
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
/// - if `encoded` is not a valid G60 encoded string.
pub fn decode(encoded: &str) -> Result<Vec<u8>, DecodingError> {
    verify(encoded)?;

    let mut slice = vec![0; compute_decoded_size(encoded.len())];

    unsafe {
        decode_in_slice_unchecked(encoded, &mut slice)?;
    }

    Ok(slice)
}

/// Decodes a G60 encoding string to a list of bytes.
/// The result is placed into `slice` and returns the number of elements written.
///
/// # Errors
/// An error will be arise in the following cases:
/// - if `encoded` is not a valid G60 encoded string.
/// - if `slice` does not have at least `ceil(8 * encoded.len() / 11)` of size.
pub fn decode_in_slice(encoded: &str, slice: &mut [u8]) -> Result<usize, DecodingError> {
    verify(encoded)?;

    unsafe { decode_in_slice_unchecked(encoded, slice) }
}

/// Decodes a G60 encoding string to a list of bytes.
/// The result is writen in `writer`.
///
/// # Errors
/// An error will be arise in the following cases:
/// - if `encoded` is not a valid G60 encoded string.
/// - if the writing process fails.
pub fn decode_in_writer<T: Write>(encoded: &str, slice: &mut T) -> Result<usize, DecodingError> {
    verify(encoded)?;

    unsafe { decode_in_writer_unchecked(encoded, slice) }
}

/// Decodes a G60 encoding string to a list of bytes without checking for its validity.
///
/// # Safety
/// This method will panic or return an undefined value if `encoded` is not a valid G60 string.
pub unsafe fn decode_unchecked(encoded: &str) -> Vec<u8> {
    let mut slice = vec![0; compute_decoded_size(encoded.len())];

    decode_in_slice_unchecked(encoded, &mut slice).unwrap();

    slice
}

/// Decodes a G60 encoding string to a list of bytes without checking for its validity.
/// The result is placed into `slice` and returns the number of elements written.
///
/// # Safety
/// This method will panic or return an undefined value if `encoded` is not a valid G60 string.
///
/// # Errors
/// An error will be arise if `slice` does not have at least `ceil(8 * encoded.len() / 11)` of size.
pub unsafe fn decode_in_slice_unchecked(
    encoded: &str,
    slice: &mut [u8],
) -> Result<usize, DecodingError> {
    let bytes = encoded.as_bytes();
    let required_slice_size = compute_decoded_size(bytes.len());

    if slice.len() < required_slice_size {
        return Err(DecodingError::NotEnoughSpaceInSlice {
            actual: slice.len(),
            required: required_slice_size,
        });
    }

    decode_in_writer_unchecked(encoded, &mut std::io::Cursor::new(slice))
}

/// Decodes a G60 encoding string to a list of bytes without checking for its validity.
/// The result is writen in `writer`.
///
/// # Safety
/// This method will panic or return an undefined value if `encoded` is not a valid G60 string.
///
/// # Errors
/// An error will be arise if the writing process fails.
pub unsafe fn decode_in_writer_unchecked<T: Write>(
    encoded: &str,
    writer: &mut T,
) -> Result<usize, DecodingError> {
    let bytes = encoded.as_bytes();
    let required_slice_size = compute_decoded_size(bytes.len());

    // Complete groups.
    for chunk in bytes.chunks_exact(11) {
        let decoded = compute_chunk(chunk);

        writer.write_all(&decoded).unwrap();
    }

    // Last incomplete group.
    let last_group_length = bytes.len() - (bytes.len() / 11 * 11);
    if last_group_length != 0 {
        let chunk = &bytes[bytes.len() - last_group_length..];
        let decoded = compute_final_chunk(chunk);
        let elements_to_write = compute_decoded_size(last_group_length);

        writer.write_all(&decoded[..elements_to_write]).unwrap();
    }

    Ok(required_slice_size)
}

// ----------------------------------------------------------------------------
// AUX METHODS ----------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Computes `ceil(8 * encoded_length / 11)` faster using only integers.
pub(crate) fn compute_decoded_size(encoded_length: usize) -> usize {
    (encoded_length << 3) / 11
}

#[inline]
fn compute_chunk(chunk: &[u8]) -> [u8; 8] {
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

    [
        b1 as u8,
        (r1 * 20 + b2) as u8,
        (r2 * 90 + b3_bis) as u8,
        (128 * r3_bis + b4) as u8,
        (r4 * 30 + b5) as u8,
        (r5 * 150 + b6) as u8,
        (r6 * 12 + b7) as u8,
        (60 * r7 + c10) as u8,
    ]
}

#[inline]
fn compute_final_chunk(chunk: &[u8]) -> [u8; 8] {
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

    [
        b1 as u8,
        (r1 * 20 + b2) as u8,
        (r2 * 90 + b3_bis) as u8,
        (128 * r3_bis + b4) as u8,
        (r4 * 30 + b5) as u8,
        (r5 * 150 + b6) as u8,
        (r6 * 12 + b7) as u8,
        (60 * r7 + c10) as u8,
    ]
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::VerificationError;

    use super::*;

    #[test]
    fn test_compute_decoded_size() {
        for encoded_length in 0usize..100 {
            let real_value = (8.0 * encoded_length as f64 / 11.0).floor() as usize;
            let computed_value = compute_decoded_size(encoded_length);

            assert_eq!(
                computed_value, real_value,
                "Incorrect for {}",
                encoded_length
            );
        }
    }

    /// This will test also `decode`, `decode_in_slice` and `decode_in_slice_unchecked`.
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

    /// This will test also `decode`, `decode_in_slice` and `decode_in_slice_unchecked`.
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

    /// This will test also `decode_in_writer_unchecked`.
    #[test]
    fn test_decode_in_writer() {
        let test = "Gt4CGFiHehzRzjCF16";
        let mut result_vector = Vec::new();
        let decoded_chars =
            decode_in_writer(test, &mut result_vector).expect("The decoding must succeed");

        let result = b"Hello, world!".to_vec();

        assert_eq!(decoded_chars, 13, "Incorrect chars");
        assert_eq!(result_vector, result, "Incorrect slice result");
    }

    /// This will test also `decode_in_slice_unchecked` and `decode_in_writer_unchecked`.
    #[test]
    fn test_decode_in_slice_exact_slice() {
        let test = "Gt4CGFiHehzRzjCF16";
        let mut result_slice = vec![0; 13];
        let decoded_chars =
            decode_in_slice(test, &mut result_slice).expect("The decoding must succeed");

        let result = b"Hello, world!".to_vec();

        assert_eq!(decoded_chars, 13, "Incorrect chars");
        assert_eq!(result_slice, result, "Incorrect slice result");
    }

    /// This will test also `decode_in_slice` and `decode_in_slice_unchecked`.
    #[test]
    fn test_decode_in_slice_bigger_slice() {
        let test = "Gt4CGFiHehzRzjCF16";
        let mut result_slice = vec![0; 15];
        let decoded_chars =
            decode_in_slice(test, &mut result_slice).expect("The decoding must succeed");

        let mut result = b"Hello, world!".to_vec();
        result.push(0);
        result.push(0);

        assert_eq!(decoded_chars, 13, "Incorrect chars");
        assert_eq!(result_slice, result, "Incorrect slice result");
    }

    /// This will test also `decode_in_slice` and `decode_in_slice_unchecked`.
    #[test]
    fn test_decode_in_slice_shorter_slice() {
        let test = "Gt4CGFiHehzRzjCF16";
        let mut result_slice = vec![0; 10];
        let error =
            decode_in_slice(test, &mut result_slice).expect_err("The decoding cannot succeed");

        assert_eq!(
            error,
            DecodingError::NotEnoughSpaceInSlice {
                actual: 10,
                required: 13,
            },
            "Incorrect for '{}'",
            test
        );
    }
}
