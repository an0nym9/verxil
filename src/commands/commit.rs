use std::error::Error;
use std::fs;
use std::path::Path;

/// List all files in a directory
pub fn list_files(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.metadata()?.is_file() {
            files.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    Ok(files)
}

/// Commit staged files by moving them from index to objects
pub fn commit_changes(message: &str) -> Result<(), Box<dyn Error>> {
    if message.is_empty() {
        println!("Expecting a commit message.");
        return Ok(());
    }

    let index_path = Path::new(".verxil/index");
    let objects_path = Path::new(".verxil/objects");

    if !index_path.exists() {
        println!("Nothing to commit. Index is empty.");
        return Ok(());
    }

    let files = list_files(index_path.to_str().unwrap())?;

    if files.is_empty() {
        println!("Nothing to commit. Index is empty.");
        return Ok(());
    }

    for file in &files {
        let old_path = index_path.join(&file);
        let new_path = objects_path.join(&file);
        fs::rename(&old_path, &new_path)?; // or copy + remove if you want
    }

    println!("Successfully committed {} files.", files.len());
    println!("Message: {}", message);

    Ok(())
}
