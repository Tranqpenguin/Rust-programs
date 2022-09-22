use std::io;
use std::io::BufRead;
use std::vec::Vec;
use std::collections::HashMap;

fn ws(c:char) -> bool{
    c == ' ' || c == '\t'
}
fn main() {
    let mut fgroups_hashmap: HashMap<String, Vec<String>> = HashMap::new();
    for line in io::stdin().lock().lines(){
        let mut line_split = line.as_ref().unwrap().splitn(2,|c:char|ws(c));
        let fp = line_split.next().unwrap().to_string();
        let name = line_split.next().unwrap().to_string();
        match fgroups_hashmap.get_mut(&fp){
            None =>{
                fgroups_hashmap.insert(fp.to_string(), vec![name]);
            }
            Some(names) => {
                names.push(name);
            }
        }
    }
    let mut current_state = "";
    for names in fgroups_hashmap.values(){
        if names.len() >1{
            print!("{}", current_state);
            for name in names{
                print!("{}", name.trim_start());
            }
        }
        current_state = "\n"
    }
}