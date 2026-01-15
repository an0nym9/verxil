use std::error::Error;
use std::path::Path;

use crate::commands::commit::list_files;

/// Check the current status of pre-commited and commited files
pub fn check_status() -> Result<(), Box<dyn Error>> {
    let staged_files = if Path::new(".verxil/index").exists() {
        list_files(".verxil/index")?
    } else {
        Vec::new()
    };

    let committed_files = if Path::new(".verxil/objects").exists() {
        list_files(".verxil/objects")?
    } else {
        Vec::new()
    };

    fn print_files(title: &str, files: &[String]) {
        println!("{}:", title);
        if files.is_empty() {
            println!("\t- Empty");
        } else {
            for file in files {
                println!("\t- {}", file);
            }
        }
    }

    print_files("Staged files", &staged_files);
    print_files("Committed files", &committed_files);

    Ok(())
}
