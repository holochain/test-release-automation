pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

/// Raise a number to the power of the exponent value
pub fn power(n: i32, exp: i32) -> i32 {
    n.pow(exp as u32)
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
