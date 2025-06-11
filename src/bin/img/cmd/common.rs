use std::path::PathBuf;

use clap::{arg, value_parser, Arg};

pub const INPUT_ARG_NAME: &str = "input";
pub fn input_arg() -> Arg {
    arg!(-i --input <file> "input png file")
        .required(true)
        .value_parser(value_parser!(PathBuf))
}

pub const OUTPUT_ARG_NAME: &str = "output";
pub fn output_arg() -> Arg {
    arg!(-o --output <file> "output png file")
        .required(true)
        .value_parser(value_parser!(PathBuf))
}
