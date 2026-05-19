use std::{fs, path::{Path, PathBuf}};

use crate::categories;

pub fn organize_files(entry:&fs::DirEntry, dry_run:bool) -> std::io::Result<()> {
    let path:PathBuf= entry.path();

    let extension: &str= categories::get_category(&path);

    let new_dir = Path::new("./test_folder").join(extension);
    let file_name= entry.file_name();

    let new_path: PathBuf= Path::new(&new_dir).join(&file_name);
    
    fs::create_dir_all(&new_dir)?;

    if new_path.exists() {
        println!("File already exists: {:?}", new_path);
        return Ok(());
    }

    if dry_run {
        println!("Would move {:?} -> {:?}", path, new_path);
    } else {
        fs::rename(&path, &new_path)?;
    }

    Ok(())
}