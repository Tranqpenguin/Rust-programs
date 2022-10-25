mod rotate;
mod cli;
use crate::cli::Cli;
use clap::Parser;
use csc411_image::{GrayImage, Read};
use array2::Array2;
use crate::rotate::{rotate_180, rotate_270, rotate_90, rotate_h, rotate_t, rotate_v};

fn main() {
    // This takes all the arguments and put them in their proper member location
    let args = Cli::parse();
    // this allows the variable to be seen at an option string from the CLI
    let filename : Option<String> = Some(args.path);
    let pattern = args.pattern;
    let img = GrayImage::read(filename.as_deref()).unwrap();
    let width : usize = img.width as usize;
    let height : usize = img.height as usize;
    let a2 = Array2::from_row_major(width, height, img.pixels);
    let mut output = Vec::new();
    //just checking if i can use pattern properly
    if pattern == "rotate180"{
        output = rotate_180(&a2).to_vec();
        for a in output{
            println!("{:?}",a);
        }
    }
    else if pattern == "rotate90" {
        output = rotate_90(&a2).to_vec();
        for a in output{
            println!("{:?}",a);
        }
    }
    else if pattern == "rotate270"{
        output = rotate_270(&a2).to_vec();
        for a in output{
            println!("{:?}",a);
        }
    }
    else if pattern == "rotateh"{
        output = rotate_h(&a2).to_vec();
        for a in output{
            println!("{:?}",a);
        }
    }
    else if pattern == "rotatev"{
        output = rotate_v(&a2).to_vec();
        for a in output{
            println!("{:?}",a);
        }
    }
    else if pattern == "rotatet"{
        output = rotate_t(&a2).to_vec();
        for a in output{
            println!("{:?}",a);
        }
    }
    else{
        let no = "NO";
        println!("{}", no);
    }
}


