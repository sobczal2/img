use std::{
    path::PathBuf,
    str::FromStr,
};

use clap::{
    ArgMatches,
    Command,
    arg,
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
        size::Size,
    },
};

pub const CMD_NAME: &str = "resize";

pub fn subcommand() -> Command {
    #[cfg(not(feature = "parallel"))]
    {
        Command::new(CMD_NAME)
            .arg(input::arg())
            .arg(output::arg())
            .arg(arg!(-s --size <size> "target size").required(true).value_parser(Size::from_str))
    }

    #[cfg(feature = "parallel")]
    {
        use crate::param::threads;

        Command::new(CMD_NAME)
            .arg(input::arg())
            .arg(output::arg())
            .arg(arg!(-s --size <size> "target size").required(true).value_parser(Size::from_str))
            .arg(threads::arg())
    }
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(input::ARG_NAME).unwrap())?;
    let target_size = matches.get_one::<Size>("size").unwrap();
    let scale = Scale::new(
        target_size.width as f32 / image.size().width() as f32,
        target_size.height as f32 / image.size().height() as f32,
    )?;

    #[cfg(not(feature = "parallel"))]
    let image = resize(&image, scale)?;

    #[cfg(feature = "parallel")]
    let image = {
        use crate::param::threads::{
            self,
            Threads,
        };

        let threads = matches.get_one::<Threads>(threads::ARG_NAME).unwrap();
        resize_par(&image, threads.number(), scale)?
    };

    write_image(&image, matches.get_one::<PathBuf>(output::ARG_NAME).unwrap())?;
    Ok(())
}
