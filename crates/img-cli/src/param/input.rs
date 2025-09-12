use std::path::PathBuf;

use clap::{
    Arg,
    value_parser,
};

pub const ARG_NAME: &str = "input";
pub fn arg() -> Arg {
    clap::arg!(-i --input <file> "input file").required(true).value_parser(value_parser!(PathBuf))
}
