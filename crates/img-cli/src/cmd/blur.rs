use std::path::PathBuf;

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

use crate::param::{
    channel_flags::{
        self,
        ChannelFlags,
    },
    input,
    output,
};

pub const CMD_NAME: &str = "blur";

const MEAN_CMD_NAME: &str = "mean";
const MEAN_CMD_ALIAS1: &str = "average";
const MEAN_CMD_ALIAS2: &str = "avg";

const GAUSSIAN_CMD_NAME: &str = "gaussian";
const GAUSSIAN_CMD_ALIAS1: &str = "gauss";

fn mean_subcommand() -> Command {
    #[cfg(not(feature = "parallel"))]
    {
        Command::new(MEAN_CMD_NAME)
            .alias(MEAN_CMD_ALIAS1)
            .alias(MEAN_CMD_ALIAS2)
            .about("apply mean blur")
            .arg(
                arg!(-r --radius <radius> "kernel radius")
                    .default_value("2")
                    .value_parser(value_parser!(usize)),
            )
            .arg(channel_flags::arg())
    }

    #[cfg(feature = "parallel")]
    {
        use crate::param::threads;

        Command::new(MEAN_CMD_NAME)
            .alias(MEAN_CMD_ALIAS1)
            .alias(MEAN_CMD_ALIAS2)
            .about("apply mean blur")
            .arg(
                arg!(-r --radius <radius> "kernel radius")
                    .default_value("2")
                    .value_parser(value_parser!(usize)),
            )
            .arg(channel_flags::arg())
            .arg(threads::arg())
    }
}

fn gaussian_subcommand() -> Command {
    #[cfg(not(feature = "parallel"))]
    {
        Command::new(GAUSSIAN_CMD_NAME)
            .alias(GAUSSIAN_CMD_ALIAS1)
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
            .arg(channel_flags::arg())
    }

    #[cfg(feature = "parallel")]
    {
        use crate::param::threads;

        Command::new(GAUSSIAN_CMD_NAME)
            .alias(GAUSSIAN_CMD_ALIAS1)
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
            .arg(channel_flags::arg())
            .arg(threads::arg())
    }
}

pub fn subcommand() -> Command {
    Command::new(CMD_NAME)
        .arg(input::arg())
        .arg(output::arg())
        .subcommand(mean_subcommand())
        .subcommand(gaussian_subcommand())
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(input::ARG_NAME).unwrap())?;
    let image = match matches.subcommand().ok_or(anyhow::anyhow!("no subcommand provided"))? {
        (MEAN_CMD_NAME | MEAN_CMD_ALIAS1 | MEAN_CMD_ALIAS2, m) => apply_mean(&image, m)?,
        (GAUSSIAN_CMD_NAME | GAUSSIAN_CMD_ALIAS1, m) => apply_gauss(&image, m)?,
        _ => unreachable!(),
    };
    write_image(&image, matches.get_one::<PathBuf>(output::ARG_NAME).unwrap())?;
    Ok(())
}

fn apply_mean(image: &Image, matches: &ArgMatches) -> anyhow::Result<Image> {
    let target_radius = matches.get_one::<usize>("radius").unwrap();
    let channel_flags = *matches.get_one::<ChannelFlags>(channel_flags::ARG_NAME).unwrap();

    #[cfg(not(feature = "parallel"))]
    let image = mean_blur(image, *target_radius, channel_flags.into())?;

    #[cfg(feature = "parallel")]
    let image = {
        use crate::param::threads::{
            self,
            Threads,
        };

        let threads = matches.get_one::<Threads>(threads::ARG_NAME).unwrap();
        mean_blur_par(image, threads.number(), *target_radius, channel_flags.into())?
    };

    Ok(image)
}

fn apply_gauss(image: &Image, matches: &ArgMatches) -> anyhow::Result<Image> {
    let target_radius = matches.get_one::<usize>("radius").unwrap();
    let sigma = matches.get_one::<f32>("sigma").unwrap();
    let channel_flags = *matches.get_one::<ChannelFlags>("flags").unwrap();

    #[cfg(not(feature = "parallel"))]
    let image = gaussian_blur(image, *target_radius, *sigma, channel_flags.into())?;

    #[cfg(feature = "parallel")]
    let image = {
        use crate::param::threads::{
            self,
            Threads,
        };

        let threads = matches.get_one::<Threads>(threads::ARG_NAME).unwrap();
        gaussian_blur_par(image, threads.number(), *target_radius, *sigma, channel_flags.into())?
    };

    Ok(image)
}
