use std::{env, fs, process};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn parce_args(args: &[String]) -> Self {
        Self {
            query: args[2],
            file_path: args[1],
        }
    }

    fn read_file() {
        
    }
}

/*fn parce_args(args: &[String]) -> Config {
    let query = &args[2];
    let file = &args[1];

    (file, query)
}

fn content_fetcher(path: &str) -> String {
    match fs::read_to_string(path) {
        Err(e) => {
            eprintln!("An error has ocurred ({}).", e);
            process::exit(2);
        },
        Ok(o) => o,
    }
}*/

fn main() {
    let arg = env::args().collect::<Vec<String>>();

    if arg.len() != 3 {
        eprintln!("You must provide 3 arguments.");
    } else {
        let (file, query) = parce_args(&arg);
       

        println!("Searching for {} in file {}", query, file);

        let contents= content_fetcher(file);
        };

        dbg!("The file has the contents {}", contents);
    }

    
}