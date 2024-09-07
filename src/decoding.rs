use std::io::Write;

use crate::constants::UTF8_TO_ENCODED_MAP;
use crate::errors::{DecodingError, VerificationError};
use crate::utils::div_rem;

/// Decodes a G60 encoded string.
pub fn decode(encoded: &str) -> Result<Vec<u8>, DecodingError> {
    let mut slice = vec![0; compute_decoded_size(encoded.len())];

    decode_in_slice(encoded, &mut slice)?;

    Ok(slice)
}

/// Decodes a G60 encoded string.
/// The result is placed into `slice` and returns the number of elements written.
///
/// # Errors
/// An error will be thrown if `slice` does not have enough space to store the decoded string.
pub fn decode_in_slice(encoded: &str, slice: &mut [u8]) -> Result<usize, DecodingError> {
    let bytes = encoded.as_bytes();
    let required_slice_size = compute_decoded_size(bytes.len());

    if slice.len() < required_slice_size {
        return Err(DecodingError::NotEnoughSpaceInSlice {
            actual: slice.len(),
            required: required_slice_size,
        });
    }

    decode_in_writer(encoded, &mut std::io::Cursor::new(slice))
}

/// Decodes a G60 encoded string.
/// The result is written in `writer`.
///
/// # Errors
/// An error will be thrown if the writing process fails.
pub fn decode_in_writer<T: Write>(encoded: &str, writer: &mut T) -> Result<usize, DecodingError> {
    let bytes = encoded.as_bytes();
    let required_slice_size = compute_decoded_size(bytes.len());

    // Check length.
    let last_group_length = bytes.len() - bytes.len() / 11 * 11;
    if let 1 | 4 | 8 = last_group_length {
        return Err(DecodingError::Verification(
            VerificationError::InvalidLength,
        ));
    }

    // Complete groups.
    let mut chunk_index = 0;
    for chunk in bytes.chunks_exact(11) {
        let decoded = compute_chunk(chunk_index, chunk)?;

        writer.write_all(&decoded).unwrap();
        chunk_index += 11;
    }

    // Last incomplete group.
    if last_group_length != 0 {
        let chunk = &bytes[bytes.len() - last_group_length..];
        let decoded = compute_chunk(chunk_index, chunk)?;
        let elements_to_write = compute_decoded_size(last_group_length);

        if decoded[elements_to_write..].iter().any(|v| *v != 0) {
            return Err(DecodingError::Verification(VerificationError::NotCanonical));
        }

        writer.write_all(&decoded[..elements_to_write]).unwrap();
    }

    Ok(required_slice_size)
}

// ----------------------------------------------------------------------------
// AUX METHODS ----------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Computes `ceil(8 * encoded_length / 11)` faster using only integers.
#[inline(always)]
pub(crate) fn compute_decoded_size(encoded_length: usize) -> usize {
    (encoded_length << 3) / 11
}

#[inline]
pub(crate) fn map_utf8_to_encoded(
    chunk_index: usize,
    index: usize,
    chunk: &[u8],
) -> Result<usize, VerificationError> {
    match chunk.get(index) {
        Some(v) => {
            let encoded = *UTF8_TO_ENCODED_MAP.get(*v as usize).unwrap_or(&255) as usize;
            if encoded == 255 {
                Err(VerificationError::InvalidByte {
                    index: chunk_index + index,
                    byte: *v,
                })
            } else {
                Ok(encoded)
            }
        }
        None => Ok(0),
    }
}

#[inline]
pub(crate) fn compute_chunk(
    chunk_index: usize,
    chunk: &[u8],
) -> Result<[u8; 8], VerificationError> {
    let c0 = map_utf8_to_encoded(chunk_index, 0, chunk)?;
    let c1 = map_utf8_to_encoded(chunk_index, 1, chunk)?;
    let c2 = map_utf8_to_encoded(chunk_index, 2, chunk)?;
    let c3 = map_utf8_to_encoded(chunk_index, 3, chunk)?;
    let c4 = map_utf8_to_encoded(chunk_index, 4, chunk)?;
    let c5 = map_utf8_to_encoded(chunk_index, 5, chunk)?;
    let c6 = map_utf8_to_encoded(chunk_index, 6, chunk)?;
    let c7 = map_utf8_to_encoded(chunk_index, 7, chunk)?;
    let c8 = map_utf8_to_encoded(chunk_index, 8, chunk)?;
    let c9 = map_utf8_to_encoded(chunk_index, 9, chunk)?;
    let c10 = map_utf8_to_encoded(chunk_index, 10, chunk)?;

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

    let c_a = u8::try_from(b1).map_err(|_| VerificationError::NotCanonical)?;
    let c_b = u8::try_from(r1 * 20 + b2).map_err(|_| VerificationError::NotCanonical)?;
    let c_c = u8::try_from(r2 * 90 + b3_bis).map_err(|_| VerificationError::NotCanonical)?;
    let c_d = u8::try_from(128 * r3_bis + b4).map_err(|_| VerificationError::NotCanonical)?;
    let c_e = u8::try_from(r4 * 30 + b5).map_err(|_| VerificationError::NotCanonical)?;
    let c_f = u8::try_from(r5 * 150 + b6).map_err(|_| VerificationError::NotCanonical)?;
    let c_g = u8::try_from(r6 * 12 + b7).map_err(|_| VerificationError::NotCanonical)?;
    let c_h = u8::try_from(60 * r7 + c10).map_err(|_| VerificationError::NotCanonical)?;

    Ok([c_a, c_b, c_c, c_d, c_e, c_f, c_g, c_h])
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::ENCODED_TO_UTF8_MAP;
    use crate::encode;
    use std::collections::HashSet;

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

    #[test]
    fn test_decoded_correct_values() {
        for length in 0..16 {
            for byte in 0..=255 {
                let bytes = vec![byte; length];
                let encoded = encode(&bytes);
                let decoded = decode(&encoded).expect("The decoding must succeed");

                assert_eq!(bytes, decoded, "Incorrect for length {length}, byte {byte}",);
            }
        }
    }

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

    /// This test checks that the decoding only works with canonical values.
    #[test]
    fn test_do_not_decode_non_canonical_values() {
        let mut ok_values = HashSet::new();

        for i in ENCODED_TO_UTF8_MAP {
            for j in ENCODED_TO_UTF8_MAP {
                let encoded = format!("{}{}", *i as char, *j as char);
                let decoded = match decode(&encoded) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let encoded_2 = encode(&decoded);

                assert_eq!(encoded, encoded_2, "[Incorrect]\n - Encoded  : {encoded} - {:?}\n - Encoded 2: {encoded_2} - {:?}\n - Decoded: {:?}",
                           encoded.as_bytes(),
                           encoded_2.as_bytes(),
                           decoded);

                ok_values.insert(encoded);
            }
        }

        for i in 0..=255 {
            let decoded = vec![i];
            let encoded = encode(&decoded);

            if encoded.len() == 2 {
                assert!(
                    ok_values.remove(&encoded),
                    "Not found encoding for {:?}. Encoded: {encoded}",
                    decoded
                );
            }

            for j in 0..=255 {
                let decoded = vec![i, j];
                let encoded = encode(&decoded);

                if encoded.len() == 2 {
                    assert!(
                        ok_values.remove(&encoded),
                        "Not found encoding for {:?}. Encoded: {encoded}",
                        decoded
                    );
                }
            }
        }

        assert!(
            ok_values.is_empty(),
            "Not found encoding for {:?}",
            ok_values
        );
    }
}
