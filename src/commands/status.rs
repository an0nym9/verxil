use std::error::Error;

use crate::commands::commit::list_files;

pub fn check_status() -> Result<(), Box<dyn Error>> {
    let staged_files = list_files(".verxil/index")?;
    let commited_files = list_files(".verxil/objects")?;
    println!("Staged files:");
    if !staged_files.is_empty() {
        for file in staged_files {
            println!("\t- {}", file);
        }
    } else {
        println!("\t- Empty")
    }
    println!("Commited files:");
    if !commited_files.is_empty() {
        for file in commited_files {
            println!("\t- {}", file);
        }
    } else {
        println!("\t- Empty")
    }
    Ok(())
}
