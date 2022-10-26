use csc411_image::{Read, RgbImage, Write};
use array2::Array2;

pub fn rotation_0(file_name: Option<&str>,col_maj: bool){
    let image_input = RgbImage::read(file_name).unwrap();
    let row = image_input.height as usize;
    let col = image_input.width as usize;
    let a2 = Array2::from_row_major(col, row, image_input.pixels).unwrap();
    print_out(a2, image_input.denominator);
}

pub fn rotation_90(file_name: Option<&str>,col_maj: bool){
    let image_input = RgbImage::read(file_name).unwrap();
    let row = image_input.height as usize;
    let col = image_input.width as usize;
    let a2 = Array2::from_row_major(col, row, image_input.pixels).unwrap();
    let mut out_image = Array2::new(row, col, csc411_image::Rgb {red: 0, green: 0, blue: 0});


    if col_maj{
        for pixel in a2.iter_col_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;

            let mut transform = out_image.get_mut(row - iterator_row - 1, iterator_col).unwrap();

            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in a2.iter_row_major() {

            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = out_image.get_mut(row - iterator_row - 1, iterator_col).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(out_image, image_input.denominator)
}

pub fn rotation_180(file_name: Option<&str>,col_maj: bool){
    let image_input = RgbImage::read(file_name).unwrap();
    let row = image_input.height as usize;
    let col = image_input.width as usize;
    let a2 = Array2::from_row_major(col, row, image_input.pixels).unwrap();

    let mut output_img = Array2::new(col,row,csc411_image::Rgb {red: 0, green: 0, blue: 0});

    if col_maj{
        for pixel in a2.iter_col_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;

            let mut transform = output_img.get_mut(col - iterator_col - 1, row - iterator_row - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in a2.iter_row_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = output_img.get_mut(col - iterator_col - 1, row - iterator_row - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image_input.denominator);
}

pub fn rotation_270(file_name: Option<&str>,col_maj: bool){
    let image_input = RgbImage::read(file_name).unwrap();
    let row = image_input.height as usize;
    let col = image_input.width as usize;
    let a2 = Array2::from_row_major(col, row, image_input.pixels).unwrap();


    let mut output_img = Array2::new(col,row,csc411_image::Rgb {red: 0, green: 0, blue: 0});
    if col_maj{
        for pixel in a2.iter_col_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = output_img.get_mut(iterator_row, col - iterator_col - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in a2.iter_row_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = output_img.get_mut(iterator_row, col - iterator_col - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image_input.denominator);
}


pub fn horizontal_rotation(file_name: Option<&str>, col_maj: bool){
    let image_input = RgbImage::read(file_name).unwrap();
    let row = image_input.height as usize;
    let col = image_input.width as usize;
    let a2 = Array2::from_row_major(col, row, image_input.pixels).unwrap();


    let mut output_img = Array2::new(col,row,csc411_image::Rgb {red: 0, green: 0, blue: 0});
    if col_maj{
        for pixel in a2.iter_col_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = output_img.get_mut(col - iterator_col - 1, iterator_row).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in a2.iter_row_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = output_img.get_mut(col - iterator_col - 1, iterator_row).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image_input.denominator);
}

pub fn vertical_rotation(file_name: Option<&str>, col_maj: bool){
    let image_input = RgbImage::read(file_name).unwrap();
    let row = image_input.height as usize;
    let col = image_input.width as usize;
    let a2 = Array2::from_row_major(col, row, image_input.pixels).unwrap();


    let mut output_img = Array2::new(col,row,csc411_image::Rgb {red: 0, green: 0, blue: 0});
    if col_maj{
        for pixel in a2.iter_col_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = output_img.get_mut(iterator_col, row - iterator_row - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in a2.iter_row_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = output_img.get_mut(iterator_col, row - iterator_row - 1).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image_input.denominator);
}

pub fn trans(file_name: Option<&str>,col_maj: bool){
    let image_input = RgbImage::read(file_name).unwrap();
    let row = image_input.height as usize;
    let col = image_input.width as usize;
    let a2 = Array2::from_row_major(col, row, image_input.pixels).unwrap();


    let mut output_img = Array2::new(col,row,csc411_image::Rgb {red: 0, green: 0, blue: 0});
    if col_maj{
        for pixel in a2.iter_col_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = output_img.get_mut(iterator_row, iterator_col).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    else{
        for pixel in a2.iter_row_major() {
            let iterator_col = pixel.0;
            let iterator_row = pixel.1;
            let mut transform = output_img.get_mut(iterator_row, iterator_col).unwrap();
            transform.red = pixel.2.red;
            transform.green = pixel.2.green;
            transform.blue = pixel.2.blue;
        }
    }
    print_out(output_img, image_input.denominator);
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