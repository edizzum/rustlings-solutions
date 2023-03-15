// quiz2.rs
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums

// Let's build a little machine in the form of a function.
// As input, we're going to give a list of strings and commands. These commands
// determine what action is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
// No hints this time!



pub enum Command {
    Uppercase,
    Trim,
    Append(usize),//usize is a numerical type whose size depends on the architecture of the operating system
}

mod my_module {
    use super::Command;

    // TODO: Complete the function signature!
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            let s = match command {//in the test part, the question wants from us to use all of the Command features
                //so we create a variable s which is equal to a match. With this way test can choose what it want to do
                Command::Uppercase => string.to_uppercase(),//we did this to make all letters big
                Command::Trim => string.trim().into(),//removes unnecessary blankets
                Command::Append(i) => string.to_owned() + &"bar".repeat(*i),//.to_owned() Gets ownership himself
                //and creates a new String. For "Append(i)" that "i" is in charge of indicating how many times
                //.repeat() will repeat. I used & for the "bar" because if we want to make it keeps writing it,
                //it has to be a reference.
            };
            output.push(s)
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
