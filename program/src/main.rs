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
    let mut sako = Array2::new_array2(9,9,vec1);
    let mut bako = Array2::iter_col_major(&sako);
    let mut mac = Array2::new_array2(9,9,&bako);
    //let mut suko2 =
    //let mut lim = Array2::from_row_major(9,9,vec1);
    //let mut time = Array2::iter_col_major(&suko);
    //let mut ye = Array2::new_array2(9,9,&time);
    //print!("{:?}", suko);
    for suk in bako{
        println!("{:?}", suk);
        println!("\n");
    }
    //suko.get(0,0);
    //print!("{:?}", bako);
    println!("\n");
    println!("-------------------------------------------------------------------------");
    println!("\n");

    // for r in 0..9{
    //     for c in 0..9{
    //         println!("{:?}", suko.get(r,c));
    //     }
    // }



}