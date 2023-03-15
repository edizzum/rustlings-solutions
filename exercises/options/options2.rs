// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {//if let is a statement that if the optional_target is Ok()
                                             //let word = optional_target
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..(range + 1) {
            optional_integers.push(Some(i));
        }

        // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
        // You can stack `Option<T>`'s into while let and if let
        while let Some(Some(integer)) = optional_integers.pop() {//while optional_integers is Ok()
                                                                 //let integer is optional_integers and keep 
                                                                 //loop it
            assert_eq!(integer, range);
            range -= 1;
        }
    }
}
