//! Generates Random Numbers for use in Prime Number Choosing
use crate::err::ErrorKind;
use super::primedb::PrimeDB;
use super::*;

use rand::rngs::EntropyRng;
use rand::Rng;
use num_bigint::{BigUint, ToBigUint};
use num_bigint::RandBigInt;
use num_traits::identities::{One, Zero};

use failure::{/*ResultExt,*/ Error};

/// A Number generator that creates random numbers through collecting entropy on the Operating System
/// First, tries to collect entropy from operations occuring on the Operating System
/// If that fails to generate enough entropy, then this will fallback to generating entropy from
/// "System Jitters" (Random number generator based on jitter in the CPU execution time, and jitter in memory access time.
/// This is significantly slower than OS operations).
/// For more information on random number gens, take a gander at rand::rngs::EntropyRng
#[derive(Default)]
pub struct NumberGenerator {
    /// Total Size of Public key (n = pq) where n is the public key
    size: KeySize,
    /// Library being used for Random Number Generation
    generator: EntropyRng
}

impl NumberGenerator {

    /// Instantiate a new NumberGenerator
    /// "size" corresponds to the size in bits the number must be
    /// Size must be larger than 512 and a power of 2
    pub fn new(size: KeySize) -> Result<Self, Error> {

        // must be larger than 512 bits and a power of 2
        if size.as_num() < MINIMUM_KEY_LENGTH || !((size.as_num() & (size.as_num() - 1 )) == 0) {
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
        let mut number = vec![0; bit_size(self.size.as_num())];
        let len = number.len();
        self.generator.fill(number.as_mut_slice());
        number[0] |= 1; //number[0] |= 1 << 0; // set LSB to 1 (so it is odd)
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

#[derive(Debug, Clone, PartialEq)]
enum ProbableVariant {
    Prime,
    Composite
}

/// Finds primes based on the NumberGeneratorA
/// **WARNING** These functions assume an odd `candidate` input that is greater than 3
impl ProbableVariant {

    // check if a candidate is prime or not by running all tests on it
    pub fn find(candidate: &BigUint) -> Self {

        return match (Self::small_primes(candidate), Self::fermat(candidate), Self::rabin_miller(candidate, 40)) {
            (ProbableVariant::Prime, ProbableVariant::Prime, ProbableVariant::Prime) => ProbableVariant::Prime,
            _ => ProbableVariant::Composite
        };
    }

    /// Check if the candidate is divisible by small primes
    /// Small primes are stored in a JSON file
    pub fn small_primes(candidate: &BigUint) -> Self {
        let primes = PrimeDB::get();
        for (key, prime) in primes.iter() {
            if (candidate % prime) == BigUint::zero() && candidate != &BigUint::from(*prime) {
                return ProbableVariant::Composite;
            }
        }
        ProbableVariant::Prime
    }

    /// If this function returns false, then the candidate is composite
    /// If this function returns true, then the candidate is probably not composite
    pub fn fermat(candidate: &BigUint) -> Self {
        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint_range(&BigUint::one(), &(candidate - BigUint::one()));

        if a.modpow(&(candidate - BigUint::one()), &candidate) == BigUint::one() {
            ProbableVariant::Prime
        } else {
            ProbableVariant::Composite
        }
    }

    /// The Candidate prime number and how many rounds (k) to process or miller-rabin
    /// References: https://stackoverflow.com/questions/6325576/how-many-iterations-of-rabin-miller-should-i-use-for-cryptographic-safe-primes
    /// (How many rounds of miller rabin to use)
    pub fn rabin_miller(candidate: &BigUint, rounds: usize) -> Self {

        let candidate_minus_one = candidate - BigUint::one();
        let mut rng = rand::thread_rng();
        let mut s: usize = 0;
        let mut d: BigUint = candidate_minus_one.clone();

        // sanity check to ensure candidate is not even
        if candidate % 2usize == BigUint::zero() {
            return ProbableVariant::Composite;
        }

        // find a d such that 2^s*d = n - 1
        while (d.clone() % 2usize) == BigUint::zero() {
            s += 1;
            d = d / BigUint::from(2usize);
        }

        for _ in 0..rounds {
            let a = rng.gen_biguint_range(&BigUint::from(2usize), &(candidate - 2usize));
            let mut x = a.modpow(&d, &candidate);
            if x == BigUint::one() || x == candidate_minus_one {
                continue;
            }
            let mut r = 1;
            while r < s {
                x = x.modpow(&BigUint::from(2usize), &candidate);

                if x == BigUint::one() {
                    return ProbableVariant::Composite;
                } else if x == candidate_minus_one {
                    break;
                }
                r += 1;
            }
            if r == s {
                return ProbableVariant::Composite;
            }
        }
        ProbableVariant::Prime
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_generate_random_numbers() {
        let gen = NumberGenerator::new(KeySize::FiveTwelve).unwrap();
        let numbers = gen.take(10).collect::<Vec<BigUint>>();
        for i in 0..10 {
            for j in 0..10 {
                if i != j {
                    assert!(numbers[i] != numbers[j]);
                }
            }
        }
    }

    #[test]
    fn should_recognize_composite_numbers() {
        let num = 20usize.to_biguint().unwrap();
        assert!(ProbableVariant::fermat(&num) == ProbableVariant::Composite);
        assert!(ProbableVariant::rabin_miller(&num, 40) == ProbableVariant::Composite);
        let num = 2695usize.to_biguint().unwrap();
        assert!(ProbableVariant::fermat(&num) == ProbableVariant::Composite);
        assert!(ProbableVariant::rabin_miller(&num, 40) == ProbableVariant::Composite);
    }

    #[test]
    fn should_recognize_possibly_prime() {
        let num = 1847usize.to_biguint().unwrap();
        assert!(ProbableVariant::fermat(&num) == ProbableVariant::Prime);
        assert!(ProbableVariant::rabin_miller(&num, 40) == ProbableVariant::Prime);
        let num = 2693usize.to_biguint().unwrap();
        assert!(ProbableVariant::fermat(&num) == ProbableVariant::Prime);
        assert!(ProbableVariant::rabin_miller(&num, 40) == ProbableVariant::Prime);
    }

    #[test]
    fn should_test_small_primes() {
        let num = 1847usize.to_biguint().unwrap();
        assert!(ProbableVariant::small_primes(&num) == ProbableVariant::Prime);
        let num = 20usize.to_biguint().unwrap();
        assert!(ProbableVariant::small_primes(&num) == ProbableVariant::Composite);
    }
}
