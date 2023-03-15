// primitive_types5.rs
// Destructure the `cat` tuple so that the println will work.
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand for a hint.



fn main() {
    let cat = ("Furry McFurson", 3.5);// for tuples we are using parantheses
    let (name, age) = cat; // cat has tuple which given (name, age) 
                           // so we created two variable in a single tuple which is (name, age)

    println!("{} is {} years old.", name, age);
}
