// strings4.rs

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!
// No hints this time!



fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());//.to_string() is to make &str -> String
    string(String::from("hi"));//String::from() classic.
    string("rust is fun!".to_owned());//the important stuff is a ownership for Strings.
    string_slice("nice weather".into());//.into() is to loop all letters nothing special.
    string(format!("Interpolation {}", "Station"));//format! is a String usecase
    string_slice(&String::from("abc")[0..1]);//it is String_from but it borrowed with & so -> &str
    string_slice("  hello there ".trim());//.trim() removing blankets nothing special
    string("Happy Monday!".to_string().replace("Mon", "Tues"));//similar like second one. you know replace from Strings3.rs
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());//.to_lowecase() this method can only use for Strings to make them lowercase. 
}
