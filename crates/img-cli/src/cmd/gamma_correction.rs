use std::path::PathBuf;

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
    param::{
        channel_flags::{
            self,
            ChannelFlags,
        },
        input,
        output,
    },
};

pub const CMD_NAME: &str = "gamma-correction";

pub fn subcommand() -> Command {
    #[cfg(not(feature = "parallel"))]
    {
        Command::new(CMD_NAME)
            .arg(input::arg())
            .arg(output::arg())
            .arg(
                arg!(-g --gamma <gamma> "gamma value to use in the filter")
                    .required(true)
                    .value_parser(value_parser!(f32)),
            )
            .arg(channel_flags::arg())
    }

    #[cfg(feature = "parallel")]
    {
        use crate::param::threads;

        Command::new(CMD_NAME)
            .arg(input::arg())
            .arg(output::arg())
            .arg(
                arg!(-g --gamma <gamma> "gamma value to use in the filter")
                    .required(true)
                    .value_parser(value_parser!(f32)),
            )
            .arg(channel_flags::arg())
            .arg(threads::arg())
    }
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(input::ARG_NAME).unwrap())?;
    let gamma = matches.get_one::<f32>("gamma").unwrap();
    let channel_flags = *matches.get_one::<ChannelFlags>(channel_flags::ARG_NAME).unwrap();

    #[cfg(not(feature = "parallel"))]
    let image = gamma_correction(&image, *gamma, channel_flags.into());

    #[cfg(feature = "parallel")]
    let image = {
        use crate::param::threads::{
            self,
            Threads,
        };

        let threads = matches.get_one::<Threads>(threads::ARG_NAME).unwrap();
        gamma_correction_par(&image, threads.number(), *gamma, channel_flags.into())
    };

    write_image(&image, matches.get_one::<PathBuf>(output::ARG_NAME).unwrap())?;
    Ok(())
}
