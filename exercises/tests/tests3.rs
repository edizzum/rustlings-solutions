// tests3.rs
// This test isn't testing our function -- make it do that in such a way that
// the test passes. Then write a second test that tests whether we get the result
// we expect to get when we call `is_even(5)`.
// Execute `rustlings hint tests3` or use the `hint` watch subcommand for a hint.



pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(is_even(4));//if num % 2 == 0 it is gonna be true
    }

    #[test]
    fn is_false_when_odd() {
        assert!(!is_even(3));//if num % 2 == 1 it will be false but we want to make test compile. So we put the
                             //"!" at the beginning of is_even() to convert it from false to true.
    }
}
