#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    let stdin=io::stdin();
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    println!("{}: command not found", input.trim());
}
