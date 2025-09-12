use std::path::PathBuf;

use clap::{
    Arg,
    value_parser,
};

pub const ARG_NAME: &str = "output";
pub fn arg() -> Arg {
    clap::arg!(-o --output <file> "output file").required(true).value_parser(value_parser!(PathBuf))
}
