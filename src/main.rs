use std::env;
use std::io::{self, Write, Read};
use std::process::{Command, Stdio};
use sysinfo::{Pid, ProcessExt, System, SystemExt};
use std::process::exit;

fn main() {
    loop {
        print!("rust-shell> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let commands: Vec<&str> = input.split('|').map(str::trim).collect();
        if commands.len() > 1 {
            if let Err(e) = handle_pipeline(commands) {
                eprintln!("Error executing pipeline: {}", e);
            }
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = match parts.next() {
            Some(cmd) => cmd,
            None => continue,
        };
        let args: Vec<&str> = parts.collect();

        match command {
            "cd" => {
                if args.len() != 1 {
                    eprintln!("Usage: cd <directory>");
                } else {
                    let path = args[0];
                    if let Err(e) = env::set_current_dir(path) {
                        eprintln!("cd: {}", e);
                    }
                }
            }
            "pwd" => {
                match env::current_dir() {
                    Ok(path) => println!("{}", path.display()),
                    Err(e) => eprintln!("pwd: {}", e),
                }
            }
            "echo" => {
                println!("{}", args.join(" "));
            }
            "ps" => {
                match ps_command() {
                    Ok(_) => {}
                    Err(e) => eprintln!("ps: {}", e),
                }
            }
            "kill" => {
                if args.len() != 1 {
                    eprintln!("Usage: kill <pid>");
                } else if let Ok(pid) = args[0].parse::<i32>() {
                    match kill_process(pid) {
                        Ok(_) => println!("Process {} killed", pid),
                        Err(e) => eprintln!("kill: {}", e),
                    }
                } else {
                    eprintln!("kill: Invalid PID");
                }
            }
            "exit" | "\\quit" => {
                println!("Exiting...");
                exit(0);
            }
            _ => {
                if let Err(e) = execute_external_command(command, &args) {
                    eprintln!("Error executing command: {}", e);
                }
            }
        }
    }
}

fn execute_external_command(command: &str, args: &[&str]) -> io::Result<()> {
    let mut child = Command::new(command)
        .args(args)
        .spawn()?;

    child.wait()?;
    Ok(())
}

fn handle_pipeline(commands: Vec<&str>) -> io::Result<()> {
    let mut previous_command = None;

    for (i, command) in commands.iter().enumerate() {
        let mut parts = command.split_whitespace();
        let cmd = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        let stdin = if let Some(output) = previous_command {
            Stdio::from(output)
        } else {
            Stdio::inherit()
        };

        let stdout = if i == commands.len() - 1 {
            Stdio::inherit()
        } else {
            Stdio::piped()
        };

        let mut child = Command::new(cmd)
            .args(&args)
            .stdin(stdin)
            .stdout(stdout)
            .spawn()?;

        previous_command = child.stdout.take();
    }

    if let Some(mut final_output) = previous_command {
        io::copy(&mut final_output, &mut io::stdout())?;
    }

    Ok(())
}

fn ps_command() -> io::Result<()> {
    let mut system = System::new_all();
    system.refresh_all();

    println!("{:<10} {:<20} {:<10}", "PID", "Name", "Running Time (ms)");
    for (pid, process) in system.processes() {
        println!("{:<10} {:<20} {:<10}", pid, process.name(), process.run_time());
    }

    Ok(())
}

fn kill_process(pid: i32) -> io::Result<()> {
    let mut system = System::new_all();
    system.refresh_all();

    if let Some(process) = system.process(Pid::from(pid as usize)) {
        if process.kill() {
            println!("Process {} killed successfully.", pid);
        } else {
            eprintln!("Failed to kill process {}.", pid);
        }
    } else {
        eprintln!("No such process with PID {}.", pid);
    }

    Ok(())
}
