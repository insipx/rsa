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
