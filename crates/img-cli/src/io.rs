use std::{
    fs::File,
    path::Path,
};

use img::{
    io::{
        png::{ReadPng, WritePng},
    },
    prelude::Image,
};

/// Read an image from png file specified in path
pub fn read_image(path: impl AsRef<Path>) -> anyhow::Result<Image> {
    let file = File::open(path)?;
    let image = Image::read_png(file)?;
    Ok(image)
}

/// Write an image to png file specified in path
pub fn write_image(image: &Image, path: impl AsRef<Path>) -> anyhow::Result<()> {
    let file = File::create(path)?;
    Ok(image.write_png(file)?)
}
