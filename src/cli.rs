//! The Front-End

use crate::rsa::{AlgoRSA, RSA, KeyType};
use crate::simpledb::SimpleDB;
use crate::primes::KeySize;
use crate::err::ErrorKind;
use std::path::PathBuf;
use std::collections::HashMap;
// use quicli::prelude::*;
use structopt::StructOpt;
use failure::{ResultExt, Error};


#[derive(Debug, StructOpt)]
#[structopt(name = "rsa", about = "A simple RSA command-line application")]
pub struct CLI {
    #[structopt(long = "db")]
    /// Specify the database that the private/public keys will be stored. Required
    database: String,

    #[structopt(long = "user", short = "u")]
    /// Specify the user for user-specific actions like encrypting, decrypting, and exporting
    user: Option<String>,

    #[structopt(long = "encrypt", short = "e")]
    /// Encrypt. Requires a String argument that is the data to encrypt
    encrypt: Option<String>,

    #[structopt(long = "encrypt-file")]
    // Specify a file to encrypt. File may include anything
    encrypt_file: Option<String>,

    #[structopt(long = "decrypt", short = "d")]
    /// Decrypt data
    decrypt: Option<String>,

    #[structopt(long = "decrypt-to")]
    /// Specify a file to decrypt data to
    decrypt_to: Option<String>,

    #[structopt(long = "generate", short = "g")]
    /// Generate a new key
    generate: bool,

    #[structopt(long = "import", short = "i")]
    /// Import a key
    import: Option<String>, // file

    #[structopt(long = "export-public")]
    /// Export a public key
    export_public: bool, // user

    #[structopt(long = "export-private")]
    /// Export a private key
    export_private: bool, // user

    #[structopt(long = "list-all", short = "l")]
    /// List all key-pairs present in the database
    list: bool,

    // #[structopt(flatten)]
    //verbosity: Verbosity,
}


fn prompt_number() -> Result<usize, Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().parse::<usize>().context(ErrorKind::WrongNumber)?)
}

fn prompt_string() -> Result<String, Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().into())
}

pub struct Opts {
    args: CLI,
    rsa: AlgoRSA
}


impl Opts {
    pub fn parse() -> Result<Self, Error> {
        let args = CLI::from_args();
        let database_file = PathBuf::from(args.database.clone());
        let database: SimpleDB<HashMap<String, RSA>> = SimpleDB::new(database_file)?;
        Ok(Opts {
            args: args,
            rsa: AlgoRSA::new(database)?
        })
    }

    //TODO: Remove panic
    pub fn generate_dialog(&self) -> Result<(), Error> {
        if self.args.generate {
            println!("Who Are You?");
            let user = prompt_string()?;

            println!("Hello {}. Choose a KeySize (One of 512, 1024, 2048, 4096, 8192)", user);
            let key_size = KeySize::from_input(&prompt_number()?)?;

            println!("Hold On, Generating Key of size {} and committing to the Database", key_size.as_num());
            // TODO: Start in separate thread
            self.rsa.create(&user, &key_size)?;

            println!("User {} with public/private keys added to database!", user);
        }

        Ok(())
    }

    pub fn decrypt_dialog(&self) -> Result<(), Error> {
        if let Some(message) = &self.args.decrypt {
            let user = self.args.user.as_ref().ok_or(ErrorKind::NoUserSpecified)?;

            println!("{}", self.rsa.decrypt(&user, &message)?);
        }

        Ok(())
    }

    pub fn encrypt_dialog(&self) -> Result<(), Error> {
        if let Some(message) = &self.args.encrypt {
            let user = self.args.user.as_ref().ok_or(ErrorKind::NoUserSpecified)?;
            let encrypted = self.rsa.encrypt(&user, &message)?;
            println!("--------------------- BEGIN RSA MESSAGE  ---------------------");
            println!("{}", textwrap::fill(&encrypted, 70));
            println!("--------------------- BEGIN RSA MESSAGE  ---------------------");
        }

        Ok(())
    }

    pub fn export_dialog(&self) -> Result<(), Error> {
        if self.args.export_public {
            let user = self.args.user.as_ref().ok_or(ErrorKind::NoUserSpecified)?;
            let key = self.rsa.export(&user, KeyType::Public)?;
            let export = format!("----------------------- BEGIN RSA PUBLIC KEY ------------------------
                                 \n {}
                                 \n----------------------- END RSA PUBLIC KEY --------------------------",
                                 textwrap::fill(&key, 70));
            println!("{}", export);
        }

        if self.args.export_private {
            let user = self.args.user.as_ref().ok_or(ErrorKind::NoUserSpecified)?;
            let key = self.rsa.export(&user, KeyType::Private)?;
            let export = format!("----------------------- BEGIN RSA PRIVATE KEY ------------------------
                                 \n {}
                                 \n----------------------- END RSA PRIVATE KEY --------------------------",
                                 textwrap::fill(&key, 70));
            println!("{}", export);
        }

        Ok(())
    }

    pub fn list_dialog(&self) -> Result<(), Error> {
        if self.args.list {
            println!("{}", self.rsa.list()?);
        }

        Ok(())
    }

    pub fn finish(self) -> Result<(), Error> {
        self.rsa.save_keys()?;
        Ok(())
    }
}

pub struct App;

impl App {


    pub fn run() -> Result<(), Error> {
        let opts = Opts::parse()?;

        opts.generate_dialog()?;
        opts.encrypt_dialog()?;
        opts.decrypt_dialog()?;
        opts.export_dialog()?;
        opts.list_dialog()?;

        opts.finish()?;
        Ok(())
    }
}
