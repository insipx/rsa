//! Some General Math Helper Functions
//! NOTE: Fermat and Rabin-Miller used for Primality testing may be found implementede within the 'ProbableVariant' enum inside primes/gen.rs
//! These functions are not exposed as public-api because their use is strictly for generating large prime numbers
//! These are generally helper functions
use crate::err::ErrorKind;

use num_bigint::{BigUint, BigInt, ToBigInt, ToBigUint};
use num_traits::{One, Zero};
use num_integer::Integer;
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



pub fn prime_phi(p: &BigUint, q: &BigUint) -> BigUint {
    (p - BigUint::one()) * (q - BigUint::one())
}


// Euclids Extended GCD
pub fn egcd<I>(a: &I, b: &I) -> (BigInt, BigInt, BigInt) where I: Integer + core::ops::Add + core::ops::Div + core::ops::Sub {
    let (mut a, mut b) = (a, b);
    let (mut x, mut y, mut u, mut v) = (Zero::zero(), One::one(), One::one(), Zero::zero());


    let (mut q, mut r, mut m, mut n) = (Zero::zero(), Zero::zero(), Zero::zero(), Zero::zero());
    while a != Zero::zero() {
        // first tuple
        q = &b / &a;
        r = &b % &a;

        // second tuple
        m = &x - &u * &q;
        n = &y - &v * &q;

        // third tuple
        b = a.clone();
        a = r.clone();
        x = u.clone();
        y = v.clone();
        u = m.clone();
        v = n.clone();
    }
    return (b.clone(), x, y)
}

// TODO: figure out a way to avoid using BigInts altogether
// usually E, Phi_n
pub fn modinv<I>(a: &I, b: &I) -> Result<BigUint, Error> where I: Integer {
    let (g, x, _) = egcd(&a, &b);
    if g == One::one() {
        return Ok((x.modulus(b)).to_biguint().ok_or(ErrorKind::BigNumConversion)?);
    } else {
        panic!("Recursion in Modular Inverse Failed!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_modinv() {
        assert_eq!(modinv(&BigUint::from(23usize), &BigUint::from(3usize)).unwrap(), 2usize.into());
        assert_eq!(modinv(&BigUint::from(19usize), &BigUint::from(7usize)).unwrap(), 3usize.into());
        assert_eq!(modinv(&BigUint::from(3083usize), &BigUint::from(487usize)).unwrap(), 5usize.into());
        assert_eq!(modinv(&BigUint::from(3361usize), &BigUint::from(211usize)).unwrap(), 14usize.into());
    }

    #[test]
    fn should_find_egcd() {
        assert_eq!(egcd(&BigUint::from(23usize), &BigUint::from(3usize)).unwrap(), (1, -1, 8));
        assert_eq!(egcd(&BigUint::from(19usize), &BigUint::from(7usize)).unwrap(), (1, 3, -8));
        assert_eq!(egcd(&BigUint::from(3083usize), &BigUint::from(487usize)).unwrap(), (1, 121, -766));
        assert_eq!(egcd(&BigUint::from(3361usize), &BigUint::from(211usize)).unwrap(), (1, 14, -223));
    }
}
