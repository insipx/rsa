//! Some General Math Helper Functions
//! NOTE: Fermat and Rabin-Miller used for Primality testing may be found implementede within the 'ProbableVariant' enum inside primes/gen.rs
//! These functions are not exposed as public-api because their use is strictly for generating large prime numbers
//! These are generally helper functions
use num_bigint::{BigUint, BigInt, ToBigInt, ToBigUint};
use num_traits::identities::{One, Zero};


pub fn prime_phi(p: &BigUint, q: &BigUint) -> BigUint {
    (p - BigUint::one()) * (q - BigUint::one())
}



// Euclids Extended GCD
// returns g, x, y so ax + by = gcd(a, b)
// Ints used here in case of negative numbers. It is all modded so nothing should actually turn negative
pub fn egcd(a: &BigUint, b: &BigUint) -> (BigInt, BigInt, BigInt) {
    let (zero, one): (BigInt, BigInt) = (Zero::zero(), One::one());
    // u_a, v_a, u_b, v_b = 1, 0, 0, 1
    let (mut u_a, mut v_a, mut u_b, mut v_b) = (one.clone(), zero.clone(), zero.clone(), one.clone());
    let (mut aa, mut bb) = (a.to_bigint().unwrap(), b.to_bigint().unwrap());

    while aa != zero {
        let q = bb.clone() / aa.clone();

        let new_a = bb.clone() - q.clone() * aa.clone();
        bb = aa;
        aa = new_a;

        let new_u_a = u_b.clone() - q.clone() * u_a.clone();
        u_b = u_a;
        u_a = new_u_a;

        let new_v_a = v_b.clone() - q.clone() * v_a.clone();
        v_b = v_a;
        v_a = new_v_a;
    }
    (bb, u_b, v_b)
}

pub fn modinv(a: &BigUint, b: &BigUint) -> BigUint {
    let (g, x, _) = egcd(&a, &b);
    println!("G: {}, X: {}", g, x);
    let (g, x) = (g, x);
    if g == One::one() {
        return x.to_biguint().unwrap() % b;
    } else {
        panic!("Recursion in Modular Inverse Failed!");
    }
}
