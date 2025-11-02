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
        let input=input.trim();
        if input.is_empty(){
            continue;
        }
        else if input.starts_with("echo "){
            let toout=input[5..].trim();
            println!("{}", toout);
        }
        else if input=="exit" || input=="quit" || input=="exit 0" || input=="quit 0" || input=="exit 1"{
            break;
        }
        else if input=="help"{
            println!("Available commands:");
            println!("  help - Show this help message");
            println!("  exit, quit, exit 0, quit 0, exit 1 - Exit the shell");
        }
        else{
            println!("{}: command not found", input.trim());
    
        }
    }
}
