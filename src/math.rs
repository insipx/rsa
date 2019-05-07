//! Some General Math Helper Functions
//! NOTE: Fermat and Rabin-Miller used for Primality testing may be found implementede within the 'ProbableVariant' enum inside primes/gen.rs
//! These functions are not exposed as public-api because their use is strictly for generating large prime numbers
//! These are generally helper functions
use crate::err::ErrorKind;

use num_bigint::{BigUint, BigInt, ToBigInt};
use num_traits::{One, Zero};
use failure::Error;


pub trait Mod<B = Self> {
    type Output;

    fn modulus(self, rhs: B) -> Self::Output;
}

impl Mod for BigUint {
    type Output = BigUint;

    fn modulus(self, rhs: BigUint) -> BigUint {
        ((self.clone() % &rhs) + &rhs) % &rhs
    }
}

impl Mod for BigInt {
    type Output = BigInt;

    fn modulus(self, rhs: BigInt) -> BigInt {
        ((self.clone() % &rhs) + &rhs) % &rhs
    }
}

impl Mod for &BigUint {
    type Output = BigUint;

    fn modulus(self, rhs: &BigUint) -> BigUint {
        ((self.clone() % rhs) + rhs) % rhs
    }
}

impl Mod for &BigInt {
    type Output = BigInt;

    fn modulus(self, rhs: &BigInt) -> BigInt {
        ((self.clone() % rhs) + rhs) % rhs
    }
}


/// Take the phi of n which are two prime numbers
pub fn prime_phi(p: &BigUint, q: &BigUint) -> BigUint {
    (p - BigUint::one()) * (q - BigUint::one())
}

// Euclids Extended GCD
pub fn egcd(a: &BigUint, b: &BigUint) -> Result<(BigInt, BigInt, BigInt), Error> {
    let (mut a, mut b) = (a.to_bigint().ok_or(ErrorKind::BigNumConversion)?, b.to_bigint().ok_or(ErrorKind::BigNumConversion)?);
    let (mut x, mut y, mut u, mut v) = (BigInt::zero(), BigInt::one(), BigInt::one(), BigInt::zero());

    let (mut q, mut r, mut m, mut n);
    while a != Zero::zero() {
        q = &b / &a;
        r = &b % &a;

        m = &x - &u * &q;
        n = &y - &v * &q;

        b = a.clone();
        a = r.clone();
        x = u.clone();
        y = v.clone();
        u = m.clone();
        v = n.clone();
    }
    return Ok((b.clone(), x, y))
}

// TODO: figure out a way to avoid using BigInts altogether
// usually E, Phi_n
pub fn modinv(a: &BigUint, b: &BigUint) -> Result<BigUint, Error> {
    let (g, x, _) = egcd(&a, &b)?;
    let b = b.to_bigint().ok_or(ErrorKind::BigNumConversion)?;
    if g == BigInt::one() {
        return Ok((x.modulus(b)).to_biguint().ok_or(ErrorKind::BigNumConversion)?);
    } else {
        // This will never (hopefully, EVER) happen since p and q are real primes and the gcd phi_n is always 1. Q.E.D
        panic!("P or Q are not real primes such that gcd(e, phi(n)) == 1. Aborting Execution.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_modinv() {
        assert_eq!(modinv(&BigUint::from(23usize), &BigUint::from(3usize)).unwrap(), 2usize.into());
        assert_eq!(modinv(&BigUint::from(19usize), &BigUint::from(7usize)).unwrap(), 3usize.into());
        assert_eq!(modinv(&BigUint::from(3083usize), &BigUint::from(487usize)).unwrap(), 121usize.into());
        assert_eq!(modinv(&BigUint::from(3361usize), &BigUint::from(211usize)).unwrap(), 14usize.into());
    }

    #[test]
    fn should_find_egcd() {
        assert_eq!(egcd(&BigUint::from(23usize), &BigUint::from(3usize)).unwrap(), (BigInt::from(1), BigInt::from(-1), BigInt::from(8)));
        assert_eq!(egcd(&BigUint::from(19usize), &BigUint::from(7usize)).unwrap(), (1.into(), 3.into(), BigInt::from(-8)));
        assert_eq!(egcd(&BigUint::from(3083usize), &BigUint::from(487usize)).unwrap(), (1.into(), 121.into(), BigInt::from(-766)));
        assert_eq!(egcd(&BigUint::from(3361usize), &BigUint::from(211usize)).unwrap(), (1.into(), 14.into(), BigInt::from(-223)));
    }
}
