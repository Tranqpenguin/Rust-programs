use std::io::{BufRead};
use std::io;


/*
Ideas:
    -Put a place holder value for every new set of 3s up to 9 in a row

 */

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
    println!("{:?}", vec1);
}