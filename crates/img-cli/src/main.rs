mod cmd;
mod io;
mod param;
mod printing;

use std::process::exit;

use clap::{
    Command,
    command,
};
use cmd::{
    crop,
    gamma_correction,
    grayscale,
    resize,
    sepia,
};
use printing::print_error;

use crate::cmd::{
    blur,
    canny,
};

fn main() {
    let command = command!()
        .subcommand_required(true)
        .subcommand(grayscale::subcommand())
        .subcommand(sepia::subcommand())
        .subcommand(resize::subcommand())
        .subcommand(crop::subcommand())
        .subcommand(blur::subcommand())
        .subcommand(gamma_correction::subcommand())
        .subcommand(canny::subcommand());

    if let Err(e) = execute_command(command) {
        print_error(e.to_string());
        exit(1);
    }
}

fn execute_command(command: Command) -> anyhow::Result<()> {
    let matches = command.get_matches();
    match matches.subcommand() {
        Some((grayscale::CMD_NAME, m)) => grayscale::action(m),
        Some((sepia::CMD_NAME, m)) => sepia::action(m),
        Some((resize::CMD_NAME, m)) => resize::action(m),
        Some((crop::CMD_NAME, m)) => crop::action(m),
        Some((blur::CMD_NAME, m)) => blur::action(m),
        Some((gamma_correction::CMD_NAME, m)) => gamma_correction::action(m),
        Some((canny::CMD_NAME, m)) => canny::action(m),
        _ => unreachable!(),
    }
}
