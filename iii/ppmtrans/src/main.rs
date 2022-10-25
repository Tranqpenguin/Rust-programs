mod rotate;
mod cli;
use crate::cli::Cli;
use clap::Parser;
use csc411_image::{RgbImage, Read};
use array2::Array2;
use crate::rotate::{rotate_180, rotate_270, rotate_90}; //rotate_h, rotate_t, rotate_v};

fn main() {
    // This takes all the arguments and put them in their proper member location
    let args = Cli::parse();
    let rotate = args.rotate;
    let _flip = args.flip;
    let _transpose = args.transpose;
    // this allows the variable to be seen at an option string from the CLI
    let filename : Option<String> = args.path;
    let img = RgbImage::read(filename.as_deref()).unwrap();
    let width : usize = img.width as usize;
    let height : usize = img.height as usize;
    let a2 = Array2::from_row_major(width, height, img.pixels).unwrap();
    let mut output = Vec::new();
    if rotate == Some(180){
        output = rotate_180(&a2, output).to_vec();
        for a in output{
            println!("{:?}",a);
        }
    }
    else if rotate == Some(90){
        output = rotate_90(&a2, output).to_vec();
        for a in output{
            println!("{:?}",a);
        }
    }
    else if rotate == Some(270){
        output = rotate_270(&a2, output).to_vec();
        for a in output{
            println!("{:?}",a);
        }
    }
    // else if flip{
    //     if rotate == "rotateh"{
    //         output = rotate_h(&a2).to_vec();
    //         for a in output{
    //             println!("{:?}",a);
    //         }
    //     }
    //     else if rotate == "rotatev"{
    //         output = rotate_v(&a2).to_vec();
    //         for a in output{
    //             println!("{:?}",a);
    //         }
    //     }
    // }
    // if transpose{
    //     output = rotate_t(&a2).to_vec();
    //     for a in output{
    //         println!("{:?}",a);
    //     }
    // }
    else{
        let no = "NO";
        println!("{}", no);
    }
}


