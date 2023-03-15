// hashmaps1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute `rustlings hint hashmaps1` or use the `hint` watch subcommand for a hint.



use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();//First we told to program that we created a HashMap with HashMap::new()
    //HashMaps are like vectors. They are also keeping datas themselves
    //with a (key, value). Key is like a Football Team name and value is how many times this team made score.
    //Another example is, lets say i'm a Good person and my name is Ediz which makes me True
    //and my friend is a bad person and his name is Josh which makes him False.
    //So if I want to reach name with True/False we can Create a new HashMap<bool, String>
    //if i say True to HashMap it'll brings me True -> Ediz so yes it is me!
    //also if i say False to HashMap it'll brings me False -> Josh...

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 3);//Then the .insert() method helped us to add more data
    basket.insert(String::from("strawberry"), 3);//into the basket which is a HashMap
                                                 //we added more fruits becuase tests wants from us

    // TODO: Put more fruits in your basket here.

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
