//! Generates Random Numbers for use in Prime Number Choosing
use crate::err::ErrorKind;
use super::*;

use rand::rngs::EntropyRng;
use rand::Rng;
use num_bigint::BigUint;

use failure::{/*ResultExt,*/ Error};

/// A Number generator that creates random numbers through collecting entropy on the Operating System
/// First, tries to collect entropy from operations occuring on the Operating System
/// If that fails to generate enough entropy, then this will fallback to generating entropy from
/// "System Jitters" (Random number generator based on jitter in the CPU execution time, and jitter in memory access time. This is significantly slower than OS operations).
/// For more information on random number gens, take a gander at rand::rngs::EntropyRng
#[derive(Default)]
struct NumberGenerator {
    /// Total Size of Public key (n = pq) where n is the public key
    size: usize,
    /// Library being used for Random Number Generation
    generator: EntropyRng
}

impl NumberGenerator {

    /// Instantiate a new NumberGenerator
    fn new(size: usize) -> Result<Self, Error> {
        // must be larger than 512 bits and a power of 2
        if size < MINIMUM_KEY_LENGTH || !((size & (size - 1 )) == 0) {
            Err(ErrorKind::InvalidKeyLength)?
        }
        Ok(NumberGenerator {
            size: size,
            generator: EntropyRng::new()
        })
    }
}

// returns number of u8 vector elements corresponds to one bit-size for one of p or q
// EX: a u32 vector with 3 elements is 96 bits in size
fn bit_size(size: usize) -> usize {
    (size / 2 ) / 32
}

/// An Iterator which spits out a new random number (based on rand::rng::EntropyRng) every iteration
impl Iterator for NumberGenerator {
    type Item = BigUint;

    fn next(&mut self) -> Option<BigUint> {
        let mut number = vec![0; bit_size(self.size)];
        let len = number.len();
        self.generator.fill(number.as_mut_slice());
        number[0] |= 1 << 0; // set LSB to 1 (so it is odd)
        number[len - 1] |= 1 << 31; // set MSB to 1 (so we know it is exactly the length specified)

        Some(BigUint::from_slice(number.as_slice()))
    }
}


pub struct Primes {
    /// P
    p: BigUint,
    /// Q
    q: BigUint,
    /// The Size of both P and Q. This size (in bits) is guaranteed to be the same for both P and Q
    size: u64
}

/// Finds primes based on the NumberGenerator
pub struct PrimeFinder {

    fn fermat(&self) {

    }

    fn rabin_miller(&self) {

    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic]
    fn should_not_create_numbers_with_less_than_512bits() {
        let gen = NumberGenerator::new(32).unwrap();
        let numbers = gen.take(4).for_each(|x| {
            println!("Number: {:#}", x);
        });
    }

    #[test]
    #[should_panic]
    fn should_fail_if_keylength_not_power_of_two() {
        NumberGenerator::new(31).unwrap();
    }

    #[test]
    fn should_generate_random_numbers() {
        let gen = NumberGenerator::new(512).unwrap();
        let numbers = gen.take(10).collect::<Vec<BigUint>>();
        for i in 0..10 {
            for j in 0..10 {
                if i != j {
                    assert!(numbers[i] != numbers[j]);
                }
            }
        }
    }
}
