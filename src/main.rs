#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env::var, fs::read_dir};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();

        match command.split_once(' ') {
            Some((command, argument)) => match command {
                "echo" => println!("{}", argument),
                "type" => match argument {
                    "echo" => println!("{} is a shell builtin", argument),
                    "type" => println!("{} is a shell builtin", argument),
                    "exit" => println!("{} is a shell builtin", argument),
                    _ => {
                        println!("type {}", argument);
                        let path = var("PATH").unwrap();
                        let directories: Vec<&str> = path.split(':').collect();
                        for directory in directories {
                            let _ = read_perms(directory, argument);
                        }
                    }
                },
                _ => println!("{}: command not found", command.trim()),
            },
            None => match command.as_str() {
                "echo" => println!(),
                "type" => println!("Need one command. \nExample: type ls"),
                "exit" => break,
                _ => println!("{}: command not found", command.trim()),
            },
        }
    }
}

fn read_perms(dir: &str, argument: &str) -> io::Result<()> {
    let entries = read_dir(dir)?;
    for entry_result in entries {
        let entry = entry_result?;
        let file = entry.file_name();
        let path = entry.path();
        if file == argument {
            println!("Binario {} encontrado en {}", argument, path.display());
        }
    }
    Ok(())
}
