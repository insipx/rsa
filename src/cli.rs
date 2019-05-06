//! The Front-End

use crate::rsa::{AlgoRSA, RSA};
use crate::simpledb::SimpleDB;
use crate::primes::KeySize;
use crate::err::ErrorKind;
use std::path::PathBuf;
use std::collections::HashMap;
use quicli::prelude::*;
use structopt::StructOpt;
use failure::{ResultExt, Error};


#[derive(Debug, StructOpt)]
pub struct CLI {
    #[structopt(long = "db")]
    database: String,

    #[structopt(long = "encrypt", short = "e")]
    encrypt: Option<String>,

    #[structopt(long = "decrypt", short = "d")]
    decrypt: Option<String>,

    #[structopt(long = "generate", short = "g")]
    generate: bool,

    #[structopt(long = "import", short = "i")]
    import: Option<String>, // file

    #[structopt(long = "export-public")]
    export_public: Option<String>, // user

    #[structopt(long = "export-private")]
    export_private: Option<String>, // user

    #[structopt(long = "list-all", short = "l")]
    list_all: bool,

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
            self.rsa.create(&user, &key_size)?;

            println!("User {} with public/private keys added to database!", user);
        }

        Ok(())
    }

    pub fn decrypt_dialog(&self) -> Result<(), Error> {
        if let Some(message) = &self.args.decrypt {
            println!("Who are you?");
            let user = prompt_string()?;

            println!("{}", self.rsa.decrypt(&user, &message)?);
        }

        Ok(())
    }

    pub fn encrypt_dialog(&self) -> Result<(), Error> {
        if let Some(message) = &self.args.encrypt {
            println!("Who are you encrypting this message to? (Enter the UserNames of Recipients): ");
            let user = prompt_string()?;
            println!("{}", self.rsa.encrypt(&user, &message)?);
        }

        Ok(())
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

        opts.generate_dialog()?;
        opts.encrypt_dialog()?;
        opts.decrypt_dialog()?;
 
        println!("\n\nGood Bye!");

        opts.finish()?;
        Ok(())
    }
}
