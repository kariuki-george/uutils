use clap::Parser;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::fs::File;

#[derive(Debug, Parser)]
struct Args{
    // The input to be searched
    #[arg(long, short,name="INPUT")]
    input: String,

    // The files to search in
    #[arg(name="FILES")]
    files: Vec<String>

}


fn main(){
    let args = Args::parse();


    for  arg_file in args.files {
        let path = Path::new(&arg_file);

        assert!(path.exists(), "grep: {}: No such file or directory", path.display());
        assert!(path.is_file(), "grep: {}: No such file or directory", path.display());    


        let file = File::open(&path).unwrap();

        let bufreader = BufReader::new(file);

        for (line_no, line_result) in bufreader.lines().enumerate(){
            let line = line_result.unwrap();


           let index_option =  line.find(&args.input);

           match index_option {
               None=>{},
               Some(_)=>{
                println!("{}: Line {}: {}", path.display(),line_no, line.trim())
               }
           }

        }

        

    }

}