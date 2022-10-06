use std::fs::File;
use std::io;
use std::io::{BufRead, Read};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn array2(){
    fn read_a_file() -> std::io::Result<Vec<u8>> {
        let mut file = File::open("test_array2.txt")?;

        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        println!("{:?}", data);
        return Ok(data);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        array2();
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
