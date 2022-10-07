pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct Array2<T: Clone> {
    array: Vec<T>,
    num_rows: usize,
    num_columns: usize,
}

///TBD if useful
pub enum Error {
    /// The given indices were out of bounds.
    IndexOutOfBounds(usize, usize),
}

impl<T: Clone> Array2<T> {


    pub fn iter_row_major(&self)-> impl Iterator< Item = (usize, usize, &T)>{
        self.values.iter().map_row_major();

    }
    ///this takes the vector<T> and checks if it matches the hight*width of array2
    pub fn from_row_major(elements: &[T], num_rows: usize, num_columns: usize) -> Self {
        let total_len = num_rows * num_columns;
        if total_len != elements.len() {
            panic!(
                "The number of elements ({}) did not match the expected size ({})",
                elements.len(),
                total_len
            );
        }
        values_column = (0..height);
        Array2 {
            array: elements.to_vec(),
            num_rows,
            num_columns,
        }
    }


    pub fn from_column_major(elements: &[T], num_rows: usize, num_columns: usize) -> Self {
        let total_len = num_rows * num_columns;
        if total_len != elements.len() {
            panic!(
                "The number of elements ({}) did not match the expected size ({})",
                elements.len(),
                total_len
            );
        }
        ///this makes a flatmap out of rows indecies going from 0....n, the it mapes the columns along side it as a pair.
        /// vector of indexes, grab those index which makes a pattern
        let indices_row_major =
            (0..num_rows).flat_map(move |row| (0..num_columns).map(move |column| (row, column)));
        let array = indices_row_major.map(|(row, column)| {
                let index = column * num_rows + row;
                elements[index].collect()
                //(row,column,elements[index]).clone()
            })
            .collect();
        Array2 {
            array,
            num_rows,
            num_columns,
        }
    }
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.get_index(row, column).map(|index| &self.array[index])
    }

    pub fn map_col_major(coor_vecs: Vec<T>) -> Iterator {
        let array = from_row_major.map(|(row,column)|
        let index = column * width + row;
            (row, column, values[index]))
    }
}

trait Iterator for Array2{
    type Item = <(usize, usize, &T)>;
    fn next(&mut self) -> option<self::item>;
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
