use csc411_image::Gray;
use array2::Array2;

pub fn rotate_90(a2: &Array2<Gray>) -> Vec<Option<&Gray>> {
    let mut output = Vec::new();
    for (row,col,_pix) in a2.iter_row_major(){
        output.push(a2.get(col,(a2.height() - 1) - row));
    }
    return output;
}


pub fn rotate_180(a2: &Array2<Gray>) -> Vec<Option<&Gray>> {
    let mut output = Vec::new();
    for (col,row,_pix) in a2.iter_row_major(){
        output.push(a2.get((a2.height() - 1) - col,(a2.width() - 1) - row));
    }
    return output;
}