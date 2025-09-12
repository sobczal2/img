use std::{
    num::{
        NonZero,
        NonZeroUsize,
    },
    str::FromStr,
};

use clap::Arg;

pub const ARG_NAME: &str = "threads";
pub fn arg() -> Arg {
    clap::arg!(-t --threads <threads> "number of threads to use")
        .default_value("auto")
        .value_parser(Threads::from_str)
}

#[derive(Debug, Clone)]
pub enum Threads {
    Auto,
    Number(NonZeroUsize),
}

impl Threads {
    pub fn number(&self) -> NonZeroUsize {
        match self {
            Threads::Auto => NonZero::new(num_cpus::get()).unwrap(),
            Threads::Number(number) => *number,
        }
    }
}

impl FromStr for Threads {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Threads::Auto),
            s => Ok(s.parse::<NonZeroUsize>().map(Threads::Number).map_err(|_| {
                anyhow::anyhow!("invalid threads format, must be \"auto\" or positive number")
            })?),
        }
    }
}
