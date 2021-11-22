use crate::{decoding, encoding};
use rand::rngs::{SmallRng, StdRng};
use rand::{Rng, SeedableRng};

/// Generates a random G60 string of `bytes` length using a non-deterministic PRNG.
pub fn random_bytes(bytes: usize) -> String {
    let mut rng = StdRng::from_entropy();

    custom_random_bytes(bytes, |v| rng.fill(v))
}

/// Generates a random G60 string of `bytes` length using a basic but faster random generator.
pub fn unsecure_random_bytes(bytes: usize) -> String {
    let mut rng = SmallRng::from_entropy();

    custom_random_bytes(bytes, |v| rng.fill(v))
}

/// Generates a random G60 string of `bytes` length using a custom random generator.
pub fn custom_random_bytes<F>(bytes: usize, mut rng: F) -> String
where
    F: FnMut(&mut [u8]),
{
    let required_slice_size = encoding::compute_encoded_size(bytes);
    let mut result = Vec::with_capacity(required_slice_size);

    // Complete groups.
    let mut chunk = [0; 11];
    for _ in 0..(bytes >> 3) {
        rng(&mut chunk);

        let encoded = encoding::compute_chunk(&chunk);

        result.extend_from_slice(&encoded);
    }

    // Last incomplete group.
    let last_group_length = bytes - (bytes >> 3 << 3);
    if last_group_length != 0 {
        rng(&mut chunk);

        let chunk = &chunk[11 - last_group_length..];
        let encoded = encoding::compute_chunk(chunk);
        let elements_to_write = encoding::compute_encoded_size(last_group_length);

        result.extend_from_slice(&encoded[..elements_to_write]);
    }

    unsafe { String::from_utf8_unchecked(result) }
}

/// Generates a random G60 string of at most `length` characters using a non-deterministic PRNG.
pub fn random_str(mut length: usize) -> String {
    // Handle incorrect lengths.
    let remaining_bytes = length - length / 11 * 11;
    if let 1 | 4 | 8 = remaining_bytes {
        length -= 1;
    }

    let bytes = decoding::compute_decoded_size(length);

    random_bytes(bytes)
}

/// Generates a random G60 string of at most `length` characters using a basic but faster random generator.
pub fn unsecure_random_str(mut length: usize) -> String {
    // Handle incorrect lengths.
    let remaining_bytes = length - length / 11 * 11;
    if let 1 | 4 | 8 = remaining_bytes {
        length -= 1;
    }

    let bytes = decoding::compute_decoded_size(length);

    unsecure_random_bytes(bytes)
}

/// Generates a random G60 string of at most `length` characters using a custom random generator.
pub fn custom_random_str<F>(mut length: usize, rng: F) -> String
where
    F: FnMut(&mut [u8]),
{
    // Handle incorrect lengths.
    let remaining_bytes = length - length / 11 * 11;
    if let 1 | 4 | 8 = remaining_bytes {
        length -= 1;
    }

    let bytes = decoding::compute_decoded_size(length);

    custom_random_str(bytes, rng)
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::encoding::encode;
    use crate::verification::verify;

    use super::*;

    #[test]
    fn test_random() {
        for bytes in 0usize..100 {
            let random = random_bytes(bytes);

            verify(random.as_str()).expect("Verification fails");

            let decoded = unsafe { decoding::decode_unchecked(random.as_str()) };

            assert_eq!(decoded.len(), bytes, "Length is incorrect");

            let encoded = encode(&decoded);

            assert_eq!(encoded, random, "Encoding and random are different")
        }
    }

    #[test]
    fn test_random_str_same_length() {
        for length in [0usize, 2, 3, 5, 6, 7, 9, 10, 11] {
            for multiplier in 0..10 {
                let length = length + multiplier * 11;
                let random = random_str(length);
                assert_eq!(random.len(), length, "Length is incorrect");

                verify(random.as_str()).expect("Verification fails");

                let decoded = unsafe { decoding::decode_unchecked(random.as_str()) };
                let encoded = encode(&decoded);

                assert_eq!(encoded, random, "Encoding and random are different")
            }
        }
    }

    #[test]
    fn test_random_str_incorrect_length() {
        for length in [1usize, 4, 8] {
            for multiplier in 0..10 {
                let length = length + multiplier * 11;
                let random = random_str(length);
                assert_eq!(random.len(), length - 1, "Length is incorrect");

                verify(random.as_str()).expect("Verification fails");

                let decoded = unsafe { decoding::decode_unchecked(random.as_str()) };
                let encoded = encode(&decoded);

                assert_eq!(encoded, random, "Encoding and random are different")
            }
        }
    }
}
