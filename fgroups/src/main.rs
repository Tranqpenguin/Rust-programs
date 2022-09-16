
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn main() {
    let mut fp: Vec<char> = Vec::new();
    let mut names: Vec<char> = Vec::new();
    let mut content: Vec<char> = Vec::new();
    let file = BufReader::new(File::open("test.txt").expect("Unable to open file"));

    for line in file.lines() {
        for ch in line.expect("Unable to read line").chars() {
            let mut space = ch.is_whitespace();
            content.push(ch);
            //println!("Character: {}", ch);
        }
    }
    
    println!("--------------------------------------");
    println!("{:?}", content);
}

