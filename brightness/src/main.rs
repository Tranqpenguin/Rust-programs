use csc411_image::{Read, GrayImage};
use std::env;


fn main() {
    let input = env::args().nth(1);
    assert!(env::args().len() == 2);
    let img = GrayImage::read(input.as_deref()).unwrap();
    for pixel in img.pixels {
        println!("{:?} pixels", pixel);
    }
}