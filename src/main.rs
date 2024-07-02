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
        let command_vec = input.split_whitespace().collect::<Vec<&str>>();
        handle_input(command_vec[0], command_vec[1..].to_vec());
    }
}

fn handle_input(command: &str, args: Vec<&str>) {
    match command {
        "exit" => exit(0),
        "echo" => println!("{}", args.join(" ")),
        "type" => {
            let builtin_commands = ["exit","echo","type"];
            if builtin_commands.contains(&args[0]) {
                println!("{} is a shell builtin", &args[0]);
            }
            else {
                println!("{}: not found", &args[0])
            }
        }
        _ => println!("{}: command not found", command),
    }
}
