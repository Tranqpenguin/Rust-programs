use csc411_image::{Read, GrayImage};
use std::env;


fn main() {
    let input = env::args().nth(1);
    assert!(env::args().len() == 2);
    let img = GrayImage::read(input.as_deref()).unwrap();
    let mut denom = img.denominator as u32;
    let mut sum = 0 as f32;
    let mut counter = 1 as i128;
    for pixel in img.pixels {
        let a = pixel.value as f32 / denom as f32;
        sum = a as f32 + sum as f32;
        counter = counter as i128 + 1 as i128;
    }
    let avg = sum as f32 / counter as f32;
    println!("{:.3}",avg);



    

    

}