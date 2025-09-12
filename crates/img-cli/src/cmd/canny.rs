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
    #[cfg(not(feature = "parallel"))]
    {
        Command::new(CMD_NAME).arg(input::arg()).arg(output::arg())
    }

    #[cfg(feature = "parallel")]
    {
        use crate::param::threads;

        Command::new(CMD_NAME).arg(input::arg()).arg(output::arg()).arg(threads::arg())
    }
}

pub fn action(matches: &ArgMatches) -> anyhow::Result<()> {
    let image = read_image(matches.get_one::<PathBuf>(input::ARG_NAME).unwrap())?;
    let image = canny(&image);

    #[cfg(not(feature = "parallel"))]
    let image = canny(&image);

    #[cfg(feature = "parallel")]
    let image = {
        use crate::param::threads::{
            self,
            Threads,
        };

        let threads = matches.get_one::<Threads>(threads::ARG_NAME).unwrap();
        canny_par(&image, threads.number())
    };

    write_image(&image, matches.get_one::<PathBuf>(output::ARG_NAME).unwrap())?;
    Ok(())
}
