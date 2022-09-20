use csc411_image::{Read, GrayImage};
use std::env;


fn main() {
    //let mut count = 1;
    let input = env::args().nth(1);
    assert!(env::args().len() == 2);
    let img = GrayImage::read(input.as_deref()).unwrap();
    let mut denom = img.denominator as u32;
    for pixel in img.pixels {
        let a = pixel.value;

        println!("{:?}",a);
    }

}