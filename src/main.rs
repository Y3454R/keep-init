use clap::{Command, Arg};
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command as ShellCommand;

fn main() -> io::Result<()> {
    let matches = Command::new("keepinit")
        .version("1.0")
        .author("Y3454R")
        .about("A CLI tool to log installation commands")
        .subcommand(
            Command::new("start")
                .about("Starts a new logging session")
                .arg(Arg::new("session_name")
                    .help("Name of the session")
                    .required(false)
                    .index(1)),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("start") {
        let session_name = matches.value_of("session_name").unwrap_or("session");
        start_session(session_name)?;
    }

    Ok(())
}

fn start_session(session_name: &str) -> io::Result<()> {
    let log_file_path = format!("{}.log", session_name);
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file_path)?;

    writeln!(log_file, "Session started: {}\n", chrono::Local::now())?;
    println!("Session '{}' started. Commands will be logged to '{}'.", session_name, log_file_path);

    loop {
        print!("> ");
        io::stdout().flush()?; // Display the prompt
        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        let command = command.trim();

        if command.eq_ignore_ascii_case("exit") {
            writeln!(log_file, "\nSession ended: {}\n", chrono::Local::now())?;
            println!("Session '{}' ended.", session_name);
            break;
        }

        // Avoid logging commands related to passwords
        if command.contains("password") {
            println!("Password input detected, not logging this command.");
            continue;
        }

        writeln!(log_file, "\n> {}", command)?;

        // Execute the command
        let output = ShellCommand::new("sh")
            .arg("-c")
            .arg(command)
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                // Print and log command output
                println!("{}", stdout);
                eprintln!("{}", stderr);
                writeln!(log_file, "{}", stdout)?;
                writeln!(log_file, "{}", stderr)?;
            }
            Err(e) => {
                eprintln!("Error executing command: {}", e);
                writeln!(log_file, "Error executing command: {}", e)?;
            }
        }
    }

    Ok(())
}

