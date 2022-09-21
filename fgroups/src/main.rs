extern crate core;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;


pub fn main() {
    //let mut groups = HashMan::new();
    let mut fp: Vec<String> = Vec::new();
    let mut names: Vec<String> = Vec::new();
    let file = BufReader::new(File::open("test.txt").expect("Unable to open file"));
    let mut switch= 1;
    let mut word1 = String::new();
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
                let mut space = ch.is_whitespace();
                if !space {
                    fp.push(ch.to_string());
                }
            }
            if switch == 2{
                if !space {
                    if ch.to_string() != "" {
                        let a = ch.to_string();
                        word1.push_str(&a);
                    }
                }
                else{
                    names.push(word1.to_string());
                    word1 = String::new();
                }
            }
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