use std::error::Error;
use std::fs;
use std::path::Path;

/// Add a file to object dir
pub fn add_object(path: &str) -> Result<(), Box<dyn Error>> {
    let staging_path = Path::new(".verxil/index");
    let file_path = Path::new(path);
    if Path::exists(file_path) && Path::is_file(file_path) {
        if !Path::exists(staging_path) {
            println!("Cannot find verxil in project directory.");
            return Ok(());
        }
        fs::write(staging_path.join(path), fs::read_to_string(path)?)?;
    } else {
        println!("Path doesn't exists.");
    }
    Ok(())
}
