use std::io::{BufRead, BufReader};
use std::fs::File;


/*
Ideas:
    -Put a place holder value for every new set of 3s up to 9 in a row

 */
pub fn main() {
    let mut vec1 = Vec::new();
    let space = '\n';
    let f = BufReader::new(File::open("test_array2.txt").expect("open failed"));
    for line in f.lines() {
        for c in line.expect("lines failed").chars() {
            if c.is_whitespace() == false {
                vec1.push(c);
            }
        }
    }
    println!("{:?}", vec1);



}