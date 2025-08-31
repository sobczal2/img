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
    operation::geometry::resize::resize,
    primitive::scale::Scale,
};

use crate::{
    io::{
        read_image,
        write_image,
    },
    param::size::Size,
};

use super::common::{
    INPUT_ARG_NAME,
    OUTPUT_ARG_NAME,
    input_arg,
    output_arg,
};

pub const CMD_NAME: &str = "resize";

pub fn subcommand() -> Command {
    Command::new(CMD_NAME)
        .arg(input_arg())
        .arg(output_arg())
        .arg(arg!(-s --size <size> "target size").required(true).value_parser(Size::from_str))
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(INPUT_ARG_NAME).unwrap())?;
    let target_size = matches.get_one::<Size>("size").unwrap();
    let scale = Scale::new(
        target_size.width as f32 / image.size().width() as f32,
        target_size.height as f32 / image.size().height() as f32,
    )?;
    let image = resize(&image, scale)?;
    write_image(&image, matches.get_one::<PathBuf>(OUTPUT_ARG_NAME).unwrap())?;
    Ok(())
}
