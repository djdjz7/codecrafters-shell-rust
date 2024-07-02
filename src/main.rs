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
        handle_input(query[0], query[1..].to_vec());
    }
}

fn handle_input(command: &str, args: Vec<&str>) {
    match command {
        "exit" => exit(0),
        "echo" => println!("{}", args.join(" ")),
        "type" => {
            let builtin_commands = ["exit","echo","type"];
            if builtin_commands.contains(&args[1]) {
                println!("{} is a shell builtin", command);
            }
            else {
                println!("{}: not found", command)
            }
        }
        _ => println!("{}: command not found", command),
    }
}
