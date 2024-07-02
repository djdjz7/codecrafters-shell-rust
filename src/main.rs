#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path, process::exit};

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
            let full_path = env::var("PATH").unwrap();
            let paths = full_path.split(":");
            let mut flag = false;

            for path in paths {
                let file_path = format!("{}/{}", path, args[0]);
                let dir_path = path::Path::new(&file_path);
                if dir_path.exists() {
                    flag = true;
                    println!("{} is {}", &args[0], dir_path.display());
                    break;
                }
            }

            if !flag {
                println!("{}: not found", &args[0])
            }
        }
        _ => println!("{}: command not found", command),
    }
}
