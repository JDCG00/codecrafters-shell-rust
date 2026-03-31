use std::{
    env,
    fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Fatal error: {}", e);
    }
}

fn run() -> io::Result<()> {
    loop {
        // Prompt
        print!("$ ");
        io::stdout().flush()?;

        // Read input
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line)?;
        if bytes_read == 0 {
            // EOF (Ctrl-D) -> exit loop
            break;
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Parse command and args
        let (command, args) = parse_command(line);

        // Builtins
        match command.as_str() {
            "exit" => break,
            "echo" => {
                println!("{}", args.join(" "));
            }
            "type" => {
                handle_type(&args)?;
            }
            other => {
                // External command: look up executable in PATH
                match find_executable_in_path(other)? {
                    Some(exec_path) => {
                        if let Err(e) = exec_command(&exec_path, &args) {
                            eprintln!("Failed to execute {}: {}", other, e);
                        }
                    }
                    None => {
                        println!("{}: command not found", other);
                    }
                }
            }
        }
    }

    Ok(())
}

/// Split a line into command (first token) and args (rest).
fn parse_command(line: &str) -> (String, Vec<String>) {
    let mut parts = line.split_whitespace();
    let command = parts.next().unwrap_or("").to_string();
    let args = parts.map(|s| s.to_string()).collect();
    (command, args)
}

/// Handle the `type` builtin semantics:
/// - if no argument: print usage
/// - if argument is a shell builtin -> print that
/// - otherwise search PATH and print the path if found, otherwise "not found"
fn handle_type(args: &[String]) -> io::Result<()> {
    if args.is_empty() {
        println!("Need one command. \nExample: type ls");
        return Ok(());
    }

    let name = &args[0];

    // Known builtins
    match name.as_str() {
        "echo" | "type" | "exit" => {
            println!("{} is a shell builtin", name);
            return Ok(());
        }
        _ => {}
    }

    match find_executable_in_path(name)? {
        Some(path) => {
            println!("{} is {}", name, path.display());
        }
        None => {
            println!("{}: not found", name);
        }
    }

    Ok(())
}

/// Try to find an executable by name in PATH.
/// If `name` contains a slash, treat it as a path and test it directly.
fn find_executable_in_path(name: &str) -> io::Result<Option<PathBuf>> {
    let candidate = Path::new(name);
    if candidate.components().count() > 1 || name.contains('/') {
        // treat as path
        if is_executable(candidate)? {
            return Ok(Some(candidate.to_path_buf()));
        } else {
            return Ok(None);
        }
    }

    // Otherwise search PATH
    if let Some(paths) = env::var_os("PATH") {
        for dir in env::split_paths(&paths) {
            let full = dir.join(name);
            if is_executable(&full)? {
                return Ok(Some(full));
            }
        }
    }

    Ok(None)
}

/// Return true if `path` exists, is a file and has at least one execute bit (unix).
fn is_executable(path: &Path) -> io::Result<bool> {
    if !path.exists() {
        return Ok(false);
    }
    let meta = fs::metadata(path)?;
    if !meta.file_type().is_file() {
        return Ok(false);
    }
    let mode = meta.permissions().mode();
    Ok((mode & 0o111) != 0)
}

/// Execute an external command given its path and args.
/// Waits for the child to finish.
fn exec_command(path: &Path, args: &[String]) -> io::Result<()> {
    let mut cmd = Command::new(path);
    if !args.is_empty() {
        cmd.args(args);
    }
    let mut child = cmd.spawn()?;
    let status = child.wait()?;
    if !status.success() {
        // Non-zero exit code is not considered a fatal IO error for the shell;
        // we just report it.
        eprintln!("Process exited with: {}", status);
    }
    Ok(())
}
