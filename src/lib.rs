/// A simple addition function
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// A simple subtraction function
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// A simple multiplication function
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// A simple division function
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2, 3), 6);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6, 3), 2);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_by_zero() {
        divide(6, 0);
    }
}

