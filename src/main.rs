#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    
    // Wait for user input
    loop{
        print!("$ ");
    
    io::stdout().flush().unwrap();

        let stdin=io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        if input.trim().is_empty(){}
        else if input.trim()=="exit" || input.trim()=="quit" || input.trim()=="exit 0" || input.trim()=="quit 0" || input.trim()=="exit 1"{
            break;
        }
        else{
            println!("{}: command not found", input.trim());
    
        }
    }
}
