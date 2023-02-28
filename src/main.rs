use std::{env, fs, process};

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
}

impl<'a> Config<'a> {
    fn parce_args(args: &'a [String]) -> Self {
        Self {
            query: &args[2],
            file_path: &args[1],
        }
    }

    fn read_file(&self) -> String {
        match fs::read_to_string(&self.file_path) {
            Err(e) => {
                eprintln!("An error has ocurred ({}).", e);
                process::exit(2);
            },
            Ok(o) => o,
        }
    }
}

fn main() {
    let arg = env::args().collect::<Vec<String>>();

    if arg.len() != 3 {
        eprintln!("You must provide 3 arguments.");
    } else {
        let args = Config::parce_args(&arg);
       

        println!("Searching for {} in file {}", args.query, args.file_path);

        let contents= args.read_file();

        dbg!("The file has the contents {}", contents);
    }
}
