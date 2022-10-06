pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Array2D<T: Clone> {
    array: Vec<T>,
    num_rows: usize,
    num_columns: usize,
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
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

}
