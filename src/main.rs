mod utils;
use utils::*;
use std::env;

fn main() {
    let arg = env::args().collect::<Vec<String>>();
    
    let args = Config::parce_args(&arg);
    println!("Searching for {} in file {}", args.query, args.file_path);
    let contents= args.read_file();
    dbg!("The file has the contents {}", contents);
}
