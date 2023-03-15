// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.



struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y:  Option<Point> = Some(Point { x: 100, y: 200 });

    match y {//we want to make a match statement without borrowing y. To fix this we used
             //a ref keyword in the Some(p) to make a reference not real value.
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),// Using `ref`, the value is borrowed, not moved
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
