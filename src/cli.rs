//! The Front-End

use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct CLI {
    #[structopt(flatten)]
    verbosity: Verbosity,
}
