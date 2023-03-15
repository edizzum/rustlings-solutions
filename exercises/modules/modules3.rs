// modules3.rs
// You can use the 'use' keyword to bring module paths from modules from anywhere
// and especially from the Rust standard library into your scope.
// Bring SystemTime and UNIX_EPOCH
// from the std::time module. Bonus style points if you can do it with one line!
// Execute `rustlings hint modules3` or use the `hint` watch subcommand for a hint.



// TODO: Complete this use statement
use std::time::SystemTime;//Struct of Current Time Zone
use std::time::UNIX_EPOCH;//A feature of SystemTime. Shows how many seconds passed since 1 January 1970
                          //std is our standard Library and time is a time library

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
