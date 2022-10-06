use std::io::{BufRead, BufReader};
use std::fs::File;
pub fn main() {
    let mut vec1 = Vec::new();
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