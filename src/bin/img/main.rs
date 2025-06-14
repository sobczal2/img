mod cmd;
mod io;
mod param;
mod printing;

use std::process::exit;

use clap::{command, Command};
use cmd::{blur, crop, grayscale, resize};
use printing::print_error;

fn main() {
    let command = command!()
        .subcommand_required(true)
        .subcommand(grayscale::subcommand())
        .subcommand(resize::subcommand())
        .subcommand(crop::subcommand())
        .subcommand(blur::subcommand());

    if let Err(e) = execute_command(command) {
        print_error(e.to_string());
        exit(1);
    }
}

fn execute_command(command: Command) -> anyhow::Result<()> {
    let matches = command.get_matches();
    match matches.subcommand() {
        Some((grayscale::CMD_NAME, m)) => grayscale::action(m),
        Some((resize::CMD_NAME, m)) => resize::action(m),
        Some((crop::CMD_NAME, m)) => crop::action(m),
        Some((blur::CMD_NAME, m)) => blur::action(m),
        _ => unreachable!(),
    }
}
