use std::path::PathBuf;

use clap::{
    ArgMatches,
    Command,
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

pub const CMD_NAME: &str = "grayscale";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME).arg(input::arg()).arg(output::arg()).arg(channel_flags::arg())
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(input::ARG_NAME).unwrap())?;
    let channel_flags = *matches.get_one::<ChannelFlags>(channel_flags::ARG_NAME).unwrap();
    let image = grayscale(&image, channel_flags.into());
    write_image(&image, matches.get_one::<PathBuf>(output::ARG_NAME).unwrap())?;
    Ok(())
}
