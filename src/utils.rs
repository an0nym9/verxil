use std::error::Error;
use std::fs;

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

/// List all sub-dirs in a directory
pub fn list_dirs(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut dirs = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            dirs.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    Ok(dirs)
}
