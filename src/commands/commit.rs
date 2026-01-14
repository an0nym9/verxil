use std::error::Error;
use std::fs;
use std::path::Path;

/// List all the files in the specified path dir
pub fn list_files(path: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_file() {
            files.push(entry.file_name().into_string().unwrap());
        }
    }
    Ok(files)
}

/// Commit changes by moving files in index to object dir
pub fn commit_changes(message: &str) -> Result<(), Box<dyn Error>> {
    if !message.is_empty() {
        let files = list_files(".verxil/index");
        for file in files? {
            let file_name = file.to_string();
            let old_path_string = format!(".verxil/index/{}", file_name);
            let new_path_string = format!(".verxil/objects/{}", file_name);
            let old_path = Path::new(&old_path_string);
            let new_path = Path::new(&new_path_string);
            fs::rename(old_path, new_path)?;
        }
        println!("Successfully commited files.");
    } else {
        println!("Expecting a message.");
    }
    Ok(())
}
