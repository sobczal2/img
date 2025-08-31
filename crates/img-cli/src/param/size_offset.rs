use std::str::FromStr;

use anyhow::{
    anyhow,
    bail,
};

use super::size::Size;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SizeOffset {
    pub size: Size,
    pub offset: Size,
}

impl FromStr for SizeOffset {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('+').collect();
        if parts.len() != 2 {
            bail!("size with offset must be in [width]x[height]+[offset_x]x[offset_y] format");
        }

        let size = parts[0].parse::<Size>().map_err(|_| anyhow!("invalid size"))?;
        let offset = parts[1].parse::<Size>().map_err(|_| anyhow!("invalid offset"))?;

        Ok(SizeOffset { size, offset })
    }
}
