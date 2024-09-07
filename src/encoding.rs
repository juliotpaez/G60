use std::io::Write;

use crate::constants::ENCODED_TO_UTF8_MAP;
use crate::errors::EncodingError;
use crate::utils::div_rem;

/// Encodes a list of bytes into a G60 encoding format.
pub fn encode(content: &[u8]) -> String {
    let mut slice = Vec::with_capacity(compute_encoded_size(content.len()));

    encode_in_writer(content, &mut slice).unwrap();

    unsafe { String::from_utf8_unchecked(slice) }
}

/// Encodes a list of bytes into a G60 encoding format.
/// The result is placed into `slice` and returns the number of elements written.
///
/// # Errors
/// An error will be thrown if `slice` does not have enough space to store the encoded string.
pub fn encode_in_slice(content: &[u8], slice: &mut [u8]) -> Result<usize, EncodingError> {
    let required_slice_size = compute_encoded_size(content.len());

    if slice.len() < required_slice_size {
        return Err(EncodingError::NotEnoughSpaceInSlice {
            actual: slice.len(),
            required: required_slice_size,
        });
    }

    encode_in_writer(content, &mut std::io::Cursor::new(slice))
}

/// Encodes a list of bytes into a G60 encoding format.
/// The result is written in `writer`.
///
/// # Errors
/// An error will be thrown if the writing process fails.
pub fn encode_in_writer<T: Write>(content: &[u8], writer: &mut T) -> Result<usize, EncodingError> {
    let required_slice_size = compute_encoded_size(content.len());

    // Complete groups.
    for chunk in content.chunks_exact(8) {
        let encoded = compute_chunk(chunk);

        writer.write_all(&encoded)?;
    }

    // Last incomplete group.
    let last_group_length = content.len() - (content.len() >> 3 << 3);
    if last_group_length != 0 {
        let chunk = &content[content.len() - last_group_length..];
        let encoded = compute_chunk(chunk);
        let elements_to_write = compute_encoded_size(last_group_length);

        writer.write_all(&encoded[..elements_to_write])?;
    }

    Ok(required_slice_size)
}

// ----------------------------------------------------------------------------
// AUX METHODS ----------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Computes `ceil(11 * content_length / 8)` faster using only integers.
#[inline(always)]
pub(crate) fn compute_encoded_size(content_length: usize) -> usize {
    (11 * content_length + 7) >> 3
}

#[inline]
pub(crate) fn compute_chunk(chunk: &[u8]) -> [u8; 11] {
    let c_a = chunk[0] as usize;
    let c_b = *chunk.get(1).unwrap_or(&0) as usize;
    let c_c = *chunk.get(2).unwrap_or(&0) as usize;
    let c_d = *chunk.get(3).unwrap_or(&0) as usize;
    let c_e = *chunk.get(4).unwrap_or(&0) as usize;
    let c_f = *chunk.get(5).unwrap_or(&0) as usize;
    let c_g = *chunk.get(6).unwrap_or(&0) as usize;
    let c_h = *chunk.get(7).unwrap_or(&0) as usize;

    let (c2, r2) = div_rem(c_b, 20);
    let (c1, r1) = div_rem(14 * c_a + c2, 60);
    let (c3, r3) = div_rem(c_c, 90);
    let b3h = c_d >> 7;
    let b3l = c_d & 0x7F;
    let (c4, r4) = div_rem((r3 << 1) + b3h, 3);
    let (c6, r6) = div_rem(c_e, 30);
    let (c5, r5) = div_rem(9 * b3l + c6, 60);
    let (c7, r7) = div_rem(c_f, 150);
    let (c8a, r8a) = div_rem(c_g, 144);
    let (c8, r8) = div_rem((r7 << 1) + c8a, 5);
    let (c9, r9) = div_rem(r8a, 12);
    let (c10, r10) = div_rem(c_h, 60);

    [
        ENCODED_TO_UTF8_MAP[c1],
        ENCODED_TO_UTF8_MAP[r1],
        ENCODED_TO_UTF8_MAP[3 * r2 + c3],
        ENCODED_TO_UTF8_MAP[c4],
        ENCODED_TO_UTF8_MAP[20 * r4 + c5],
        ENCODED_TO_UTF8_MAP[r5],
        ENCODED_TO_UTF8_MAP[(r6 << 1) + c7],
        ENCODED_TO_UTF8_MAP[c8],
        ENCODED_TO_UTF8_MAP[12 * r8 + c9],
        ENCODED_TO_UTF8_MAP[5 * r9 + c10],
        ENCODED_TO_UTF8_MAP[r10],
    ]
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_encoded_size() {
        for content_length in 0usize..100 {
            let real_value = (11.0 * content_length as f64 / 8.0).ceil() as usize;
            let computed_value = compute_encoded_size(content_length);

            assert_eq!(
                computed_value, real_value,
                "Incorrect for {}",
                content_length
            );
        }
    }

    #[test]
    fn test_encode_in_writer() {
        let test = "Hello, world!";
        let mut result_vector = Vec::new();
        let encoded_chars = encode_in_writer(test.as_bytes(), &mut result_vector)
            .expect("The encoding must succeed");

        let result = b"Gt4CGFiHehzRzjCF16".to_vec();

        assert_eq!(encoded_chars, 18, "Incorrect chars");
        assert_eq!(result_vector, result, "Incorrect slice result");
    }

    #[test]
    fn test_encode_in_slice_exact_slice() {
        let test = "Hello, world!";
        let mut result_slice = vec![0; 18];
        let encoded_chars =
            encode_in_slice(test.as_bytes(), &mut result_slice).expect("The encoding must succeed");

        let result = b"Gt4CGFiHehzRzjCF16".to_vec();

        assert_eq!(encoded_chars, 18, "Incorrect chars");
        assert_eq!(result_slice, result, "Incorrect slice result");
    }

    #[test]
    fn test_encode_in_slice_bigger_slice() {
        let test = "Hello, world!";
        let mut result_slice = vec![0; 20];
        let encoded_chars =
            encode_in_slice(test.as_bytes(), &mut result_slice).expect("The encoding must succeed");

        let mut result = b"Gt4CGFiHehzRzjCF16".to_vec();
        result.push(0);
        result.push(0);

        assert_eq!(encoded_chars, 18, "Incorrect chars");
        assert_eq!(result_slice, result, "Incorrect slice result");
    }

    #[test]
    fn test_encode_in_slice_shorter_slice() {
        let test = "Hello, world!";
        let mut result_slice = vec![0; 15];
        let error = encode_in_slice(test.as_bytes(), &mut result_slice)
            .expect_err("The encoding cannot succeed");

        assert_eq!(
            error,
            EncodingError::NotEnoughSpaceInSlice {
                actual: 15,
                required: 18,
            },
            "Incorrect for '{}'",
            test
        );
    }

    /// This test checks that the encoding is monotonic. This means that the encoding of a number
    /// is always greater than the encoding of the previous number.
    #[test]
    fn test_monotonic_encoding() {
        // One byte.
        {
            let mut prev = 0;

            for current in 1..=255 {
                let prev_encoded = encode(&[prev]);
                let current_encoded = encode(&[current]);

                assert!(
                    prev_encoded < current_encoded,
                    "Monotonicity failed for 1-byte {}",
                    current
                );
                prev = current;
            }
        }

        // Two bytes.
        {
            let mut prev = 0;

            for current in 1..=255 {
                for prev_second in 0..=255 {
                    for current_second in 0..=255 {
                        let prev_encoded = encode(&[prev, prev_second]);
                        let current_encoded = encode(&[current, current_second]);

                        assert!(
                            prev_encoded < current_encoded,
                            "Monotonicity failed for 2-bytes {}: Second Prev: {}, Second Current: {}",
                            current,
                            prev_second,
                            current_second
                        );
                    }
                }

                prev = current;
            }
        }

        // Nine bytes. (second chunk)
        {
            let mut prev = 0;

            for current in 1..=255 {
                for prev_second in 0..=255 {
                    for current_second in 0..=255 {
                        let prev_encoded = encode(&[prev, 0, 0, 0, 0, 0, 0, 0, prev_second]);
                        let current_encoded =
                            encode(&[current, 0, 0, 0, 0, 0, 0, 0, current_second]);

                        assert!(
                            prev_encoded < current_encoded,
                            "Monotonicity failed for 9-bytes {}: Second Prev: {}, Second Current: {}",
                            current,
                            prev_second,
                            current_second
                        );
                    }
                }

                prev = current;
            }
        }
    }
}
