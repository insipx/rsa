mod gen;
mod primedb;
use crate::err::ErrorKind;
use failure::{Error};
use num_bigint::BigUint;
use gen::{NumberGenerator, ProbableVariant};

const MINIMUM_KEY_LENGTH: usize = 512;

pub enum KeySize {
    FiveTwelve,
    TenTwentyFour,
    TwentyFourtyEight,
    FourtyNinetySix,
    EightyOneNinetyTwo,
}

impl KeySize {
    fn as_num(&self) -> usize {
        match *self {
            KeySize::FiveTwelve => 512,
            KeySize::TenTwentyFour => 1024,
            KeySize::TwentyFourtyEight => 2048,
            KeySize::FourtyNinetySix => 4096,
            KeySize::EightyOneNinetyTwo => 8192,
        }
    }
}

impl Default for KeySize {
    fn default() -> Self {
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
    pub fn find(size: KeySize) -> Result<BigUint, Error> {
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
    extern crate test;
    use super::*;
    use test::{Bencher, black_box};

    #[test]
    fn should_find_large_prime() {
        let prime = PrimeFinder::find(KeySize::FiveTwelve).unwrap();
        println!("Prime Number Found: {:?}", prime);
        let prime = PrimeFinder::find(KeySize::TwentyFourtyEight).unwrap();
        println!("Prime Number Found: {:?}", prime);
    }

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
}
