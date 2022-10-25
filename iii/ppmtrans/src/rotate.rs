use csc411_image::{Read, RgbImage, Write};
use array2::Array2;

//, image: Array2<csc411_image::Rgb>, row: usize, col: usize, original: Array2<T>
pub fn rot_0(file_name: Option<&str>,col_maj: bool){
    let image = RgbImage::read(file_name).unwrap();
    let row = image.height as usize;
    let col = image.width as usize;
    let original = Array2::from_row_major(col,row,image.pixels).unwrap();
    print_out(original,image.denominator);
}

pub fn rot_90(file_name: Option<&str>,col_maj: bool){
    let image = RgbImage::read(file_name).unwrap();
    let row = image.height as usize;
    let col = image.width as usize;
    let original = Array2::from_row_major(col,row,image.pixels).unwrap();
    let mut output_img = Array2::new(row,col,csc411_image::Rgb {red: 0, green: 0, blue: 0});


    if col_maj{
        for pixel in original.iter_col_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;

            let mut transform = output_img.get_mut(row - iter_row - 1, iter_col).unwrap();

            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in original.iter_row_major() {

            let iter_col = pixel.0;
            let iter_row = pixel.1;
            //println!("{:?} {:?}", row,iter_row);
            let mut transform = output_img.get_mut(row - iter_row - 1, iter_col).unwrap();
            //println!("{:?} {:?}", row,iter_row);
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image.denominator)
}

pub fn rot_180(file_name: Option<&str>,col_maj: bool){
    let image = RgbImage::read(file_name).unwrap();
    let row = image.height as usize;
    let col = image.width as usize;
    let original = Array2::from_row_major(col,row,image.pixels).unwrap();

    let mut output_img = Array2::new(col,row,csc411_image::Rgb {red: 0, green: 0, blue: 0});

    if col_maj{
        for pixel in original.iter_col_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;

            let mut transform = output_img.get_mut(col - iter_col - 1, row - iter_row - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in original.iter_row_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;
            let mut transform = output_img.get_mut(col - iter_col - 1, row - iter_row - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image.denominator);
}

pub fn rot_270(file_name: Option<&str>,col_maj: bool){
    let image = RgbImage::read(file_name).unwrap();
    let row = image.height as usize;
    let col = image.width as usize;
    let original = Array2::from_row_major(col,row,image.pixels).unwrap();


    let mut output_img = Array2::new(col,row,csc411_image
    ::Rgb {red: 0, green: 0, blue: 0});
    if col_maj{
        for pixel in original.iter_col_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;
            let mut transform = output_img.get_mut(iter_row, col - iter_col - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in original.iter_row_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;
            let mut transform = output_img.get_mut(iter_row, col - iter_col - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image.denominator);
}


pub fn horiz(file_name: Option<&str>,col_maj: bool){
    let image = RgbImage::read(file_name).unwrap();
    let row = image.height as usize;
    let col = image.width as usize;
    let original = Array2::from_row_major(col,row,image.pixels).unwrap();


    let mut output_img = Array2::new(col,row,csc411_image
    ::Rgb {red: 0, green: 0, blue: 0});
    if col_maj{
        for pixel in original.iter_col_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;
            let mut transform = output_img.get_mut(col - iter_col - 1, iter_row).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in original.iter_row_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;
            let mut transform = output_img.get_mut(col - iter_col - 1, iter_row).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image.denominator);
}

pub fn vert(file_name: Option<&str>,col_maj: bool){
    let image = RgbImage::read(file_name).unwrap();
    let row = image.height as usize;
    let col = image.width as usize;
    let original = Array2::from_row_major(col,row,image.pixels).unwrap();


    let mut output_img = Array2::new(col,row,csc411_image
    ::Rgb {red: 0, green: 0, blue: 0});
    if col_maj{
        for pixel in original.iter_col_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;
            let mut transform = output_img.get_mut(iter_col, row - iter_row - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in original.iter_row_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;
            let mut transform = output_img.get_mut(iter_col, row - iter_row - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image.denominator);
}

pub fn trans(file_name: Option<&str>,col_maj: bool){
    let image = RgbImage::read(file_name).unwrap();
    let row = image.height as usize;
    let col = image.width as usize;
    let original = Array2::from_row_major(col,row,image.pixels).unwrap();


    let mut output_img = Array2::new(col,row,csc411_image
    ::Rgb {red: 0, green: 0, blue: 0});
    if col_maj{
        for pixel in original.iter_col_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;
            let mut transform = output_img.get_mut(iter_row, iter_col).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in original.iter_row_major() {
            let iter_col = pixel.0;
            let iter_row = pixel.1;
            let mut transform = output_img.get_mut(iter_row, iter_col).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image.denominator);
}

pub fn print_out(img: Array2<csc411_image
::Rgb>, d: u16){
    let mut pixel_total = Vec::new();
    for pixel in img.iter_row_major(){
        pixel_total.push(pixel.2.clone());
    }
    let output = RgbImage{
        pixels: pixel_total,
        width: img.width() as u32,
        height: img.height() as u32,
        denominator: d,
    };
    output.write(None);
}