#!/usr/bin/env cargo

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

fn rename_files_in_directory(path: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let prefix = "temp_rename_"; // A unique prefix to avoid name collisions

    // First pass: Rename all files to have a unique prefix
    for entry in &entries {
        if entry.is_file() {
            let new_name = format!("{}/{}{}", path, prefix, entry.file_name().unwrap().to_str().unwrap());
            fs::rename(&entry, &Path::new(&new_name))?;
        }
    }

    // Second pass: Rename all files to sequential numbers
    let mut counter = 1;
    let mut prefixed_entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    prefixed_entries.sort_by_key(|path| path.file_name().unwrap().to_owned());

    for entry in prefixed_entries {
        if entry.is_file() {
            let ext = entry.extension()
                           .and_then(OsStr::to_str)
                           .map_or(String::new(), |s| format!(".{}", s));

            let new_name = format!("{}/{}{}", path, counter, ext);
            fs::rename(&entry, &Path::new(&new_name))?;
            counter += 1;
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_directory>", args[0]);
        std::process::exit(1);
    }

    let path = &args[1];
    if let Err(e) = rename_files_in_directory(path) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_rename_files_in_directory() -> io::Result<()> {
        // Setup: Create a temporary directory and some dummy files
        let dir = tempdir()?;
        let dir_path = dir.path();

        File::create(dir_path.join("file1.txt"))?;
        File::create(dir_path.join("file2.txt"))?;
        File::create(dir_path.join("image.png"))?;

        // Execution: Rename files in the directory
        rename_files_in_directory(dir_path.to_str().unwrap())?;

        // Assertion: Check if files are correctly renamed
        let mut entries = fs::read_dir(dir_path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;
        entries.sort(); // Sort the entries to assert in order

        assert_eq!(entries[0].file_name().unwrap().to_str().unwrap(), "1.txt");
        assert_eq!(entries[1].file_name().unwrap().to_str().unwrap(), "2.txt");
        assert_eq!(entries[2].file_name().unwrap().to_str().unwrap(), "3.png");

        // Cleanup: Done automatically by the tempdir destructor

        Ok(())
    }
}
