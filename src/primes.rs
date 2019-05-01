mod gen;
mod primedb;

use crate::err::ErrorKind;
use failure::{Error};
use num_bigint::BigUint;
use gen::{NumberGenerator, ProbableVariant};

//TODO Make panic messages better. This program should never panic

// Minimum KeySize is 512
#[derive(Debug, Clone, PartialEq)]
pub enum KeySize {
    TwoFiftySix,
    FiveTwelve,
    TenTwentyFour,
    TwentyFourtyEight,
    FourtyNinetySix,
    EightyOneNinetyTwo,
}

impl KeySize {
    pub fn as_num(&self) -> usize {
        match *self {
            KeySize::TwoFiftySix => 256,
            KeySize::FiveTwelve => 512,
            KeySize::TenTwentyFour => 1024,
            KeySize::TwentyFourtyEight => 2048,
            KeySize::FourtyNinetySix => 4096,
            KeySize::EightyOneNinetyTwo => 8192,
        }
    }

    // Panics if keysize is not at least 512
    pub fn as_half(&self) -> Self {
        match *self {
            KeySize::FiveTwelve => KeySize::TwoFiftySix,
            KeySize::TenTwentyFour => KeySize::FiveTwelve,
            KeySize::TwentyFourtyEight => KeySize::TenTwentyFour,
            KeySize::FourtyNinetySix => KeySize::TwentyFourtyEight,
            KeySize::EightyOneNinetyTwo => KeySize::FourtyNinetySix,
            _ => panic!("Invalid Key Size. Minimum Key Size is 512 bits"),
        }
    }
}

impl From<usize> for KeySize {
    fn from(size: usize) -> KeySize {

        match size {
            512  => KeySize::FiveTwelve,
            1024 => KeySize::TenTwentyFour,
            2048 => KeySize::TwentyFourtyEight,
            4096 => KeySize::FourtyNinetySix,
            8192 => KeySize::EightyOneNinetyTwo,
            _ => panic!("KeySize is too small or too large. Minimum Size: 512, Max Size: 8192")
        }

    }
}

impl Default for KeySize {
    fn default() -> KeySize {
        KeySize::TwentyFourtyEight
    }
}

pub struct PrimeFinder;


// Should make this asynchronous
// TODO: Currently goes in a infinite loop if a prime is never found, NOT GOOD
// should add some time deltas to inform the user what is going on
// Rather than using a Stateless (Unit Struct), consider making this at least remember the KeySize
// However, that would be easily done within the RSA Module
impl PrimeFinder {
    pub fn find(size: &KeySize) -> Result<BigUint, Error> {
        let mut generator = NumberGenerator::new(size)?;
        if let Some(prime) = generator.find(|x| ProbableVariant::find(x) == ProbableVariant::Prime) {
            return Ok(prime);
        } else {
            Err(ErrorKind::PrimeNotFound)?
        }
    }
}

#[cfg(test)]
mod tests {
    // extern crate test;
    use super::*;
    // use test::{Bencher, black_box};

    #[test]
    fn should_find_large_prime() {
        let prime = PrimeFinder::find(&KeySize::FiveTwelve).unwrap();
        println!("Prime Number Found: {:?}", prime);
        let prime = PrimeFinder::find(&KeySize::TwentyFourtyEight).unwrap();
        println!("Prime Number Found: {:?}", prime);
    }

    /*
    #[bench]
    fn bench_2048bit_key(b: &mut Bencher) {
        b.iter(|| {
              PrimeFinder::find(KeySize::TwentyFourtyEight).unwrap();
        });
    }

    #[bench]
    fn bench_512bit_key(b: &mut Bencher) {
        b.iter(|| {
            PrimeFinder::find(KeySize::FiveTwelve).unwrap();
        });
    }
    */
}
