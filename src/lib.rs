pub mod calc {
    ///```
    /// let result = rusty::calc::celsius2farenheit(10);
    /// assert_eq!(result,50);
    /// ```
    pub fn celsius2farenheit(celsius: i32) -> i32 {
        let f: i32 = (celsius * 9 / 5) + 32;
        f
    }

    ///```
    /// let result = rusty::calc::farenheit2celsius(50);
    /// assert_eq!(result,10);
    /// ```
    pub fn farenheit2celsius(farenheit: i32) -> i32 {
        let c: i32 = (farenheit - 32) * 5 / 9;
        c
    }

    ///```
    /// let result = rusty::calc::fibonacci_loop(10);
    /// assert_eq!(result,55);
    /// ```
    pub fn fibonacci_loop(n: u32) -> u64 {
        //current = sum of prev 2

        let mut prev: u64 = 0;
        let mut current: u64 = 1;

        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        }

        for _ in 2..=n {
            let next: u64 = prev + current;
            prev = current;
            current = next;
        }

        current
    }

    ///```
    /// let result = rusty::calc::fibonacci_rec(10);
    /// assert_eq!(result,55);
    /// ```
    pub fn fibonacci_rec(n: u32) -> u64 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else {
            fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::calc;

    #[test]
    fn unit_test_c_to_f() {
        assert_eq!(calc::celsius2farenheit(0), 32);
        assert_eq!(calc::celsius2farenheit(100), 212);
        assert_eq!(calc::celsius2farenheit(-40), -40);
        assert_eq!(calc::celsius2farenheit(37), 98);
    }

    #[test]
    fn unit_test_f_to_c() {
        assert_eq!(calc::farenheit2celsius(32), 0);
        assert_eq!(calc::farenheit2celsius(212), 100);
        assert_eq!(calc::farenheit2celsius(-40), -40);
        assert_eq!(calc::farenheit2celsius(98), 36);
    }

    #[test]
    fn unit_test_fib_loop() {
        assert_eq!(calc::fibonacci_loop(10), 55);
    }

    #[test]
    fn unit_test_fib_rec() {
        assert_eq!(calc::fibonacci_rec(10), 55)
    }
}
