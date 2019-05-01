mod primes;
mod rsa;
mod err;
mod simpledb;
mod math;
mod cli;

use cli::App;
use failure::Error;

fn main() -> Result<(), Error> {
    App::run()?;
    Ok(())
}
