#![allow(unused)]

mod rotate;
pub use array2::Array2;
use csc411_image::{Read,RgbImage,Write};
use std::env;
use crate::rotate::{horiz, rot_0, rot_180, rot_270, rot_90, trans, vert};
// use clap::Parser;

// /// Search for a pattern in a file and display the lines that contain it.
// #[derive(Parser)]
// struct Cli {
//     /// The pattern to look for
//     pattern: String,
//     /// The path to the file to read
//     #[clap(parse(from_os_str))]
//     path: std::path::PathBuf,
// }

fn main() {
    //let args = Cli::parse();
    let mut arguments = Vec::new();
    let mut i = 0;
    let mut name = 0;
    for arg_counts in env::args() {
        arguments.push(arg_counts.clone());
        if (arg_counts.chars().nth(0).unwrap() != "-".chars().nth(0).unwrap())
            && (arg_counts.chars().nth(0).unwrap() != "0".chars().nth(0).unwrap())
            && (arg_counts.chars().nth(0).unwrap() != "9".chars().nth(0).unwrap())
            && (arg_counts.chars().nth(0).unwrap() != "1".chars().nth(0).unwrap())
            && (arg_counts.chars().nth(0).unwrap() != "2".chars().nth(0).unwrap()){
            if(arg_counts != "horizontal")&&(arg_counts != "vertical")&&(arg_counts != "transpose"){
                name = i;
            }
        }
        i += 1;
    }
    let file_name = env::args().nth(name);

    let mut col_maj = true;
    if arguments.contains(&"-block-major".to_string()){
        std::process::exit(1);
    }
    else if arguments.contains(&"-row-major".to_string()){
        col_maj = false;
    }

    // let image = RgbImage::read(file_name).unwrap();
    // let row = image.height as usize;
    // let col = image.width as usize;
    // let original = Array2::from_row_major(row,col,image.pixels).unwrap();


    if arguments.contains(&"0".to_string()){
        rot_0(file_name.as_deref(),col_maj);
    }
    if arguments.contains(&"90".to_string()){
        rot_90(file_name.as_deref(),col_maj);
    }
    if arguments.contains(&"180".to_string()){
        rot_180(file_name.as_deref(),col_maj);
    }
    if arguments.contains(&"270".to_string()){
        rot_270(file_name.as_deref(),col_maj)
    }
    if arguments.contains(&"vertical".to_string()){
        horiz(file_name.as_deref(),col_maj)
    }
    if arguments.contains(&"horizontal".to_string()){
        vert(file_name.as_deref(),col_maj)
    }
    if arguments.contains(&"transpose".to_string()){
        trans(file_name.as_deref(),col_maj)
    }
    else{
        std::process::exit(1);
    }
//, image, row, col, original
}