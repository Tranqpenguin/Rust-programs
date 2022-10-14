use std::env;
use array2::Array2;
use csc411_image::{Read, GrayImage};

pub fn main() {
    let filename = env::args().nth(1);
    let img = GrayImage::read(filename.as_deref()).unwrap();
    let width : usize = img.width as usize;
    let height : usize = img.height as usize;
    let mut charz: Vec<u16> = Vec::new();
    let mut num1: Vec<u16> = Vec::new();
    let mut num2: Vec<u16> = Vec::new();
    let mut num3: Vec<u16> = Vec::new();
    let a2 = Array2::from_row_major(width, height, img.pixels);
    let mut z = 0;
    for row_iterator in a2.iter_row_major(){
        checker(&mut charz, row_iterator);
        if (z % 9) < 3{
            checker(&mut num1, row_iterator);
        }
        else if (z % 9) < 6{
            checker(&mut num2, row_iterator);
        }
        else{
            checker(&mut num3, row_iterator)
        }
        z +=1;
    }
    for col_iterator in a2.iter_col_major(){
        checker(&mut charz, col_iterator);
    }
    pub fn checker(output: &mut Vec<u16>, (_cur_row,_cur_col, cur_pixel) : (usize, usize, &csc411_image::Gray)){
        if !output.contains(&cur_pixel.value){
            output.push(cur_pixel.value);
        }
        else{
            std::process::exit(1);
        }
        if output.len() == 9{
            output.clear();
        }
    }
}