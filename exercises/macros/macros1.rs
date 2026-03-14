// macros1.rs
//
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE hon 宏在rust中用!表示

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
