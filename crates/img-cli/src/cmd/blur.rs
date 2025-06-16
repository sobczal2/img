use std::path::PathBuf;

use clap::{arg, value_parser, ArgMatches, Command};
use img::ops::blur::mean::mean_blur;

use crate::io::{read_image, write_image};

use super::common::{input_arg, output_arg, INPUT_ARG_NAME, OUTPUT_ARG_NAME};

pub const CMD_NAME: &str = "blur";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME)
        .arg(input_arg())
        .arg(output_arg())
        .arg(
            arg!(-r --radius <radius> "kernel radius")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    let target_radius = matches.get_one::<usize>("radius").unwrap();
    let image = mean_blur(&image, *target_radius)?;
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}
