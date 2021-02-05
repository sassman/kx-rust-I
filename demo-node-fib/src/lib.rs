#[no_mangle]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

#[test]
fn test_fib() {
    assert_eq!(fibonacci(0), 1);
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 2);
    assert_eq!(fibonacci(3), 3);
    assert_eq!(fibonacci(4), 5);
    assert_eq!(fibonacci(5), 8);
    assert_eq!(fibonacci(6), 13);
}