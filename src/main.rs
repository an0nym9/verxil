use std::env;
use std::error::Error;
mod commands;
use commands::{add, commit, init, status};

enum Command<'a> {
    Init,
    Add(&'a str),
    Commit(&'a str),
    Status,
    Help,
    Unknown(&'a str),
}

/// Check if the args is correct
fn parse_command(args: &[String]) -> Command<'_> {
    match args.get(0).map(String::as_str) {
        Some("init") => Command::Init,
        Some("add") => Command::Add(args.get(1).map(String::as_str).unwrap_or("")),
        Some("commit") => Command::Commit(args.get(1).map(String::as_str).unwrap_or("")),
        Some("status") => Command::Status,
        Some(cmd) => Command::Unknown(cmd),
        None => Command::Help,
    }
}

/// Prints current features with short descriptions
fn print_help() {
    let cmds = [
        ("init", "initialize the repository"),
        ("add", "add files to be committed"),
        ("commit", "commit changes"),
        ("status", "check repository status"),
    ];

    println!("Supported commands:");
    for (cmd, desc) in cmds {
        println!("  {:<8} {}", cmd, desc);
    }
}

/// Entry point of the program
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    match parse_command(&args) {
        Command::Init => init::initialize_project()?,
        Command::Add(path) => add::add_object(&path.to_string())?,
        Command::Commit(msg) => commit::commit_changes(msg)?,
        Command::Status => status::check_status()?,
        Command::Help => print_help(),
        Command::Unknown(cmd) => {
            eprintln!("Unknown command: '{}'\n", cmd);
            print_help();
        }
    }

    Ok(())
}
