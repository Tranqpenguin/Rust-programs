// I started the Lab to late, this is how far I got, almost got it but i have not a single clue how to do that given equation.
#![allow(unused)]
use array2::Array2;
use clap::Parser;
use csc411_image::{Read, RgbImage, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {

    /// The path to the file to read
    pub path : Option<String>,
    pub path2 : Option<String>
}

fn main() {
    let args = Cli::parse();
    let file_name : Option<String> = args.path;
    let file_name_two : Option<String> = args.path2;

    let image_one = RgbImage::read(file_name.as_deref()).unwrap();
    let mut row_1 = image_one.height as usize;
    let mut col_1 = image_one.width as usize;
    if !row_1 % 2 == 0{
        row_1 = row_1 - 1 as usize;
    }
    if !col_1 % 2 == 0{
        col_1 = col_1 - 1 as usize;
    }
    let a1 = Array2::from_row_major(row_1,col_1,image_one.pixels).unwrap();

    let image_two = RgbImage::read(file_name.as_deref()).unwrap();
    let mut row_2 = image_two.height as usize;
    let mut col_2 = image_two.width as usize;
    if !row_2 % 2 == 0{
        row_2 = row_2 - 1 as usize;
        if row_2 > row_1{
            row_2 = image_one.width as usize - 1;
        }
    }
    if !col_2 % 2 == 0{
        col_2 = col_2 - 1 as usize;
        if col_2 > col_1{
            col_2 = image_one.height as usize - 1;
        }
    }
    let a2 = Array2::from_row_major(row_2,col_2,image_two.pixels);

    //------------------------------------------------------------------------------------------------------------------------------
    let mut out_image_one = Array2::new(row_1, col_1, csc411_image::Rgb {red: 0, green: 0, blue: 0});
    for pixel in a1.iter_row_major() {
        let iterator_col = pixel.0;
        let iterator_row = pixel.1;
        let mut transform = out_image_one.get_mut(iterator_row, iterator_col).unwrap();
        transform.red = pixel.2.red;
        transform.green = pixel.2.green;
        transform.blue = pixel.2.blue;
    }
    //print_out(out_image_one, image_one.denominator);

    //let mut out_image_two = Array2::new(row, col, csc411_image::Rgb {red: 0, green: 0, blue: 0});

}





