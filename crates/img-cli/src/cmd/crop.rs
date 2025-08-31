use std::{
    path::PathBuf,
    str::FromStr,
};

use clap::{
    ArgMatches,
    Command,
    arg,
};
use img::operation::geometry::crop::crop;

use crate::{
    io::{
        read_image,
        write_image,
    },
    param::size_offset::SizeOffset,
};

use super::common::{
    INPUT_ARG_NAME,
    OUTPUT_ARG_NAME,
    input_arg,
    output_arg,
};

pub const CMD_NAME: &str = "crop";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME).arg(input_arg()).arg(output_arg()).arg(
        arg!(-s --size <size_offset> "target size with offset")
            .required(true)
            .value_parser(SizeOffset::from_str),
    )
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    let target_size_offset = matches.get_one::<SizeOffset>("size").unwrap();

    let size = target_size_offset.size.try_into()?;
    let offset = target_size_offset.offset.into();

    let image = crop(&image, size, offset)?;
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}
