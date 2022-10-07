use std::io::{BufRead};
use std::io;
use array2::Array2;

fn ws(c:char) -> bool{
    c == ' ' || c == '\t'
}
pub fn main() {
    let mut vec1 = Vec::new();
    for line in io::stdin().lock().lines(){
        let mut line_split = line.as_ref().unwrap().splitn(1,|c:char|ws(c));
        let a = line_split.next().unwrap().to_string();
        vec1.push(a);
    }
    let a = Array2{
        array: vec1,
        num_rows: 9 as usize,
        num_columns: 9 as usize,
    };
    println!("{:?}", vec1);
}