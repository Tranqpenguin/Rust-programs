pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn minimum1(v1: &Vec<i32>) -> i32{
    let min_value = v1.iter().min().unwrap();
    return *min_value;
}


pub fn square(x: i16) -> i32{
    let a = x as i32 *x as i32; 
    return a.into()
}
#[cfg(test)]
mod tests {
    use crate::{minimum1, square};

    #[test]
    fn square_two(){
        assert_eq!(square(2), 4);
    }

    #[test]
    fn square_10k(){
        assert_eq!(square(10000), 100000000);
    }
    
    #[test]
    fn square_neg_three(){
        assert_eq!(square(-3), 9);
    }

    #[test]
    fn minimum_v(){
        let v = vec![5,4,3,2,76];
        assert_eq!(minimum1(&v), 2);
    }
}
