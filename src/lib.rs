pub mod calc {
    /// ```
    /// let temp_f = rusty::calc::celsius_to_fahrenheit(0);
    /// assert_eq!(temp_f, 32);
    /// ```
    ///
    /// ```
    /// let temp_f = rusty::calc::celsius_to_fahrenheit(100);
    /// assert_eq!(temp_f, 212);
    /// ```
    pub fn celsius_to_fahrenheit(celsius: i32) -> i32 {
        (celsius * 9 / 5) + 32
    }

    /// ```
    /// let temp_c = rusty::calc::fahrenheit_to_celsius(32);
    /// assert_eq!(temp_c, 0);
    /// ```
    ///
    /// ```
    /// let temp_c = rusty::calc::fahrenheit_to_celsius(212);
    /// assert_eq!(temp_c, 100);
    /// ```
    pub fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
        (fahrenheit - 32) * 5 / 9
    }

    /// ```
    /// let fib = rusty::calc::fibonacci(10);
    /// assert_eq!(fib, 55);
    /// ```
    pub fn fibonacci(n: u32) -> u64 {
        let mut a: u64 = 0;
        let mut b: u64 = 1;
        for _ in 0..n {
            let temp = b;
            b = a + b;
            a = temp;
        }
        a
    }

    /// ```
    /// let fib = rusty::calc::fibonacci_recursive(10);
    /// assert_eq!(fib, 55);
    /// ```
    pub fn fibonacci_recursive(n: u32) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::calc;

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(calc::celsius_to_fahrenheit(0), 32);
        assert_eq!(calc::celsius_to_fahrenheit(100), 212);
        assert_eq!(calc::celsius_to_fahrenheit(-40), -40);
        assert_eq!(calc::celsius_to_fahrenheit(37), 98);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        assert_eq!(calc::fahrenheit_to_celsius(32), 0);
        assert_eq!(calc::fahrenheit_to_celsius(212), 100);
        assert_eq!(calc::fahrenheit_to_celsius(-40), -40);
        assert_eq!(calc::fahrenheit_to_celsius(98), 36);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(calc::fibonacci(0), 0);
        assert_eq!(calc::fibonacci(1), 1);
        assert_eq!(calc::fibonacci(2), 1);
        assert_eq!(calc::fibonacci(3), 2);
        assert_eq!(calc::fibonacci(10), 55);
        assert_eq!(calc::fibonacci(20), 6765);
    }

    #[test]
    fn test_fibonacci_recursive() {
        assert_eq!(calc::fibonacci_recursive(0), 0);
        assert_eq!(calc::fibonacci_recursive(1), 1);
        assert_eq!(calc::fibonacci_recursive(2), 1);
        assert_eq!(calc::fibonacci_recursive(3), 2);
        assert_eq!(calc::fibonacci_recursive(10), 55);
        assert_eq!(calc::fibonacci_recursive(20), 6765);
    }
}
