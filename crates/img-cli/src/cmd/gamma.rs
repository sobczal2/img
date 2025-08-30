use std::path::PathBuf;

use clap::{ArgMatches, Command, arg, value_parser};
use img::operation::color::gamma_correction::gamma_correction;

use crate::io::{read_image, write_image};

use super::common::{INPUT_ARG_NAME, OUTPUT_ARG_NAME, input_arg, output_arg};

pub const CMD_NAME: &str = "gamma";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME)
        .arg(input_arg())
        .arg(output_arg())
        .arg(
            arg!(-g --gamma <gamma> "gamma value to use in the filter")
                .required(true)
                .value_parser(value_parser!(f32)),
        )
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    let gamma = matches.get_one::<f32>("gamma").unwrap();
    let image = gamma_correction(&image, *gamma);
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}
