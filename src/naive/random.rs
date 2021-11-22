use crate::constants::ENCODED_TO_UTF8_MAP;
use rand::{
    rngs::{SmallRng, StdRng},
    Rng, SeedableRng,
};

/// Generates a random G60 string of exactly `length` characters using a non-deterministic PRNG.
/// Note: depending on the length it can result in a valid G60 encoded string or not.
pub fn exactly_random(length: usize) -> String {
    let mut rng = StdRng::from_entropy();

    exactly_custom_random(length, |v| rng.fill(v))
}

/// Generates a random G60 string of exactly `length` characters using a basic but faster random generator.
/// Note: depending on the length it can result in a valid G60 encoded string or not.
pub fn exactly_unsecure_random(length: usize) -> String {
    let mut rng = SmallRng::from_entropy();

    exactly_custom_random(length, |v| rng.fill(v))
}

/// Generates a random G60 string of exactly `length` characters using a custom random generator.
/// Note: depending on the length it can result in a valid G60 encoded string or not.
pub fn exactly_custom_random<F>(length: usize, mut rng: F) -> String
where
    F: FnMut(&mut [u8]),
{
    let mut result = Vec::with_capacity(length);

    let mut chunk = [0; 11];
    'out: while result.len() != result.capacity() {
        rng(&mut chunk);

        for byte in &chunk {
            match ENCODED_TO_UTF8_MAP.get(*byte as usize) {
                Some(v) => result.push(*v),
                None => continue,
            }

            if result.len() == result.capacity() {
                break 'out;
            }
        }
    }

    unsafe { String::from_utf8_unchecked(result) }
}

// ----------------------------------------------------------------------------
// TESTS ----------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::naive::NaiveG60String;

    #[test]
    fn test_exactly_random_valid() {
        for initial_length in [0usize, 2, 3, 5, 6, 7, 9, 10, 11] {
            for multiplier in 0..10 {
                let length = initial_length + multiplier * 11;
                let random = exactly_random(length);
                assert_eq!(random.len(), length, "Length is incorrect");

                assert!(
                    !NaiveG60String::verify(random.as_str()).expect("Verification fails"),
                    "The verification is incorrect"
                );
            }
        }
    }

    #[test]
    fn test_exactly_random_invalid() {
        for length in [1usize, 4, 8] {
            for multiplier in 0..10 {
                let length = length + multiplier * 11;
                let random = exactly_random(length);
                assert_eq!(random.len(), length, "Length is incorrect");

                assert!(
                    NaiveG60String::verify(random.as_str()).expect("Verification fails"),
                    "The verification is incorrect"
                );
            }
        }
    }
}
