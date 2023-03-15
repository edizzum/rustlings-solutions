// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.



fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(vec0.clone());//This is a part of ownership. Using alone vec0 means we borrow
                                          //vec0 ownership and give it to vec1 so we lost vec0. 
                                          //If we just clone the values in vec0 everyone will be happy :) 

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
