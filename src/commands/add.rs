use std::error::Error;
use std::fs;
use std::path::Path;

// Hash the given string
fn hash(content: &str) -> String {
    let bytes = content.as_bytes();
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Add a file to object dir
pub fn add_object(path: &str) -> Result<(), Box<dyn Error>> {
    let staging_path = Path::new(".verxil/index");
    let file_path = Path::new(path);
    if Path::exists(file_path) && Path::is_file(file_path) {
        if !Path::exists(staging_path) {
            println!("Cannot find verxil in project directory.");
            return Ok(());
        }
        if let Some(name) = file_path.file_name().and_then(|s| s.to_str()) {
            fs::write(staging_path.join(hash(name)), fs::read_to_string(path)?)?;
        } else {
            println!("Failed to add file.");
        }
    } else {
        println!("Path doesn't exists.");
    }
    Ok(())
}
