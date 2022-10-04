pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn minimum(n: u64, vector1: &mut [u64]){
    
}
pub fn square(x: i16) -> i32{
    let a = x as i32 *x as i32; 
    return a.into()
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::square;
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
}
