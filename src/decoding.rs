use std::io::Write;

use crate::constants::CHAR_TO_G60;
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

    let last_group_length = bytes.len() - bytes.len() / 11 * 11;
    if let 1 | 4 | 8 = last_group_length {
        return Err(DecodingError::Verification(
            VerificationError::InvalidLength,
        ));
    }

    let mut pos = 0;
    let mut chunk_index = 0;

    for chunk in bytes.chunks_exact(11) {
        let decoded = compute_chunk(chunk_index, chunk)?;
        slice[pos..pos + 8].copy_from_slice(&decoded);
        pos += 8;
        chunk_index += 11;
    }

    if last_group_length != 0 {
        let chunk = &bytes[bytes.len() - last_group_length..];
        let decoded = compute_chunk(chunk_index, chunk)?;
        let elements_to_write = compute_decoded_size(last_group_length);

        check_canonical_tail(&decoded, elements_to_write)?;

        slice[pos..pos + elements_to_write].copy_from_slice(&decoded[..elements_to_write]);
    }

    Ok(required_slice_size)
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

        writer.write_all(&decoded)?;
        chunk_index += 11;
    }

    // Last incomplete group.
    if last_group_length != 0 {
        let chunk = &bytes[bytes.len() - last_group_length..];
        let decoded = compute_chunk(chunk_index, chunk)?;
        let elements_to_write = compute_decoded_size(last_group_length);

        check_canonical_tail(&decoded, elements_to_write)?;

        writer.write_all(&decoded[..elements_to_write])?;
    }

    Ok(required_slice_size)
}

// ----------------------------------------------------------------------------
// AUX METHODS ----------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Computes `floor(8 * encoded_length / 11)` faster using only integers.
#[inline(always)]
pub(crate) fn compute_decoded_size(encoded_length: usize) -> usize {
    (encoded_length << 3) / 11
}

/// Returns an error if the padding bytes of a partial decoded chunk are non-zero,
/// which would indicate the encoded string is not the canonical representation.
#[inline]
pub(crate) fn check_canonical_tail(
    decoded: &[u8; 8],
    elements_to_write: usize,
) -> Result<(), VerificationError> {
    if decoded[elements_to_write..].iter().any(|v| *v != 0) {
        Err(VerificationError::NotCanonical)
    } else {
        Ok(())
    }
}

#[inline]
pub(crate) fn map_char_to_g60_index(
    chunk_index: usize,
    index: usize,
    chunk: &[u8],
) -> Result<usize, VerificationError> {
    match chunk.get(index) {
        Some(&v) => {
            let value = CHAR_TO_G60[v as usize] as usize;
            if value == 255 {
                Err(VerificationError::InvalidByte {
                    index: chunk_index + index,
                    byte: v,
                })
            } else {
                Ok(value)
            }
        }
        None => Ok(0),
    }
}

#[inline(always)]
fn to_u8(value: usize) -> Result<u8, VerificationError> {
    if value > 255 {
        Err(VerificationError::NotCanonical)
    } else {
        Ok(value as u8)
    }
}

#[inline]
pub(crate) fn compute_chunk(
    chunk_index: usize,
    chunk: &[u8],
) -> Result<[u8; 8], VerificationError> {
    let c0 = map_char_to_g60_index(chunk_index, 0, chunk)?;
    let c1 = map_char_to_g60_index(chunk_index, 1, chunk)?;
    let c2 = map_char_to_g60_index(chunk_index, 2, chunk)?;
    let c3 = map_char_to_g60_index(chunk_index, 3, chunk)?;
    let c4 = map_char_to_g60_index(chunk_index, 4, chunk)?;
    let c5 = map_char_to_g60_index(chunk_index, 5, chunk)?;
    let c6 = map_char_to_g60_index(chunk_index, 6, chunk)?;
    let c7 = map_char_to_g60_index(chunk_index, 7, chunk)?;
    let c8 = map_char_to_g60_index(chunk_index, 8, chunk)?;
    let c9 = map_char_to_g60_index(chunk_index, 9, chunk)?;
    let c10 = map_char_to_g60_index(chunk_index, 10, chunk)?;

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

    Ok([
        to_u8(b1)?,
        to_u8(r1 * 20 + b2)?,
        to_u8(r2 * 90 + b3_bis)?,
        to_u8(128 * r3_bis + b4)?,
        to_u8(r4 * 30 + b5)?,
        to_u8(r5 * 150 + b6)?,
        to_u8(r6 * 12 + b7)?,
        to_u8(60 * r7 + c10)?,
    ])
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::G60_TO_CHAR;
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

        for i in G60_TO_CHAR {
            for j in G60_TO_CHAR {
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
