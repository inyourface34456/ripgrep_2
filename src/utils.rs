use std::{fs, process};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn parce_args(args: &'a [String]) -> Self {
        if args.len() != 3 {
            eprintln!("There must be exatly 3 arguments.");
            process::exit(2)
        } else {
            Self {
                query: &args[2],
                file_path: &args[1],
            }
        }
    }

    pub fn read_file(&self) -> String {
        fs::read_to_string(self.file_path).unwrap_or_else(|err| {
            eprintln!("An error has ocurred ({err})");
            process::exit(2);
        })
    }

}


pub fn search<'a>(query: &str, contents: &'a str, case: Option<bool>) -> Vec<&'a str> {
    let mut result = Vec::new();
    let case = case.unwrap_or(false);

    if !case {
        for line in contents.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }
    } else {
        for line in contents.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                result.push(line);
            }
        }
    }
    result
}
