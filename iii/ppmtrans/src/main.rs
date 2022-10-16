use std::env;
use clap::Parser;
use csc411_image::{GrayImage, Read};
use array2::Array2;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let pattern = env::args().nth(1);
    let filename = env::args().nth(2);
    let img = GrayImage::read(filename.as_deref()).unwrap();
    let width : usize = img.width as usize;
    let height : usize = img.height as usize;
    let a2 = Array2::from_row_major(width, height, img.pixels);
    println!("{:?}", pattern);
    // if pattern == "col"{
    //     for a in a2.iter_col_major(){
    //         println!("{:?}", a);
    //     }
    // }
    // else{
    //     for a in a2.iter_row_major(){
    //         println!("{:?}", a);
    //     }
    // }

}