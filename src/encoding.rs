use std::io::Write;

use crate::constants::ENCODED_TO_UTF8_MAP;
use crate::utils::div_rem;
use crate::EncodingError;

/// Encodes a string into a G60 encoding format.
pub fn encode_str(content: &str) -> String {
    encode(content.as_bytes())
}

/// Encodes a list of bytes into a G60 encoding format.
pub fn encode(content: &[u8]) -> String {
    let mut buffer = vec![0; compute_buffer_size(content.len())];

    encode_in_buffer(content, &mut buffer).unwrap();

    unsafe { String::from_utf8_unchecked(buffer) }
}

/// Encodes a list of bytes into a G60 encoding format.
/// The result is placed into `buffer` and returns the number of elements written.
///
/// # Errors
/// An error will be arise if `buffer` does not have at least `ceil(11 * content.len() / 8)` of size.
pub fn encode_in_buffer(content: &[u8], mut buffer: &mut [u8]) -> Result<usize, EncodingError> {
    let required_buffer_size = compute_buffer_size(content.len());

    if buffer.len() < required_buffer_size {
        return Err(EncodingError::NotEnoughSpaceInBuffer {
            actual: buffer.len(),
            required: required_buffer_size,
        });
    }

    // Complete groups.
    for chunk in content.chunks_exact(8) {
        let c_a = chunk[0] as usize;
        let c_b = chunk[1] as usize;
        let c_c = chunk[2] as usize;
        let c_d = chunk[3] as usize;
        let c_e = chunk[4] as usize;
        let c_f = chunk[5] as usize;
        let c_g = chunk[6] as usize;
        let c_h = chunk[7] as usize;

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

        let encoded = [
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
        ];

        buffer.write_all(&encoded).unwrap();
    }

    // Last incomplete group.
    let last_group_length = content.len() - (content.len() >> 3 << 3);
    if last_group_length != 0 {
        let chunk = &content[content.len() - last_group_length..];

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

        let encoded = [
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
        ];

        let elements_to_write = compute_buffer_size(last_group_length);
        buffer.write_all(&encoded[..elements_to_write]).unwrap();
    }

    Ok(required_buffer_size)
}

// ----------------------------------------------------------------------------
// AUX METHODS ----------------------------------------------------------------
// ----------------------------------------------------------------------------

/// Computes `ceil(11 * content_length / 8)` faster using only integers.
fn compute_buffer_size(content_length: usize) -> usize {
    (11 * content_length + 7) >> 3
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_buffer_size() {
        for content_length in 0usize..100 {
            let real_value = (11.0 * content_length as f64 / 8.0).ceil() as usize;
            let computed_value = compute_buffer_size(content_length);

            assert_eq!(
                computed_value, real_value,
                "Incorrect for {}",
                content_length
            );
        }
    }

    /// This will test also `encode` and `encode_in_buffer`.
    #[test]
    fn test_encode_str() {
        let test = "Hello, world!";
        let encoded = encode_str(test);

        assert_eq!(encoded, "Gt4CGFiHehzRzjCF16", "Incorrect for '{}'", test);

        // --------------------------------------------------------------------

        let test = "Hella, would???";
        let encoded = encode_str(test);

        assert_eq!(encoded, "Gt4CGFEHehzRzsCF26RHF", "Incorrect for '{}'", test);
    }

    #[test]
    fn test_encode_in_buffer_exact_buffer() {
        let test = "Hello, world!";
        let mut result_buffer = vec![0; 18];
        let encoded_chars = encode_in_buffer(test.as_bytes(), &mut result_buffer)
            .expect("The encoding must succeed");

        let result = b"Gt4CGFiHehzRzjCF16".to_vec();

        assert_eq!(encoded_chars, 18, "Incorrect chars");
        assert_eq!(result_buffer, result, "Incorrect buffer result");
    }

    #[test]
    fn test_encode_in_buffer_bigger_buffer() {
        let test = "Hello, world!";
        let mut result_buffer = vec![0; 20];
        let encoded_chars = encode_in_buffer(test.as_bytes(), &mut result_buffer)
            .expect("The encoding must succeed");

        let mut result = b"Gt4CGFiHehzRzjCF16".to_vec();
        result.push(0);
        result.push(0);

        assert_eq!(encoded_chars, 18, "Incorrect chars");
        assert_eq!(result_buffer, result, "Incorrect buffer result");
    }

    #[test]
    fn test_encode_in_buffer_shorter_buffer() {
        let test = "Hello, world!";
        let mut result_buffer = vec![0; 15];
        let error = encode_in_buffer(test.as_bytes(), &mut result_buffer)
            .expect_err("The encoding cannot succeed");

        assert_eq!(
            error,
            EncodingError::NotEnoughSpaceInBuffer {
                actual: 15,
                required: 18,
            },
            "Incorrect for '{}'",
            test
        );
    }
}
