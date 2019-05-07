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
use failure::{Error, ResultExt};

pub const E: usize = 65537; // the encryption exponent

/*
 * 1. Bob Chooses Secret primes p and q and computes n = pq
 * 2. Bob Chooses e with gcd(e, (p-1)(q-1)) = 1
 * 3. Bob computes d with de = 1 (mod (p-1)(q-1))
 * 4. Bob makes n and e public, keeps p, q, d secret
 * 5.Alice encrypts m as c = m^e (mod n) and sends c to bob
 * 6. Bob decrypts by computing m = c^d (mod n)
 */

pub enum KeyType {
    Public,
    Private
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RSA {
    d: BigUint,
    n: BigUint,
    size: KeySize
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

    pub fn size(&self) -> &KeySize {
        &self.size
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
        let multiple_size = size.as_half();
        let mut p = PrimeFinder::find(&multiple_size)?;
        let mut q = PrimeFinder::find(&multiple_size)?;
        loop {
            if &p % E == BigUint::zero() {
                p = PrimeFinder::find(&multiple_size)?;
            }

            if &q % E == BigUint::zero() {
                q = PrimeFinder::find(&multiple_size)?;
            }

            if &p % E != BigUint::zero() && &q % E != BigUint::zero() {
                break;
            }
        }
        let n = &p * &q;
        let phi_n = prime_phi(&p, &q);
        let d = math::modinv(&E.into(), &phi_n)?;

        return Ok(RSA { d, n, size: size.clone() })
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

    pub fn import(&self, user: &str, opts: RSA) -> BigUint {
        unimplemented!();
    }

    pub fn export(&self, user: &str, key: KeyType) -> Result<String, Error> {
        if let Some(rsa) = self.map.borrow().get(user) {

            match key {
                KeyType::Private => {
                    let key = base64::encode(&rsa.private().to_bytes_be());
                    /*let export = format!("======================= BEGIN RSA PRIVATE KEY ========================
                                         \n {}
                                         \n======================= END RSA PRIVATE KEY ==========================",
                                         key);
                    */
                    // Ok(textwrap::fill(&export, 70))
                    Ok(key)
                },
                KeyType::Public => {
                    let key = base64::encode(&rsa.public().to_bytes_be());
                    // let export = format!("======================= BEGIN RSA PUBLIC KEY ========================
                                         // \n {}
                                         // \n======================= END RSA PUBLIC KEY ==========================",
                                         // key);
                    // Ok(textwrap::fill(&export, 70))
                    Ok(key)
                }
            }
        } else {
            Err(ErrorKind::UserNotFound)?
        }
    }

    pub fn list(&self) -> Result<String, Error> {
        let mut list = String::new();
        list.push_str(&format!("{}\n", self.db.file_path().canonicalize()?.to_str().unwrap()));
        list.push_str("------------------------------------------\n");
        for (user, rsa) in self.map.borrow().iter() {
            list.push_str(&format!("{}: rsa{}/{}\n", user, rsa.size().as_string(), self.public_identifier(user, rsa)));
        }
        Ok(list)
    }

    fn public_identifier(&self, user: &str, rsa: &RSA) -> String {
        let key = base64::encode(rsa.public().to_bytes_be().as_slice());
        key[0..16].to_string().to_ascii_uppercase()
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
