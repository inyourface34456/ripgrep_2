use std::{fs, process};
use regex::Regex;

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub case: bool,
}

struct Flags {
    case: bool,
    regex: bool,
}

#[cfg(target_os = "linux")]
const NUM_ARGS: usize = 3;

#[cfg(target_os = "windows")]
const NUM_ARGS: usize = 2;

impl<'a> Config<'a> {
    pub fn parce_args(args: &'a [String]) -> Self {
        let mut flag = Flags {case: false, regex: false};
        if args.len() <= NUM_ARGS {
            eprintln!("There must be greater then or equal to 3 arguments.");
            process::exit(2)
        } else {
            for i in args.iter() {
                let i = i.as_str();
                match i {
                    "-c" => flag.case = true,
                    "-r" => flag.regex = true,
                    _ => (),
                }
            }
            
            Self {
                query: &args[2],
                file_path: &args[1],
                case: flag.case,
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


pub fn search<'a>(query: &str, contents: &'a str, case: bool) -> Vec<&'a str> {
    let mut result = Vec::new();

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
