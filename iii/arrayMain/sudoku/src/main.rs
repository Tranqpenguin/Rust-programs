use std::env;
use array2::Array2;
use csc411_image::{Read, GrayImage};

pub fn main() {
    let filename = env::args().nth(1);
    let img = GrayImage::read(filename.as_deref()).unwrap();
    let width : usize = img.width as usize;
    let height : usize = img.height as usize;

    let a2 = Array2::from_row_major(width, height, img.pixels);

    let mut line: Vec<u16> = Vec::new();
    let mut b1: Vec<u16> = Vec::new();
    let mut b2: Vec<u16> = Vec::new();
    let mut b3: Vec<u16> = Vec::new();

    let mut i = 0;

    for iter_row in a2.as_ref().expect("REASON").iter_row_major(){
        sudoku_check(&mut line, iter_row);
        if (i % 9) < 3{
            sudoku_check(&mut b1, iter_row);
        }
        else if (i % 9) < 6{
            sudoku_check(&mut b2, iter_row);
        }
        else{
            sudoku_check(&mut b3, iter_row)
        }
        i+=1;
    }

    for iter_col in a2.expect("REASON").iter_col_major(){
        sudoku_check(&mut line, iter_col);
    }

    pub fn sudoku_check(segment : &mut Vec<u16>, (_cur_row,_cur_col, cur_pixel) : (usize, usize, &csc411_image::Gray)){
        if !segment.contains(&cur_pixel.value){
            segment.push(cur_pixel.value);
        }
        else{
            std::process::exit(1);
        }
        if segment.len() == 9{
            segment.clear();
        }
    }
}