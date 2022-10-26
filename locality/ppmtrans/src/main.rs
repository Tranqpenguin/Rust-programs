#![allow(unused)]
mod rotate;
mod cli;
pub use array2::Array2;
use csc411_image::{Read,RgbImage,Write};
use std::env;
use crate::cli::Cli;
use clap::Parser;
use crate::rotate::{horizontal_rotation, rotation_0, rotation_180,
                    rotation_270, rotation_90, trans, vertical_rotation};

fn main() {
    let args = Cli::parse();
    let file_name : Option<String> = args.path;
    let rotate = args.rotate;
    let _flip = args.flip;
    let _transpose = args.transpose;
    let mut col_maj = args.col_major_arg;

    if col_maj{
        col_maj = false;
    }
    if rotate == Some(0){
        rotation_0(file_name.as_deref(),col_maj);
    }
    if rotate == Some(90){
        rotation_90(file_name.as_deref(),col_maj);
    }
    if rotate == Some(180){
        rotation_180(file_name.as_deref(),col_maj);
    }
    if rotate == Some(270){
        rotation_270(file_name.as_deref(),col_maj)
    }
    if _flip == Some("vertical".to_string()){
        horizontal_rotation(file_name.as_deref(), col_maj)
    }
    if _flip == Some("horizontal".to_string()){
        vertical_rotation(file_name.as_deref(), col_maj)
    }
    if _transpose{
        trans(file_name.as_deref(),col_maj)
    }
    else{
        std::process::exit(1);
    }
}