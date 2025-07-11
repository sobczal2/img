use std::str::FromStr;

use anyhow::anyhow;
use anyhow::bail;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl FromStr for Size {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('x').collect();
        if parts.len() != 2 {
            bail!("size must be in [width]x[height] format");
        }

        let width = parts[0]
            .parse::<usize>()
            .map_err(|_| anyhow!("invalid width"))?;
        let height = parts[1]
            .parse::<usize>()
            .map_err(|_| anyhow!("invalid width"))?;

        Ok(Size { width, height })
    }
}

impl From<Size> for (usize, usize) {
    fn from(value: Size) -> Self {
        (value.width, value.height)
    }
}
