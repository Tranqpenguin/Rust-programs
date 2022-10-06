use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct array2{
    height: u32,
    width: u32,
}

pub fn iter_row_major(){

}

pub fn iter_row_column(){

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        array2();
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
