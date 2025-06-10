use std::{fs::File, path::Path, process::exit};

use img::{
    image::Image,
    io::{ReadPng, WritePng},
};

use crate::printing::print_error;

pub fn read_image(path: impl AsRef<Path>) -> Image {
    match read_image_inner(path) {
        Ok(image) => image,
        Err(message) => {
            print_error(message);
            exit(1);
        }
    }
}

fn read_image_inner(path: impl AsRef<Path>) -> Result<Image, Box<str>> {
    let file = File::open(path).map_err(|_| "can't open file")?;
    let image = Image::read_png(file).map_err(|_| "can't read file as png")?;
    if image.size().0 == 0 || image.size().1 == 0 {
        return Err("image width or height is 0".into());
    }
    Ok(image)
}

pub fn write_image(image: &Image, path: impl AsRef<Path>) {
    if let Err(message) = write_image_inner(image, path) {
        print_error(message);
        exit(1);
    }
}

fn write_image_inner(image: &Image, path: impl AsRef<Path>) -> Result<(), Box<str>> {
    let file = File::create(path).map_err(|_| "can't create file")?;
    Ok(image
        .write_png(file)
        .map_err(|_| "can't write file as png")?)
}
