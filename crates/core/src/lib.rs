use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub description: String,
    pub version: String,
    pub debug_mode: bool,
}

pub fn get_default_config() -> AppConfig {
    AppConfig {
        app_name: "My Application".to_string(),
        description: "A test application description".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        debug_mode: false,
    }
}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

pub fn calculate_fibonacci(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    } else if n == 1 {
        return vec![0];
    }

    let mut fib = vec![0, 1];
    for i in 2..n {
        let next = fib[i as usize - 1] + fib[i as usize - 2];
        fib.push(next);
    }

    fib
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let result = greet("Rust");
        assert!(result.contains("Hello, Rust!"));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(calculate_fibonacci(5), vec![0, 1, 1, 2, 3]);
    }
}
