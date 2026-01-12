use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

/// Entry point of the program
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    match args[0].as_str() {
        "init" => {
            let path = Path::new(".verxil");
            if path.exists() {
                println!("verxil is already initlize in the project.");
            } else {
                fs::create_dir(path)?;
                fs::create_dir(path.join("commits"))?;
                fs::create_dir(path.join("objects"))?;
                // Checks if the O.S. is Windows
                if cfg!(windows) {
                    Command::new("attrib")
                        .args(&["+h", ".verxil"])
                        .status()
                        .expect("Failed to set hidden attribute.");
                }
                println!("Successfully initialize verxil in the project.");
            }
        }
        "add" => {
            let path = Path::new(".verxil/objects");
            let file_path = Path::new(&args[1]);
            if !path.join(file_path).exists() {
                if !Path::exists(file_path) {
                    println!("Cannot find file, failed to add file.");
                } else {
                    let contents = fs::read(file_path)?;
                    if !path.exists() {
                        fs::create_dir(path)?;
                        println!("Created object dir.");
                    }
                    fs::write(path.join(&args[1]), contents)?;
                    println!("Succesfully added file.");
                }
            } else {
                println!("File is already added.");
            }
        }
        "commit" => {
            let path = Path::new(".verxil/commits");
            if args.len() < 2 {
                println!("Expecting a commit message.");
            } else {
                let msg = &args[1];
                fs::write(path.join("commit-sample.txt"), msg)?;
                println!("Succesfully added a commit message.");
            }
        }
        _ => println!("Unknwon command."),
    }
    Ok(())
}
