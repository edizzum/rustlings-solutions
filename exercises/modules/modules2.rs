// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a hint.



mod delicious_snacks {
    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit;//test wants"mY sHiFt KeY iS sTiCkY".to_lowercase() us to make fruit. With this we find a opportunity to reach PEAR
    pub use self::veggies::CUCUMBER as veggie;//same as above also we did both of them pub to make them visible to fn_main()

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
