pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Array2{
    height: u32,
    width: u32,
}

pub fn iter_row_major(){

}

pub fn iter_row_column(){

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
