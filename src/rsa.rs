mod lib;
use num_bigint::BigUint;

pub struct Primes {
    /// P
    p: BigUint,
    /// Q
    q: BigUint,
    /// The Size of both P and Q. This size (in bits) is guaranteed to be the same for both P and Q
    size: u64
}

