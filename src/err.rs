use failure::{Context, Fail, Backtrace};
use std::fmt::Display;

#[derive(Debug)]
struct RSAError {
    inner: Context<ErrorKind>
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "The key length entered is too small, or not a power of 2")]
    InvalidKeyLength,
    #[fail(display = "Could not convert native integer to large integer library")]
    CouldNotConvert,
    #[fail(display = "Could not Find A Suitable Prime Number")]
    PrimeNotFound,
    #[fail(display = "Error Loading Keys into or From the Database")]
    Database,
    #[fail(display = "Error when parsing bytes during encryption/decryption")]
    BytesParse,
    #[fail(display = "User not in Database. Have you created a key?")]
    UserNotFound,
    #[fail(display = "Conversion between BigInteger types failed")]
    BigNumConversion,
    #[fail(display = "Could Not Fetch Database of Small Primes")]
    DataLoad,
    #[fail(display = "Failed to Parse Input. Number must be greater than 0")]
    WrongNumber,
    #[fail(display = "Failed to parse input")]
    CouldNotParse,
    #[fail(display = "Could not decode base64 input")]
    CouldNotDecode,
    #[fail(display = "Must specify a user")]
    NoUserSpecified
}


impl Fail for RSAError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for RSAError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl RSAError {
    pub fn kind(&self) -> ErrorKind {
        *self.inner.get_context()
    }
}

impl From<ErrorKind> for RSAError {
    fn from(kind: ErrorKind) -> RSAError {
        RSAError { inner: Context::new(kind) }
    }
}

impl From<Context<ErrorKind>> for RSAError {
    fn from(inner: Context<ErrorKind>) -> RSAError {
        RSAError { inner: inner }
    }
}
