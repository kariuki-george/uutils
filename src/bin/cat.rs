/// Print a file contents

use std::env;
use std::{fs,io};


fn main(){
    // Get the file from the user
    let filepath: Vec<String> = env::args().collect();
    // Check if file is passed

    assert!(!(filepath.len() ==1), "File to cat not provided" );
   
    let filepath:&str = &filepath[1];

    // Check if the file exists


    fn parse_error(e: io::Error){
        match e.kind() {
            io::ErrorKind::NotFound => println!("The file specified not found"),
            _ => println!("An error occurred")
        }

    }

    match fs::read_to_string(filepath){
        Ok(content) => println!("{}",content),
        Err(e) => parse_error(e),
    }


}