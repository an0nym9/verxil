use std::error::Error;
use std::{collections::BTreeMap, env};
mod commands;
use commands::{add, commit, init};

/// Entry point of the program
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    let default = String::new();
    let cmd = args.get(0).unwrap_or(&default);
    let arg = args.get(1).unwrap_or(&default);
    match cmd.to_lowercase().as_str() {
        "init" => init::initialize_project().expect("Failed to initialize verxil."),
        "add" => add::add_object(arg).expect("Failed to add object."),
        "commit" => commit::commit_changes(arg).expect("Failed to commit."),
        "help" => {
            let cmds = BTreeMap::from([
                ("init", "initialize the repository, usage 'init'."),
                ("add", "add files to be commited, usage 'add <file>'."),
                (
                    "commit",
                    "commit changes in files, usage 'commit <message>'.",
                ),
            ]);
            println!("Here are the commands this program currently supports");
            for (key, value) in cmds.iter().rev() {
                println!("\t- {}: {}", key, value);
            }
        }
        _ => println!("Unknown option '{}', run help for more info.", cmd),
    }
    Ok(())
}
