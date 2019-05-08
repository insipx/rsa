//! Where the magic happens
use crate::primes::{KeySize, PrimeFinder};
use crate::simpledb::SimpleDB;
use crate::math;
use crate::err::ErrorKind;
use std::collections::HashMap;
use std::cell::RefCell;
use num_bigint::BigUint;
use num_traits::{Zero};
use serde::{Serialize, Deserialize};
use failure::{Error};

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
    n: BigUint,
    d: Option<BigUint>,
    size: KeySize
}

type PrivateKey = BigUint;
type PublicKey = BigUint;

impl RSA {
    pub fn new(n: BigUint, d: Option<BigUint>, size: KeySize) -> Self {
        RSA { n,d,size }
    }

    pub fn private_exists(&self) -> bool {
        self.d.is_some()
    }

    pub fn private(&self) -> Result<&PrivateKey, Error> {
        Ok(self.d.as_ref().ok_or(ErrorKind::PrivateKeyNotFound)?)
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
        let phi_n = math::prime_phi(&p, &q);
        let d = math::modinv(&E.into(), &phi_n)?;

        return Ok(RSA { d: Some(d), n, size: size.clone() })
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
    // TODO: accept a message *as bytes* allowing for anything to be encrypted
    pub fn encrypt(&self, user: &String, data: &[u8]) -> Result<String, Error> {
        // TODO: change so base64 is only used once
        if let Some(rsa) = self.map.borrow().get(user) {
            let mut encrypted = String::new();
            for block in data.chunks(Self::chunk_size(rsa.size())) {
                let num = BigUint::from_bytes_be(block);
                let encrypted_block = num.modpow(&E.into(), rsa.public());
                // encrypted.extend(encrypted_block.to_bytes_be().iter());
                encrypted.push_str(&format!("{}?", base64::encode(encrypted_block.to_bytes_be().as_slice())));
            }
            Ok(encrypted)
        } else {
            return Err(ErrorKind::UserNotFound)?;
        }
    }

    pub fn decrypt(&self, user: &String, data: &String) -> Result<Vec<u8>, Error> {
        if let Some(rsa) = self.map.borrow().get(user) {
            let mut decrypted = Vec::new();
            for data_chunk in data.split('?').filter(|&x| !x.is_empty()) {
                // println!("DATACHUNK: {}", data_chunk);
                let raw = base64::decode(&data_chunk)?;
                let encrypted = BigUint::from_bytes_be(&raw.as_slice());
                let decrypted_chunk = encrypted.modpow(rsa.private()?, rsa.public());
                decrypted.append(&mut decrypted_chunk.to_bytes_be());
            }
            Ok(decrypted)
        } else {
            return Err(ErrorKind::UserNotFound)?;
        }
    }

    fn chunk_size(key_size: &KeySize) -> usize {
        (key_size.as_num() - 16) / 8
    }

    pub fn import(&self, user: &str, opts: RSA) {
        // if user already exists in DB, we might only want to add the private key
        if opts.private_exists() {
            if let Some(rsa) = self.map.borrow_mut().get_mut(user) { // if the user already exists in the DB
                (*rsa).d = opts.d;
                (*rsa).n = opts.n;
                (*rsa).size = opts.size;
            }
        } else {
            self.map.borrow_mut().insert(user.to_string(), opts);
        }
    }

    pub fn import_private(&self, user: &str, private_key: &BigUint) -> Result<(), Error> {
        if self.map.borrow().contains_key(user) {
            if let Some(rsa) = self.map.borrow_mut().get_mut(user) {
                (*rsa).d = Some(private_key.to_owned());
            }
        } else {
            Err(ErrorKind::ImportOrder)?
        }
        Ok(())
    }

    pub fn export(&self, user: &str, key: KeyType) -> Result<String, Error> {
        if let Some(rsa) = self.map.borrow().get(user) {
            match key {
                KeyType::Private => Ok(base64::encode(&rsa.private()?.to_bytes_be())),
                KeyType::Public => Ok(base64::encode(&rsa.public().to_bytes_be()))
            }
        } else {
            Err(ErrorKind::UserNotFound)?
        }
    }

    // if the user exists, the private key must exist
    pub fn user_exists(&self, user: &str) -> bool {
        self.map.borrow().contains_key(user)
    }

    pub fn list(&self) -> Result<String, Error> {
        let mut list = String::new();
        list.push_str(&format!("{}\n", self.db.file_path().canonicalize()?.to_str().unwrap()));
        list.push_str("------------------------------------------\n");
        for (user, rsa) in self.map.borrow().iter() {
            list.push_str(&format!("{}: rsa{}/{}\n", user, rsa.size().as_string(), self.public_identifier(rsa)));
        }
        Ok(list)
    }

    fn public_identifier(&self, rsa: &RSA) -> String {
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
