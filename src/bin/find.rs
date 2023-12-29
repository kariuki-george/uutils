/// Print a file contents


use clap::Parser;
use std::path::Path;

// Find implementation to search for files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Name of the file
    #[arg(short, long, name="NAME",default_value="")]
    name: String,

    #[arg( name = "PATH", default_value=".")]
    path: String
}


fn print_files(path: &Path, file_name: &str){

    let entries = path.read_dir().unwrap();

    for entry in entries{
        match entry{
            Err(err)=> panic!("{:?}",err),
            Ok(entry)=>{
                if entry.path().is_dir(){

                  print_files(entry.path().as_path(),file_name);
                }else{
                    
                    if entry.path().ends_with(file_name){
                        println!("{:?}",entry.path());
                    }
                }
            }
        }
    }
}






fn main(){
    let  args: Args = Args::parse();

    // Assert that the directory provided exists
    let  path  = Path::new(&args.path);
    assert!(path.exists(),"find: '{}' : No such file or directory",args.path);


    print_files(path, &args.name );
  

   
}