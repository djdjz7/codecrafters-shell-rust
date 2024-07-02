#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    loop {
        // Uncomment this block to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();
        let query = input.split_whitespace().collect::<Vec<&str>>();
        match query[0] {
            "exit" => exit(0),
            "echo" => print!("{}", query[1..].join(" ")),
            _ => println!("{}: command not found", input),
        }
    }
}
