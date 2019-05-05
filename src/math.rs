//! Some General Math Helper Functions
//! NOTE: Fermat and Rabin-Miller used for Primality testing may be found implementede within the 'ProbableVariant' enum inside primes/gen.rs
//! These functions are not exposed as public-api because their use is strictly for generating large prime numbers
//! These are generally helper functions
use num_bigint::{BigUint, BigInt, ToBigInt, ToBigUint};
use num_traits::{One, Zero};


pub trait Mod<B = Self> {
    type Output;

    fn modulus(self, rhs: B) -> Self::Output;
}

impl Mod for BigUint {
    type Output = BigUint;

    fn modulus(self, rhs: BigUint) -> BigUint {
        (self.clone() % rhs.clone()) + rhs.clone()
    }
}

impl Mod for BigInt {
    type Output = BigInt;

    fn modulus(self, rhs: BigInt) -> BigInt {
        (self.clone() % rhs.clone()) + rhs.clone()
    }
}

pub fn prime_phi(p: &BigUint, q: &BigUint) -> BigUint {
    (p - BigUint::one()) * (q - BigUint::one())
}



// Euclids Extended GCD
pub fn egcd(a: &BigUint, b: &BigUint) -> (BigInt, BigInt, BigInt) {
    let (mut a, mut b) = (a.to_bigint().unwrap(), b.to_bigint().unwrap());
    let (mut x, mut y, mut u, mut v) = (BigInt::zero(), BigInt::one(), BigInt::one(), BigInt::zero());


    let (mut q, mut r, mut m, mut n) = (BigInt::zero(), BigInt::zero(), BigInt::zero(), BigInt::zero());
    while a != BigInt::zero() {
        // first tuple
        q = b.clone() / a.clone();
        r = b.clone() % a.clone();

        // second tuple
        m = x.clone() - u.clone() * q.clone();
        n = y.clone() - v.clone() * q.clone();

        // third tuple
        b = a.clone();
        a = r.clone();
        x = u.clone();
        y = v.clone();
        u = m.clone();
        v = n.clone();
    }
    let gcd = b;
    return (gcd.clone(), x, y)
}

// usually E, Phi_n
pub fn modinv(a: &BigUint, b: &BigUint) -> BigUint {
    let (g, x, _) = egcd(&a, &b);
    let b = b.to_bigint().expect("Conversion failed");
    println!("X % B: {}", (x.clone().modulus(b.clone())));
    if g == One::one() {
        return (x.modulus(b)).to_biguint().expect("Result was negative");
    } else {
        panic!("Recursion in Modular Inverse Failed!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_find_modinv() {
        let inv = modinv(&BigUint::from(23usize), &BigUint::from(3usize));
        println!("Inv: {}", inv);
        let inv = modinv(&BigUint::from(19usize), &BigUint::from(7usize));
        println!("Inv: {}", inv);
        let inv = modinv(&BigUint::from(3083usize), &BigUint::from(487usize));
        println!("Inv: {}", inv);
        let inv = modinv(&BigUint::from(3361usize), &BigUint::from(211usize));
        println!("Inv: {}", inv);
    }

    #[test]
    fn should_find_egcd() {
        let (g, x, y) = egcd(&BigUint::from(7usize), &BigUint::from(19usize));
        println!("G: {}, X: {}, Y: {}", g, x, y);
    }
}
