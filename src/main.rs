#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    env,
    path::PathBuf,
    process::{exit, Command},
};

const BUILTIN_COMMANDS: [&str; 5] = ["exit", "echo", "type", "pwd", "cd"];

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
        "pwd" => {
            let current_dir = env::current_dir().unwrap();
            println!("{}", current_dir.display());
        }
        "cd" => {
            if args[0] == "~" {
                let home_dir = env::var("HOME").unwrap();
                env::set_current_dir(&home_dir).unwrap();
            } else {
                let current_dir = env::current_dir().unwrap();
                let after = current_dir.join(args.join(" "));
                match env::set_current_dir(&after.as_path()) {
                    Ok(()) => {}
                    Err(e) => match e.kind() {
                        io::ErrorKind::NotFound => {
                            println!("cd: {}: No such file or directory", args.join(" "))
                        }
                        _ => {}
                    },
                }
            }
        }
        "type" => {
            let mut flag = false;

            if BUILTIN_COMMANDS.contains(&args[0]) {
                flag = true;
                println!("{} is a shell builtin", &args[0]);
            } else {
                match search_command_in_paths(&args[0]) {
                    Some(path) => {
                        flag = true;
                        println!("{} is {}", &args[0], &path.display());
                    }
                    None => {}
                }
            }
            if !flag {
                println!("{}: not found", &args[0])
            }
        }
        _ => match search_command_in_paths(command) {
            Some(path) => {
                let output = Command::new(&path).args(args).output().unwrap();
                io::stdout().write_all(&output.stdout).unwrap();
            }
            None => println!("{}: not found", command),
        },
    }
}

fn search_command_in_paths(command: &str) -> Option<PathBuf> {
    let full_path = env::var("PATH").unwrap();
    let paths = env::split_paths(&full_path);
    for path in paths {
        let file_path = path.join(command);
        if file_path.exists() {
            return Some(file_path);
        }
    }
    return None;
}
