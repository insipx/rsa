//! Where the magic happens
use crate::primes::KeySize;
use crate::simpledb::SimpleDB;
use std::collections::HashMap;
use num_bigint::BigUint;
use serde::{Serialize, Deserialize};


/*
 * 1. Bob Chooses Secret primes p and q and computes n = pq
 * 2. Bob Chooses e with gcd(e, (p-1)(q-1)) = 1
 * 3. Bob computes d with de = 1 (mod (p-1)(q-1))
 * 4. Bob makes n and e public, keeps p, q, d secret
 * 5.Alice encrypts m as c = m^e (mod n) and sends c to bob
 * 6. Bob decrypts by computing m = c^d (mod n)
 */

pub struct KeyGen<'a> {
    p: &'a BigUint,
    q: &'a BigUint
}

impl<'a> KeyGen<'a> {

    pub fn new() -> Self {
        unimplemented!();
    }

    // Big Uint used for simplicity purposes
    fn find_e(p: &'a BigUint, q: &'a BigUint) -> BigUint {
        unimplemented!();
    }

    fn find_d(p: &'a BigUint, q: &'a BigUint) -> BigUint {
        unimplemented!();
    }
}

#[derive(Serialize, Deserialize)]
pub struct RSA {
    p: BigUint,
    q: BigUint,
    e: BigUint,
    d: BigUint,
    n: BigUint
}


pub struct AlgoRSA {
    db: SimpleDB<HashMap<String, RSA>>
}

impl AlgoRSA {
   
    /// Creates a new key and adds it to the Database
    pub fn create(user: String, size: KeySize) -> Self {
        unimplemented!();
    }

    pub fn load(user: &String) -> Self {
        unimplemented!();
    }

    pub fn encrypt() -> BigUint {
        unimplemented!();
    }

    pub fn decrypt() -> BigUint {
        unimplemented!();
    }
}
