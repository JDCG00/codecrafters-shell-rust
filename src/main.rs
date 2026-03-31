#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env::var, fs::read_dir, os::unix::fs::PermissionsExt, process::Command};

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
                        let mut found = false;
                        for directory in directories {
                            if read_directory(directory, "", argument).unwrap_or(false) {
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            println!("{}: not found", argument)
                        }
                    }
                },
                _ => {
                    let path = var("PATH").unwrap_or_default();
                    let directories: Vec<&str> = path.split(':').collect();
                    let mut found = false;
                    for directory in directories {
                        if read_directory(directory, command, argument).unwrap_or(false) {
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        println!("{}: command not found", command.trim())
                    } else {
                        let _ = exec_command(command, argument);
                    }
                }
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

fn read_directory(dir: &str, command: &str, argument: &str) -> io::Result<bool> {
    let entries = read_dir(dir)?;
    let input = if command.is_empty() {
        argument
    } else {
        command
    };

    for entry_result in entries {
        let entry = entry_result?;
        let file = entry.file_name();

        if file == input {
            {
                let permissions = entry.metadata()?.permissions().mode();
                let is_exec = (permissions & 0o111) != 0;

                if is_exec {
                    let path = entry.path();
                    if command.is_empty() {
                        println!("{} is {}", file.to_string_lossy(), path.display());
                    }

                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

fn exec_command(command: &str, argument: &str) -> io::Result<()> {
    let args = argument.split(' ');

    let mut child = Command::new(command)
        .args(args)
        .spawn()
        .expect("Failed to execute command");

    child.wait()?;
    // let formated_output = output;
    // println!("Output: {:?}", formated_output);
    // io::stdout().write_all(&output.stdout)?;

    // let mut list_dir = Command::new(command);
    // let status = list_dir.status().expect("Failed to execute command");
    // println!("process finished with: {status}");

    Ok(())
}
