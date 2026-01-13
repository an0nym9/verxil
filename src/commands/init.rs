use std::error::Error;
use std::fs;
use std::path::Path;

/// Initialize verxil project
pub fn initialize_project() -> Result<(), Box<dyn Error>> {
    let path = Path::new(".verxil");
    if !Path::exists(&path) {
        fs::create_dir(path)?;
        fs::create_dir(path.join("objects"))?;
        fs::create_dir(path.join("index"))?;
        println!("Successfully initialize Verxil.");
    } else {
        println!("Verxil is already initialize in the project.");
    }
    Ok(())
}
