use std::path::PathBuf;

use clap::{ArgMatches, Command, ValueEnum, arg, value_parser};
use img::{image::Image, operation::blur::gaussian::gaussian_blur};

use crate::io::{read_image, write_image};

use super::common::{INPUT_ARG_NAME, OUTPUT_ARG_NAME, input_arg, output_arg};

pub const CMD_NAME: &str = "blur";

#[derive(ValueEnum, Clone)]
pub enum BlurAlgorithm {
    Mean,
    Gaussian,
    Median,
}

pub fn subcommand() -> Command {
    Command::new(CMD_NAME)
        .arg(input_arg())
        .arg(output_arg())
        .arg(
            arg!(-r --radius <radius> "kernel radius")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(-a --algorithm <algorithm> "blur algorith")
                .default_value("gaussian")
                .value_parser(value_parser!(BlurAlgorithm)),
        )
        .arg(
            arg!(-s --sigma <sigma> "sigma value")
                .default_value("3")
                .value_parser(value_parser!(f32)),
        )
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    let image = match matches.get_one::<BlurAlgorithm>("algorithm").unwrap() {
        BlurAlgorithm::Mean => apply_mean(&image, matches)?,
        BlurAlgorithm::Gaussian => apply_gaussian(&image, matches)?,
        BlurAlgorithm::Median => apply_median(&image, matches)?,
    };
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}

fn apply_mean(_image: &Image, matches: &ArgMatches) -> anyhow::Result<Image> {
    let _target_radius = matches.get_one::<usize>("radius").unwrap();
    // Ok(mean_blur(image, *target_radius)?)
    todo!()
}

fn apply_gaussian(image: &Image, matches: &ArgMatches) -> anyhow::Result<Image> {
    let target_radius = matches.get_one::<usize>("radius").unwrap();
    let sigma = matches.get_one::<f32>("sigma").unwrap();
    Ok(gaussian_blur(image, *target_radius, *sigma)?)
}

fn apply_median(_image: &Image, matches: &ArgMatches) -> anyhow::Result<Image> {
    let _target_radius = matches.get_one::<usize>("radius").unwrap();
    // Ok(median_blur(image, *target_radius)?)
    todo!()
}
