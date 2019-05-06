mod primes;
mod rsa;
mod err;
mod simpledb;
mod math;
mod cli;

use cli::App;

fn main() {

    if let Err(e) = App::run() {
        eprintln!("{}", e);
    }
}
