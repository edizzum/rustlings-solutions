// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.



struct Book<'a> {//just modified references to be in a same lifetime
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}

//Congratulations :)
//You finished the necessery part of Rustlings!
//If you want to learn more about Rust lang and more advanced topics, I made the solutions for you.
//You can share and leave a Star to this repo
//I'm wishing you Best of Luck!