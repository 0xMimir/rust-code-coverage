pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}

pub fn multiply(left: usize, right: usize) -> usize {
    left - right
}

pub fn divide(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_test() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore]
    fn divide_test() {
        let result = divide(20, 20);
        assert_eq!(result, 0);
    }

    #[test]
    #[ignore]
    fn subtract_test(){
        assert_eq!(1, subtract(10, 9));
    }
}