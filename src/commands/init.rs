use std::error::Error;
use std::fs;
use std::path::Path;

/// Initialize verxil project
pub fn initialize_project() -> Result<(), Box<dyn Error>> {
    let path = Path::new(".verxil");

    if !path.exists() {
        // Create the main .verxil directory
        fs::create_dir(&path)?;
        // Create subdirectories
        fs::create_dir_all(path.join("objects"))?;
        fs::create_dir_all(path.join("index"))?;
        println!("Successfully initialized Verxil.");
    } else {
        println!("Verxil is already initialized in this project.");
    }

    Ok(())
}
