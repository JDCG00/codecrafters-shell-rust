#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        // if command == "exit" {
        //     break;
        // }

        match command.split_once(' ') {
            Some((command, arguments)) => match command {
                "echo" => println!("{}", arguments),
                "type" => match arguments {
                    "echo" => println!("{} is a shell builtin", arguments),
                    "type" => println!("{} is a shell builtin", arguments),
                    "exit" => println!("{} is a shell builtin", arguments),
                    _ => println!("{}: not found", arguments),
                },
                _ => println!("{}: command not found", command.trim()),
            },
            None => match command.as_str() {
                "exit" => break,
                "echo" => println!(),
                _ => println!("{}: command not found", command.trim()),
            },
        }
        // if command.starts_with("echo ") {
        //     println!("{}", &command[5..]
        // } else {
        //     println!("{}: command not found", command.trim());
        // }
    }
}
