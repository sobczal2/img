use std::str::FromStr;

use anyhow::bail;
use clap::{
    Arg,
    arg,
};

pub const ARG_NAME: &str = "flags";
pub fn arg() -> Arg {
    arg!(-f --flags <flags> "channel flags in format [R][G][B][A]")
        .default_value("RGB")
        .value_parser(ChannelFlags::from_str)
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ChannelFlags {
    pub red: bool,
    pub green: bool,
    pub blue: bool,
    pub alpha: bool,
}

impl FromStr for ChannelFlags {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut flags = ChannelFlags::default();

        for ch in s.chars() {
            match ch {
                'R' => {
                    if flags.red {
                        bail!("Red channel set multiple times")
                    } else {
                        flags.red = true
                    }
                }
                'G' => {
                    if flags.green {
                        bail!("Green channel set multiple times")
                    } else {
                        flags.green = true
                    }
                }
                'B' => {
                    if flags.blue {
                        bail!("Blue channel set multiple times")
                    } else {
                        flags.blue = true
                    }
                }
                'A' => {
                    if flags.alpha {
                        bail!("Alpha channel set multiple times")
                    } else {
                        flags.alpha = true
                    }
                }
                _ => bail!("available channels are R(Red), G(Green), B(Blue) and A(Alpha)"),
            }
        }

        Ok(flags)
    }
}

impl From<ChannelFlags> for img::prelude::ChannelFlags {
    fn from(value: ChannelFlags) -> Self {
        let red = if value.red {
            img::prelude::ChannelFlags::RED
        } else {
            img::prelude::ChannelFlags::empty()
        };
        let green = if value.green {
            img::prelude::ChannelFlags::GREEN
        } else {
            img::prelude::ChannelFlags::empty()
        };
        let blue = if value.blue {
            img::prelude::ChannelFlags::BLUE
        } else {
            img::prelude::ChannelFlags::empty()
        };
        let alpha = if value.alpha {
            img::prelude::ChannelFlags::ALPHA
        } else {
            img::prelude::ChannelFlags::empty()
        };

        // SAFETY: `ChannelFlags::from_bits` only returns `Err` if the bits value
        // can't be represented, but here all values that it is constructed from
        // are valid.
        img::prelude::ChannelFlags::from_bits(
            red.bits() + green.bits() + blue.bits() + alpha.bits(),
        )
        .unwrap()
    }
}
