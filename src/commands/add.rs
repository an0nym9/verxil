use std::error::Error;
use std::fs;
use std::path::Path;

/// Add file or dirs to stage
pub fn add_object(path: &str) -> Result<(), Box<dyn Error>> {
    let target = if path == "-A" || path == "-all" {
        Path::new(".")
    } else {
        Path::new(path)
    };

    if !target.exists() {
        return Err(format!("path '{}' does not exist", target.display()).into());
    }

    let staging = Path::new(".verxil/index");
    fs::create_dir_all(staging)?;

    if target.is_file() {
        add_file(target, staging)?;
    } else if target.is_dir() {
        add_directory(target, staging)?;
    }

    Ok(())
}

/// Add directory to staging
fn add_directory(dir: &Path, staging: &Path) -> Result<(), Box<dyn Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            add_directory(&path, staging)?;
        } else if path.is_file() {
            add_file(&path, staging)?;
        }
    }
    Ok(())
}

/// Add file to staging
fn add_file(path: &Path, staging: &Path) -> Result<(), Box<dyn Error>> {
    let hash = hash_file(path)?;
    let object_path = staging.join(hash);

    fs::write(object_path, fs::read(path)?)?;
    println!("added {}", path.display());

    Ok(())
}

/// Hash file contents
fn hash_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    Ok(bytes.iter().map(|b| format!("{:02x}", b)).collect())
}
