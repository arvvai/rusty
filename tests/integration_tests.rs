use rusty::calc;

#[test]
fn integ_test_c_to_f() {
    assert_eq!(calc::celsius2farenheit(0), 32);
    assert_eq!(calc::celsius2farenheit(100), 212);
    assert_eq!(calc::celsius2farenheit(-40), -40);
    assert_eq!(calc::celsius2farenheit(37), 98);
}

#[test]
fn integ_test_f_to_c() {
    assert_eq!(calc::farenheit2celsius(32), 0);
    assert_eq!(calc::farenheit2celsius(212), 100);
    assert_eq!(calc::farenheit2celsius(-40), -40);
    assert_eq!(calc::farenheit2celsius(98), 36);
}

#[test]
fn integ_test_fib_loop() {
    assert_eq!(calc::fibonacci_loop(10), 55);
}

#[test]
fn integ_test_fib_rec() {
    assert_eq!(calc::fibonacci_rec(10), 55)
}
