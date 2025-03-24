/// A simple addition function
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// A simple subtraction function
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// A simple multiplication function
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// A simple division function
pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("Cannot divide by zero");
    }
    a / b
}

/// Raise a number to the power of the exponent value
pub fn power(n: f64, exp: f64) -> f64 {
    (n as u32).pow(exp as u32) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6.0, 3.0), 2.0);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_by_zero() {
        divide(6.0, 0.0);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(3.0, 3.0), 27.0);
    }
}
