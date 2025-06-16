use colored::Colorize;

pub fn print_error(message: impl AsRef<str>) {
    eprintln!("{} {}", "error:".red(), message.as_ref());
}
