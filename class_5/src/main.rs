// Prelude loads the basic funtionality of Rust

use std::io;

fn main() {
    let mut input = String::new();

    // *&* pass variable by ref
    io::stdin().read_line(&mut input)
    // *expect* handles if an error occurs while user inputs a value
    .expect("failed to read line");

    println!("{}", input);
}
