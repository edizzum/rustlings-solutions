// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.



fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);
//I changed above and below one get & and one is not because rustlings wants like that :)
    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {//We can define &String like a reference of data but the type
                                    // also will be &String
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {//deleted & for to take ownership
    data = data.to_uppercase();

    println!("{}", data);
}
