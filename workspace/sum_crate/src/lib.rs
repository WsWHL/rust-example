pub fn sum(a: u64, b: u64) -> u64 {
    a + b
}


#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn sum_test() {
        let a = 10;
        let b = 33;

        let result = sum(a, b);
        assert_eq!(result, 43);
    }
}
