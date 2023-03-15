// traits2.rs
//
// Your task is to implement the trait
// `AppendBar` for a vector of strings.
//
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time,
// you can do this!
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.



trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String> {//in the test part there is 2 assert_eq!() func which they shows us that there are 2 words
                                //have to be in Vec<> which they are: "Bar" & "Foo"
    fn append_bar(mut self) -> Self {//So we made mutable self
        self.push(String::from("Bar"));//then we pushed "Bar" into the vector
        self// then we returned self which is pushing "Bar". If you don't understand look at the bottom part.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //The purpose of the append_bar() function is to push the word "Bar" into an outer Vector. Which is foo.
    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();//foo created a new vector with vec![String stuff] and then 
                                                             //used .append_bar() fucntion to push "Bar" into the foo vector
                                                             //which is a new vector.
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));//The purpose of of the .pop() is, it removes the last piece 
                                                            //from the vector which is "Bar". Then assert_eq!() does its job.
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
