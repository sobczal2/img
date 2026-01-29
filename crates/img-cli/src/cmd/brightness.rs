use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};
#[cfg(not(feature = "parallel"))]
use img::operation::color::brightness;
#[cfg(feature = "parallel")]
use img::operation::color::brightness_par;

use crate::{
    io::{read_image, write_image},
    param::{
        channel_flags::{self, ChannelFlags},
        input, output,
    },
};

pub const CMD_NAME: &str = "brightness";

const FACTOR_ARG_NAME: &str = "factor";

fn factor_arg() -> Arg {
    Arg::new(FACTOR_ARG_NAME)
        .short('f')
        .long("factor")
        .help("Brightness multiplier (>1.0 = brighter, <1.0 = darker)")
        .required(true)
        .value_parser(clap::value_parser!(f32))
}

/// Create the brightness subcommand for the CLI.
///
/// This command adjusts the brightness of an image by multiplying
/// pixel values by a configurable factor.
pub fn subcommand() -> Command {
    #[cfg(not(feature = "parallel"))]
    {
        Command::new(CMD_NAME)
            .about("Adjust image brightness by a factor")
            .arg(input::arg())
            .arg(output::arg())
            .arg(factor_arg())
            .arg(channel_flags::arg())
    }
    #[cfg(feature = "parallel")]
    {
        use crate::param::threads;

        Command::new(CMD_NAME)
            .about("Adjust image brightness by a factor")
            .arg(input::arg())
            .arg(output::arg())
            .arg(factor_arg())
            .arg(channel_flags::arg())
            .arg(threads::arg())
    }
}

/// Execute the brightness command.
///
/// Reads the input image, applies brightness adjustment with the
/// specified factor, and writes the result to the output path.
pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(input::ARG_NAME).unwrap())?;
    let channel_flags = *matches.get_one::<ChannelFlags>(channel_flags::ARG_NAME).unwrap();
    let factor = *matches.get_one::<f32>(FACTOR_ARG_NAME).unwrap();

    #[cfg(not(feature = "parallel"))]
    let image = brightness(&image, factor, channel_flags.into());

    #[cfg(feature = "parallel")]
    let image = {
        use crate::param::threads::{self, Threads};

        let threads = matches.get_one::<Threads>(threads::ARG_NAME).unwrap();
        brightness_par(&image, threads.number(), factor, channel_flags.into())
    };

    write_image(&image, matches.get_one::<PathBuf>(output::ARG_NAME).unwrap())?;
    Ok(())
}
