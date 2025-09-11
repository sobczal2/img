use std::{
    path::PathBuf,
    str::FromStr,
};

use clap::{
    ArgMatches,
    Command,
    arg,
    value_parser,
};
use img::prelude::*;

use crate::{
    io::{
        read_image,
        write_image,
    },
    param::channel_flags::ChannelFlags,
};

use super::common::{
    INPUT_ARG_NAME,
    OUTPUT_ARG_NAME,
    input_arg,
    output_arg,
};

pub const CMD_NAME: &str = "gamma-correction";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME)
        .arg(input_arg())
        .arg(output_arg())
        .arg(
            arg!(-g --gamma <gamma> "gamma value to use in the filter")
                .required(true)
                .value_parser(value_parser!(f32)),
        )
        .arg(
            arg!(-f --flags <flags> "channel flags in format [R][G][B][A]")
                .default_value("RGB")
                .value_parser(ChannelFlags::from_str),
        )
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    let gamma = matches.get_one::<f32>("gamma").unwrap();
    let channel_flags = *matches.get_one::<ChannelFlags>("flags").unwrap();
    let image = gamma_correction(&image, *gamma, channel_flags.into());
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}
