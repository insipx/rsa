use std::collections::HashMap;
use serde::Deserialize;


// TODO: This should not be static
static PRIMES: &'static str = include_str!("small_primes.json");

#[derive(Deserialize, Debug, Clone)]
pub struct PrimeDB (HashMap<String, usize>);

impl PrimeDB {

    pub fn get() -> HashMap<String, usize> {
        let db: PrimeDB = serde_json::from_str(PRIMES).unwrap();
        db.0
    }
}

