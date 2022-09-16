//Andre Quiroa 
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut line = String::new();
    println!("Enter File Name: ");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    let mut file = File::open(b1.to_string()).expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");
    println!("{}", data);

}

