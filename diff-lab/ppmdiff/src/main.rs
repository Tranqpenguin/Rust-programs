use array2::Array2;
use clap::Parser;
use csc411_image::{Read, RgbImage};

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
    let image_one = RgbImage::read(file_name).unwrap();

}
