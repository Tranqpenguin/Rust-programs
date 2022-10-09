use std::io::{BufRead};
use std::io;
use array2::Array2;

fn ws(c:char) -> bool{
    c == ' ' || c == '\t'
}
pub fn main() {
    let mut vec1 = Vec::new();
    let mut length = 0;
    for line in io::stdin().lock().lines(){
        let mut line_split = line.as_ref().unwrap().splitn(1,|c:char|ws(c));
        let a = line_split.next().unwrap().to_string();
        let split: String = a;
        for s in split.chars(){
            if s != ' '{
                length = length + 1 as usize;
                vec1.push(s);
            }
        }
    }

    let mut suko = Array2::new_array2(9,9,vec1);
    //let mut lim = Array2::from_row_major(9,9,vec1);
    let mut time = Array2::iter_col_major(&suko);
    let mut ye = Array2::new_array2(9,9,&time);
    //let bam = suko.get_mut(0,0);
    println!(ye);



}