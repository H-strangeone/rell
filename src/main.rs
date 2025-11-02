#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashSet;


        
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
        let builtin_cmds:HashSet<&str>=["exit","quit","help","exit 0","quit 0","exit 1","echo","type"].into_iter().collect();
        let out:HashSet<&str>=["exit","quit","help","exit 0","quit 0","exit 1"].into_iter().collect();
        if input.starts_with("type "){
            let tocheck=input[5..].trim();
            if builtin_cmds.contains(tocheck){
                println!("{} is a shell builtin", tocheck);
            }
            else{
                println!("{}: not found", tocheck);
            }
        }
        else if input.starts_with("echo "){
            let toout=input[5..].trim();
            println!("{}", toout);
        }
        else if out.contains(input){
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
