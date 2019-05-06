//! Where the magic happens
use crate::primes::{KeySize, PrimeFinder};
use crate::simpledb::SimpleDB;
use crate::math;
use crate::err::ErrorKind;
use std::collections::HashMap;
use std::cell::RefCell;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use serde::{Serialize, Deserialize};
use failure::Error;

pub const E: usize = 65537; // the encryption exponent

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

#[derive(Serialize, Deserialize, Debug)]
pub struct RSA {
    d: BigUint,
    n: BigUint,
}

type PrivateKey = BigUint;
type PublicKey = BigUint;

impl RSA {

    pub fn private(&self) -> &PrivateKey {
        &self.d
    }

    pub fn public(&self) -> &PublicKey {
        &self.n
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

    // could extract finding D logic to a different method maybe?
    fn generate(size: &KeySize) -> Result<RSA, Error> {
        let size = size.as_half();
        let mut p = PrimeFinder::find(&size)?;
        let mut q = PrimeFinder::find(&size)?;
        loop {
            if &p % E == BigUint::zero() {
                p = PrimeFinder::find(&size)?;
            }

            if &q % E == BigUint::zero() {
                q = PrimeFinder::find(&size)?;
            }

            if &p % E != BigUint::zero() && &q % E != BigUint::zero() {
                break;
            }
        }
        let n = &p * &q;
        let phi_n = prime_phi(&p, &q);
        let d = math::modinv(&E.into(), &phi_n)?;

        return Ok(RSA { d, n})
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
            let num = BigUint::from_bytes_be(bytes);
            let encrypted = num.modpow(&E.into(), &rsa.n);
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
        map.iter().for_each(|x| println!("{:?}", x));
        self.db.save(map)?;
        Ok(())
    }
}

fn prime_phi(p: &BigUint, q: &BigUint) -> BigUint {
    (p - BigUint::one()) * (q - BigUint::one())
}
