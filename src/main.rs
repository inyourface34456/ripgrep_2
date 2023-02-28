use std::env;

fn main() {
    let arg = env::args().collect::<Vec<String>>();

    if arg.len() != 3 {
        eprintln!("You must provide 3 arguments.");
    } else {
        let file = &arg[1];
        let query = &arg[2];

        println!("Searching for {} in file {}", query, file);
    }

    
}