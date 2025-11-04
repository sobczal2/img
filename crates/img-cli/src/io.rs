use std::path::Path;

use anyhow::bail;
use img::prelude::Image;

#[cfg(feature = "jpeg")]
use img::io::jpeg::{
    ReadJpeg,
    WriteJpeg,
};

#[cfg(feature = "png")]
use img::io::png::{
    ReadPng,
    WritePng,
};

/// Read an image from a file specified in path (supports png or/and jpeg based on enabled features)
#[cfg(any(feature = "png", feature = "jpeg"))]
pub fn read_image(path: impl AsRef<Path>) -> anyhow::Result<Image> {
    use std::fs::File;
    let path = path.as_ref();
    let extension = path.extension().ok_or(anyhow::anyhow!("No file extension found"))?;
    let file = File::open(path)?;

    let image = match extension.to_string_lossy().as_ref() {
        #[cfg(feature = "png")]
        "png" => Image::read_png(file)?,
        #[cfg(feature = "jpeg")]
        "jpg" | "jpeg" => Image::read_jpeg(file)?,
        _ => bail!("Invalid file - unsupported file format"),
    };

    Ok(image)
}

#[cfg(not(any(feature = "png", feature = "jpeg")))]
pub fn read_image(_path: impl AsRef<Path>) -> anyhow::Result<Image> {
    bail!("No image format support compiled in (enable the `png` or `jpeg` feature)")
}

/// Write an image to a file specified in path (supports png or/and jpeg based on enabled features)
#[cfg(any(feature = "png", feature = "jpeg"))]
pub fn write_image(image: &Image, path: impl AsRef<Path>) -> anyhow::Result<()> {
    use std::fs::File;
    let path = path.as_ref();
    let extension = path.extension().ok_or(anyhow::anyhow!("No file extension found"))?;

    match extension.to_string_lossy().as_ref() {
        #[cfg(feature = "png")]
        "png" => {
            let file = File::create(path)?;
            image.write_png(file)?;
        }
        #[cfg(feature = "jpeg")]
        "jpg" | "jpeg" => {
            let file = File::create(path)?;
            image.write_jpeg(file, Default::default(), Default::default())?;
        }
        _ => bail!("Invalid file - unsupported file format"),
    };

    Ok(())
}

#[cfg(not(any(feature = "png", feature = "jpeg")))]
pub fn write_image(_image: &Image, _path: impl AsRef<Path>) -> anyhow::Result<()> {
    bail!("No image format support compiled in (enable the `png` or `jpeg` feature)")
}
