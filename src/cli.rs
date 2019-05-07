//! The Front-End

use crate::rsa::{AlgoRSA, RSA, KeyType};
use crate::simpledb::SimpleDB;
use crate::primes::KeySize;
use crate::err::ErrorKind;
use num_bigint::BigUint;
use std::path::PathBuf;
use std::collections::HashMap;
use structopt::StructOpt;
use structopt::clap::AppSettings;
use regex::Regex;
use failure::{ResultExt, Error};


// TODO: Make strings that are supposed to be files, files

#[derive(Debug, StructOpt)]
#[structopt(raw(global_setting = "AppSettings::AllowLeadingHyphen"))]
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

    #[structopt(long = "import-public")]
    /// import a public key
    import_public: Option<String>, // data

    #[structopt(long = "import-private")]
    /// import a private key (must have imported a public key first)
    import_private: Option<String>,

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

// Take from format --- BEGIN ---- {content} ---- END --- to just content
fn parse_rsa_format(input: &str) -> Result<String, Error> {
    //  do some basic input sanitization first, in case the user/OS entered some newlines in the file
    //  actually modifying the base64 in any way would lead to total failure, however
    let re_replace = Regex::new(r"[\t\n]*")?;
    let input = re_replace.replace_all(input, "");
    let re_base64 = Regex::new(r"-+[ A-Z]+-+([A-Za-z0-9+/=]+)")?;
    let base64_cap = re_base64.captures(&input).ok_or(ErrorKind::RegexParse)?;
    Ok(base64_cap.get(1).ok_or(ErrorKind::RegexParse)?.as_str().into())
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
            let message = parse_rsa_format(&message)?;
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

    pub fn import_dialog(&self) -> Result<(), Error> {
        let user = self.args.user.as_ref().ok_or(ErrorKind::NoUserSpecified)?;
        if self.args.import_public.is_some() {
            let pubkey = base64::decode(&parse_rsa_format(&self.args.import_public.as_ref().expect("Within checked scope; Q.E.D"))?)?;
            let size = KeySize::from_input(&(pubkey.len() * 8))?;
            let pubkey = BigUint::from_bytes_be(&pubkey);
            let rsa = RSA::new(pubkey, None, size);
            self.rsa.import(user, rsa);
        }

        if self.args.import_private.is_some() {
            let privkey = base64::decode(&parse_rsa_format(&self.args.import_private.as_ref().expect("Within checked scope; Q.E.D"))?)?;
            self.rsa.import_private(user, &BigUint::from_bytes_be(&privkey))?;
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
        opts.import_dialog()?;
        opts.finish()?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_parsed_rsa_format() {
        let test_str = "


\t\t\n------------------ BEGIN  RSA PUBLIC   KEY ---------------\t\n-- --


\t
Onlythisshouldremain
\n

------------------ END RSA PUBLIC KEY ---------------------";
        parse_rsa_format(&test_str).unwrap();

    }
}
