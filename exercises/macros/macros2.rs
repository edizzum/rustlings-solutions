// macros2.rs
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a hint.



//Unlike other things in Rust, the order of "where you define a macro" versus
//"where you use it" actually matters.

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
