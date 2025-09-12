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
        input,
        output,
    },
};

pub const CMD_NAME: &str = "canny";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME).arg(input::arg()).arg(output::arg())
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(input::ARG_NAME).unwrap())?;
    let image = canny(&image);
    write_image(&image, matches.get_one::<PathBuf>(output::ARG_NAME).unwrap())?;
    Ok(())
}
