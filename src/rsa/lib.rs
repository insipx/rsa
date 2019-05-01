//! Where the magic happens
use crate::primes::{KeySize, PrimeFinder};
use crate::simpledb::SimpleDB;
use crate::math;
use crate::err::ErrorKind;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use num_bigint::BigUint;
use num_traits::identities::{One, Zero};
use serde::{Serialize, Deserialize};
use failure::Error;

pub const BASE: u32 = 16;

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
    d: BigUint,
    n: BigUint,
    e: BigUint,
}

pub struct PrivateKey {
    p: BigUint,
    q: BigUint,
    d: BigUint
}

pub struct PublicKey {
    n: BigUint,
    e: BigUint
}

impl RSA {
    pub fn private(&self) -> PrivateKey {
        return PrivateKey {
            p: self.p.clone(),
            q: self.q.clone(),
            d: self.d.clone()
        };
    }

    pub fn public(&self) -> PublicKey {
        return PublicKey {
            n: self.n.clone(),
            e: self.e.clone()
        }
    }
}



pub struct AlgoRSA {
    db: SimpleDB<HashMap<String, RSA>>,
    // HashMap extracted from DB
    map: RefCell<HashMap<String, RSA>>,
}

impl AlgoRSA {

    pub fn new(db: SimpleDB<HashMap<String, RSA>>) -> Result<Self, Error> {
        let map = db.get()?;
        Ok(AlgoRSA {
            db,
            map: RefCell::new(map)
        })
    }

    // uses a static e at size 65537
    // TODO: Try to remove as many clones as possible. This is fairly ridiculous tbh
    // could extract finding D logic to a different method maybe?
    fn generate(size: &KeySize) -> Result<RSA, Error> {
        let e = BigUint::from(65537usize);
        let size = size.as_half();
        let mut p = PrimeFinder::find(&size)?;
        let mut q = PrimeFinder::find(&size)?;
        loop {
            if p.clone() % e.clone() == BigUint::zero() {
                p = PrimeFinder::find(&size)?;
            }

            if q.clone() % e.clone() == BigUint::zero() {
                q = PrimeFinder::find(&size)?;
            }

            if p.clone() % e.clone() != BigUint::zero() && q.clone() % e.clone() != BigUint::zero() {
                break;
            }
        }
        let n = p.clone() * q.clone();
        let phi_n = prime_phi(&p, &q);
        let d = math::modinv(&e, &phi_n);

        return Ok(RSA { p, q, e, d, n})
    }

    /// Creates a new key and adds it to the Database
    pub fn create(&self, user: &String, size: &KeySize) -> Result<(), Error> {
        let rsa = Self::generate(&size)?;
        self.map.borrow_mut().insert(user.clone(), rsa);
        Ok(())
    }
    // 5.Alice encrypts m as c = m^e (mod n) and sends c to bob
    // 6. Bob decrypts by computing m = c^d (mod n)
    //
    // Returns a Base64-encoded string that is the encrypted message
    // User here is the user the message is being encrypted for
    // TODO: Create separate ASCII-armor methods
    pub fn encrypt(&self, user: &String, message: &String) -> Result<String, Error> {
        // TODO: Make this error better
        if let Some(rsa) = self.map.borrow().get(user) {
            let bytes = message.as_bytes();
            let num = BigUint::parse_bytes(bytes, BASE).ok_or(ErrorKind::BytesParse)?;
            let encrypted = num.modpow(&rsa.e, &rsa.n);
            Ok(base64::encode(encrypted.to_bytes_be().as_slice()))
        } else {
            return Err(ErrorKind::UserNotFound)?;
        }
    }

    pub fn decrypt(&self, user: &String, message: &String) -> Result<String, Error> {
        if let Some(rsa) = self.map.borrow().get(user) {
            let message = base64::decode(&message)?;
            let encrypted = BigUint::from_bytes_be(&message.as_slice());
            let decrypted = encrypted.modpow(&rsa.d, &rsa.n);
            Ok(String::from_utf8(decrypted.to_bytes_be())?)
        } else {
            return Err(ErrorKind::UserNotFound)?;
        }
    }

    pub fn import(user: String, opts: RSA) -> BigUint {
        unimplemented!();
    }

    pub fn export(user: String) -> BigUint {
        unimplemented!();
    }

    // consumes self, saving data to our database.
    // Should be used at the end of the program
    pub fn save_keys(self) -> Result<(), Error> {
        let map = self.map.into_inner();
        self.db.save(map)?;
        Ok(())
    }
}

fn prime_phi(p: &BigUint, q: &BigUint) -> BigUint {
    (p - BigUint::one()) * (q - BigUint::one())
}
