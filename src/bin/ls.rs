use chalk::colors::Colors::*;
use chalk::Chalk;
/// Lists all the contents of a directory
use std::{env, path};

fn main() {
    // Get the file from the user
    let mut filepath: Vec<String> = env::args().collect();

    // If no path, default to current directory

    if filepath.len() == 1 {
        filepath.push(".".to_string());
    }

    let filepath = path::Path::new(&filepath[1]);

    // Check if a path is a directory
    assert!(filepath.is_dir(), "No such directory");

    let entries = filepath.read_dir().unwrap();

    for entry in entries {
        match entry {
            Ok(entry) => {
                if entry.path().is_dir() {
                    let colored_entry =
                        Chalk::new(Blue, entry.file_name().to_str().unwrap()).color();
                    print!("{} ", colored_entry);
                } else {
                    print!("{} ", entry.file_name().to_str().unwrap());
                }
            }
            Err(e) => {
                panic!("{:?}", e);
            } // Maybe println
        }
    }
    println!()
}
