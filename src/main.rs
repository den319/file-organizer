use std::{fs};

use clap::Parser;

mod categories;
mod organizer;
mod config;

fn main() -> std::io::Result<()> {
    let config= config::Config::parse();

    if config.dry_run {
        println!("Dry run enabled");
    }

    let entries= fs::read_dir(&config.path)?;

    for entry in entries {
        let entry: fs::DirEntry = match entry {
            Ok(entry) => entry,
            Err(err) => {
                eprintln!("Error reading entry: {}", err);
                continue;
            }
        };

        if let Ok(file_type)= entry.file_type() {
            if file_type.is_dir() {
                continue;
            }
        }

        if let Err(err)= organizer::organize_files(&entry, config.dry_run) {
            eprintln!("Failed to organize {:?}: {}", entry.path(), err);
        }

        // println!("{:?} {:?}", config, entry.file_name());

    }

    Ok(())
}
