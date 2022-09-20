use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn main() {
    let mut map: HashMap<&str, &str> = HashMap::new();
    let file = BufReader::new(File::open("test.txt").expect("Unable to open file"));
    for line in file.lines() {
        for ch in line.as_ref().expect("Unable to read line").chars() {
            let space = ch.is_whitespace();
            let astring = (ch);
            if space == true {
                for ch in line.expect("Unable to read line").chars() {
                    let bstring = (ch);
                    if space == true {
                        map.insert(
                            &astring.to_string(),
                            &bstring.to_string(),
                        );
                        break;
                    }
                }

            }
            //println!("Character: {}", ch);
            //let groups1 = HashMap::from([groups::new(ch.to_string(), " ")]);
        }

        for(a,b) in &map{
            print!("{a}: \"{b}\"");
        }
        println!("--------------------------------------");
        //println!("{:?}", content);
    }
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