use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;


pub fn main() {
    let mut fp: Vec<char> = Vec::new();
    let mut names: Vec<char> = Vec::new();
    let mut content: Vec<char> = Vec::new();
    let file = BufReader::new(File::open("test.txt").expect("Unable to open file"));
    let mut switch= 1;
    for line in file.lines() {
        for ch in line.expect("Unable to read line").chars() {
            let mut space = ch.is_whitespace();
            if space {
                if switch == 1{
                    switch = 2;
                }
                else{
                    switch = 1;
                }
            }
            if switch == 1{
                fp.push(ch);
            }
            if switch == 2{
                names.push(ch);
            }
            //println!("Character: {}", ch);
        }
    }

    println!("--------------------------------------");
    println!("{:?}", fp);
    println!("{:?}", names);
}

/*
let mut state = "somthing"

for group in groups{
    print!("{}", state);
    for entry in group{
        print!("{}", entry);
    }
    state = "somthing else";
}
 */