//! The Front-End

use crate::rsa::{AlgoRSA, RSA};
use crate::simpledb::SimpleDB;
use crate::primes::KeySize;
use std::path::PathBuf;
use std::collections::HashMap;
use quicli::prelude::*;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
pub struct CLI {
    #[structopt(long = "db")]
    database: String,
    #[structopt(long = "decrypt", short = "d")]
    decrypt: Option<String>,
    #[structopt(long = "encrypt", short = "e")]
    encrypt: Option<String>,
    // supply a username
    #[structopt(long = "generate", short = "g")]
    generate: Option<String>,

    // #[structopt(flatten)]
    //verbosity: Verbosity,
}

pub struct Opts {
    decrypt: Option<String>,
    encrypt: Option<String>,
    // supply a username
    generate: Option<String>,
    rsa: AlgoRSA
}

impl Opts {
    pub fn parse() -> Result<Self, Error> {
        let args = CLI::from_args();
        let database_file = PathBuf::from(args.database);
        let database: SimpleDB<HashMap<String, RSA>> = SimpleDB::new(database_file)?;
        Ok(Opts {
            encrypt: args.encrypt,
            decrypt: args.decrypt,
            generate: args.generate,
            rsa: AlgoRSA::new(database)?
        })
    }

    //TODO: Remove panic
    pub fn generate_dialog(&self) -> Result<(), Error> {
        if let Some(name) = &self.generate {
            println!("Hello {}. Choose a KeySize (One of 512, 1024, 2048, 4096, 8192)", name);
            let mut input = String::new();
            std::io::stdin().read_line(&mut input);
            let key_size: usize = input.trim().parse().unwrap(); // TODO: Get rid of unwrap
            let key_size = KeySize::from(key_size);
            println!("Hold On, Generating Key of size {} and committing to the Database", key_size.as_num());
            self.rsa.create(name, &key_size)?;
            println!("User {} with public/private keys added to database!", name);
            Ok(())
        } else {
            panic!("Generate Empty");
        }
    }

    pub fn decrypt_dialog(&self) -> Result<(), Error> {
        if let Some(message) = &self.decrypt {
            println!("Who are you?");
            let mut user = String::new();
            std::io::stdin().read_line(&mut user);
            println!("{}", self.rsa.decrypt(&user, &message)?);
            Ok(())
        } else {
            panic!("Decryption Failed. Decryption Variable Empty.");
        }
    }

    pub fn encrypt_dialog(&self) -> Result<(), Error> {
        if let Some(message) = &self.encrypt {
            println!("Who are you encrypting this message to? (Enter the UserNames of Recipients): ");
            let mut user = String::new();
            std::io::stdin().read_line(&mut user);
            println!("{}", self.rsa.encrypt(&user, &message)?);
            Ok(())
        } else {
            panic!("Encryption Variable Empty. Failed encryption");
        }
    }

    pub fn finish(self) -> Result<(), Error> {
        self.rsa.save_keys()?;
        Ok(())
    }
}

pub struct App {
    opts: Opts
}

impl App {
    pub fn new(opts: Opts) -> Self {
        App { opts }
    }

    pub fn run() -> Result<(), Error> {
        let opts = Opts::parse()?;
        if opts.generate.is_some() {
            opts.generate_dialog()?;
        } else if opts.encrypt.is_some() {
            opts.encrypt_dialog()?;
        } else if opts.decrypt.is_some() {
            opts.decrypt_dialog()?;
        }
        println!("\n\nGood Bye!");
        opts.finish()?;
        Ok(())
    }
}
