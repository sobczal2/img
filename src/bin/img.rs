use std::fs::File;

use img::{
    image::Image,
    io::{ReadPng, WritePng},
    ops::color::grayscale::grayscale,
};

fn main() {
    let data = include_bytes!("../../assets/sunflower.png");
    let mut image = Image::read_png(&data[..]).unwrap();
    grayscale(&mut image);
    image.write_png(File::create("test.png").unwrap()).unwrap();
}
