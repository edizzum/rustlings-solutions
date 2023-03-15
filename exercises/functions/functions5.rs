// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.



fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num //; removed.
    //There is 2 return type. 
    //First: using return and ; - Example: return num * num;
    //Second: without using return and ; - Example: num * num
}
