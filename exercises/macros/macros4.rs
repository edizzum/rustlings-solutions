// macros4.rs
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a hint.



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };//we need to use semicolon if you want to use another macro func
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
