// errors4.rs
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a hint.



#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Hmm...? Why is this only returning an Ok value?
        match value {
            1.. => Ok(PositiveNonzeroInteger(value as u64)),//if value is 1 or greater returns Ok()
            0 => Err(CreationError::Zero),//if value is 0 returns Err()
            _ => Err(CreationError::Negative),//if value is less then 0 returns Err()
        }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
