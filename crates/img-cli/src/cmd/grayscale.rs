use std::path::PathBuf;

use clap::{ArgMatches, Command};
use img::prelude::*;

use crate::io::{read_image, write_image};

use super::common::{input_arg, output_arg, INPUT_ARG_NAME, OUTPUT_ARG_NAME};

pub const CMD_NAME: &str = "grayscale";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME).arg(input_arg()).arg(output_arg())
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let mut image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    grayscale(&mut image);
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}
