use std::{
    path::PathBuf,
    str::FromStr,
};

use clap::{
    ArgMatches,
    Command,
    arg,
};
use img::{
    operation::geometry::crop::crop,
    primitive::{
        margin::Margin,
        size::Size,
    },
};

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

    let old_size = image.size();
    let new_size: Size = target_size_offset.size.try_into()?;
    let offset = target_size_offset.offset;

    let margin = Margin::new(
        offset.height,
        old_size.width() - new_size.width() - offset.width,
        old_size.height() - new_size.height() - offset.height,
        offset.width,
    );

    let image = crop(&image, margin)?;
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}
