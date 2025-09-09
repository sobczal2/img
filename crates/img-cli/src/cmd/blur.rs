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

use crate::io::{
    read_image,
    write_image,
};

use crate::param::channel_flags::ChannelFlags;

use super::common::{
    INPUT_ARG_NAME,
    OUTPUT_ARG_NAME,
    input_arg,
    output_arg,
};

pub const CMD_NAME: &str = "blur";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME)
        .arg(input_arg())
        .arg(output_arg())
        .subcommand(
            Command::new("mean")
                .alias("average")
                .alias("avg")
                .about("apply mean blur")
                .arg(
                    arg!(-r --radius <radius> "kernel radius")
                        .default_value("2")
                        .value_parser(value_parser!(usize)),
                )
                .arg(
                    arg!(-f --flags <flags> "channel flags in format [R][G][B][A]")
                        .default_value("RGB")
                        .value_parser(ChannelFlags::from_str),
                ),
        )
        .subcommand(
            Command::new("gaussian")
                .alias("gauss")
                .about("apply gaussian blur")
                .arg(
                    arg!(-r --radius <radius> "kernel radius")
                        .default_value("2")
                        .value_parser(value_parser!(usize)),
                )
                .arg(
                    arg!(-s --sigma <sigma> "sigma value")
                        .default_value("3")
                        .value_parser(value_parser!(f32)),
                )
                .arg(
                    arg!(-f --flags <flags> "channel flags in format [R][G][B][A]")
                        .default_value("RGB")
                        .value_parser(ChannelFlags::from_str),
                ),
        )
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    let image = match matches.subcommand().unwrap() {
        ("mean" | "average" | "avg", m) => apply_mean(&image, m)?,
        ("gaussian" | "gauss", m) => apply_gauss(&image, m)?,
        _ => unreachable!(),
    };
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}

fn apply_mean(image: &Image, matches: &ArgMatches) -> anyhow::Result<Image> {
    let target_radius = matches.get_one::<usize>("radius").unwrap();
    let channel_flags = *matches.get_one::<ChannelFlags>("flags").unwrap();
    Ok(mean_blur(image, *target_radius, channel_flags.into())?)
}

fn apply_gauss(image: &Image, matches: &ArgMatches) -> anyhow::Result<Image> {
    let target_radius = matches.get_one::<usize>("radius").unwrap();
    let sigma = matches.get_one::<f32>("sigma").unwrap();
    let channel_flags = *matches.get_one::<ChannelFlags>("flags").unwrap();
    Ok(gaussian_blur(image, *target_radius, *sigma, channel_flags.into())?)
}
