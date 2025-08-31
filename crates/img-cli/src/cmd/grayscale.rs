use std::path::PathBuf;

use clap::{
    ArgMatches,
    Command,
};
use img::operation::color::grayscale::grayscale;

use crate::io::{
    read_image,
    write_image,
};

use super::common::{
    INPUT_ARG_NAME,
    OUTPUT_ARG_NAME,
    input_arg,
    output_arg,
};

pub const CMD_NAME: &str = "grayscale";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME).arg(input_arg()).arg(output_arg())
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    let image = grayscale(&image);
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}
