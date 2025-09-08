use std::path::PathBuf;

use clap::{ArgMatches, Command};
use img::operation::blur::kuwahara::kuwahara;

use crate::{cmd::common::{input_arg, output_arg, INPUT_ARG_NAME, OUTPUT_ARG_NAME}, io::{read_image, write_image}};


pub const CMD_NAME: &str = "kuwahara";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME).arg(input_arg()).arg(output_arg())
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    let image = kuwahara(&image);
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}
