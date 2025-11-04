use std::{
    fs::File,
    path::Path,
};

use anyhow::bail;
use img::{
    io::{
        jpeg::{
            ReadJpeg,
            WriteJpeg,
        },
        png::{
            ReadPng,
            WritePng,
        },
    },
    prelude::Image,
};

/// Read an image from png file specified in path
pub fn read_image(path: impl AsRef<Path>) -> anyhow::Result<Image> {
    let path = path.as_ref();
    let extension = path.extension().ok_or(anyhow::anyhow!("No file extension found"))?;
    let file = File::open(path)?;

    let image = match extension.to_string_lossy().as_ref() {
        "png" => Image::read_png(file)?,
        "jpg" | "jpeg" => Image::read_jpeg(file)?,
        _ => bail!("Invalid file - supported files are png and jpeg"),
    };

    Ok(image)
}

/// Write an image to png file specified in path
pub fn write_image(image: &Image, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let path = path.as_ref();
    let extension = path.extension().ok_or(anyhow::anyhow!("No file extension found"))?;

    match extension.to_string_lossy().as_ref() {
        "png" => {
            let file = File::create(path)?;
            image.write_png(file)?;
        }
        "jpg" | "jpeg" => {
            let file = File::create(path)?;
            image.write_jpeg(file, Default::default(), Default::default())?;
        }
        _ => bail!("Invalid file - supported files are png and jpeg"),
    };

    Ok(())
}
