mod io;
mod printing;

use std::path::PathBuf;

use clap::{arg, command, value_parser, Arg, Command};
use img::ops::{color::grayscale::grayscale, geometry::resize::resize};
use io::{read_image, write_image};

fn input_arg() -> Arg {
    arg!(-i --input <file> "input png file")
        .required(true)
        .value_parser(value_parser!(PathBuf))
}

fn output_arg() -> Arg {
    arg!(-o --output <file> "output png file")
        .required(true)
        .value_parser(value_parser!(PathBuf))
}

fn main() {
    let command = command!()
        .subcommand_required(true)
        .subcommand(Command::new("grayscale").arg(input_arg()).arg(output_arg()))
        .subcommand(
            Command::new("resize")
                .arg(input_arg())
                .arg(output_arg())
                .arg(
                    arg!(-w --width <width> "target width")
                        .required(true)
                        .value_parser(value_parser!(usize)),
                )
                .arg(
                    arg!(-h --height <height> "target height")
                        .required(true)
                        .value_parser(value_parser!(usize)),
                ),
        );

    let matches = command.clone().get_matches();
    match matches.subcommand() {
        Some(("grayscale", m)) => {
            let mut image = read_image(m.get_one::<PathBuf>("input").unwrap());
            grayscale(&mut image);
            write_image(&image, m.get_one::<PathBuf>("output").unwrap());
        }
        Some(("resize", m)) => {
            let image = read_image(m.get_one::<PathBuf>("input").unwrap());
            let target_width = m.get_one::<usize>("width").unwrap();
            let target_height = m.get_one::<usize>("width").unwrap();
            let scale = (
                *target_width as f32 / image.size().0 as f32,
                *target_height as f32 / image.size().1 as f32,
            );
            let image = resize(&image, scale);
            write_image(&image, m.get_one::<PathBuf>("output").unwrap());
        }
        _ => unreachable!(),
    }
}
