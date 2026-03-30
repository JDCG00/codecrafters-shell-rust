#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env::var, fs::read_dir, os::unix::fs::PermissionsExt};

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
                        let path = var("PATH").unwrap_or_default();
                        let directories: Vec<&str> = path.split(':').collect();
                        for directory in directories {
                            let found = read_directory(directory, argument);

                            if found.expect("Error") {
                                break;
                            } else {
                                println!("{}: not found", argument)
                            }
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

fn read_directory(dir: &str, argument: &str) -> io::Result<bool> {
    let entries = read_dir(dir)?;

    for entry_result in entries {
        let entry = entry_result?;
        let file = entry.file_name();

        if file == argument {
            {
                let permissions = entry.metadata()?.permissions().mode();
                let is_exec = (permissions & 0o111) != 0;

                if is_exec {
                    let path = entry.path();
                    println!("{} is {}", file.to_string_lossy(), path.display());

                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}
