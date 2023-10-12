#!/usr/bin/env cargo

use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn rename_files_in_directory(path: &str) -> io::Result<()> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    let mut counter = 1;
    for entry in entries {
        if entry.is_file() {
            let ext = entry.extension()
                           .and_then(std::ffi::OsStr::to_str)
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
