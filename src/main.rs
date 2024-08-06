#!/usr/bin/env cargo

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::Path;

fn rename_files_in_directory(path: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .filter(|e| {
            e.path().is_file() && !e.file_name().to_str().map_or(false, |s| s.starts_with("."))
        })
        .map(|res| res.path())
        .collect::<Vec<_>>();

    let prefix = "temp_rename_"; // A unique prefix to avoid name collisions

    // First pass: Rename all files to have a unique prefix
    for entry in &entries {
        let new_name = format!(
            "{}/{}{}",
            path,
            prefix,
            entry.file_name().unwrap().to_str().unwrap()
        );
        fs::rename(&entry, &Path::new(&new_name))?;
    }

    // Second pass: Rename all files to sequential numbers
    let mut counter = 1;
    let mut prefixed_entries = fs::read_dir(path)?
        .filter_map(|res| res.ok())
        .filter(|e| {
            e.path().is_file()
                && e.file_name()
                    .to_str()
                    .map_or(false, |s| s.starts_with(prefix))
        })
        .map(|e| e.path())
        .collect::<Vec<_>>();
    prefixed_entries.sort_by_key(|path| path.file_name().unwrap().to_owned());

    for entry in prefixed_entries {
        let ext = entry
            .extension()
            .and_then(OsStr::to_str)
            .map_or(String::new(), |s| format!(".{}", s));

        let new_name = format!("{}/{}{}", path, counter, ext);
        fs::rename(&entry, &Path::new(&new_name))?;
        counter += 1;
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
        File::create(dir_path.join(".DS_Store"))?;
        File::create(dir_path.join(".hidden_file"))?;
        fs::create_dir(dir_path.join("subdir"))?; // Create a subdirectory

        // Execution: Rename files in the directory
        rename_files_in_directory(dir_path.to_str().unwrap())?;

        // Assertion: Check if files are correctly renamed
        let mut entries: Vec<_> = fs::read_dir(dir_path)?
            .filter_map(|res| res.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path())
            .collect();
        entries.sort(); // Sort the entries to assert in order

        assert_eq!(entries.len(), 5); // Total number of files (including .DS_Store and .hidden_file)
        assert_eq!(
            entries[0].file_name().unwrap().to_str().unwrap(),
            ".DS_Store"
        );
        assert_eq!(
            entries[1].file_name().unwrap().to_str().unwrap(),
            ".hidden_file"
        );
        assert_eq!(entries[2].file_name().unwrap().to_str().unwrap(), "1.txt");
        assert_eq!(entries[3].file_name().unwrap().to_str().unwrap(), "2.txt");
        assert_eq!(entries[4].file_name().unwrap().to_str().unwrap(), "3.png");

        // Check if subdirectory still exists and is not renamed
        assert!(dir_path.join("subdir").exists());
        assert!(dir_path.join("subdir").is_dir());

        Ok(())
    }
}
