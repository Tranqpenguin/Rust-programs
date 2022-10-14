use std::iter::Iterator;
use std::borrow::Borrow;
 pub struct Array2<T: Clone> {
        width: usize,
        height: usize,
        data: Vec<T>,
    }
    impl<T: Clone> Array2<T> {
        // Create a new 2D Array
        pub fn new_array(rows: usize, cols: usize, val: T) -> Self {
            Array2 {
                width : cols,
                height : rows,
                data : vec![val; rows * cols],
            }
        }
    pub fn from_row_major(rows: usize, cols: usize, values: Vec<T>) -> Self {
        Array2 {
            width : cols,
            height : rows,
            data : values,
        }
    }
    pub fn get(&self, col: usize, row: usize) -> Option<&T> {
        if row >= self.height || col >= self.width{
            None
        }
        else{
            Some(self.data[row * self.width.borrow() + col].borrow())
        }
    }
    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().enumerate().map(move |(i, v)| (i % self.width, i / self.width, v))
    }
    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width).map(move |c| (c, self.data.iter().skip(c))).flat_map(move |(c, col)| {
                col.step_by(self.width).enumerate().map(move |(r, val)| (c, r, val))
            })
    }
}


