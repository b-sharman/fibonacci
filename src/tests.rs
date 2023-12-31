use super::*;

#[test]
fn test_recursion() {
    assert_eq!(1, recursion(0));
    assert_eq!(1, recursion(1));
    assert_eq!(8, recursion(5));
}

#[test]
fn test_dp() {
    let mut dp = DP::new();
    assert_eq!(1, dp.nth_fib(0));
    assert_eq!(1, dp.nth_fib(1));
    assert_eq!(8, dp.nth_fib(5));
}
