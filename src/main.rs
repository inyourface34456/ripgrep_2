mod utils;
use utils::*;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result<'a>() {
        let args = Config {query: "duct", file_path: "src/test.txt", case: true};
        let s1 = args.read_file();
        let contents = s1.as_str();

        assert_eq!(vec!["Rust is productive"], search(args.query, contents, args.case));
    }

    #[test]
    fn case_insesative() {
        let args = Config {query: "rUsT", file_path: "src/test.txt", case: false};
        let s1 = args.read_file();
        let contents = s1.as_str();

        assert_eq!(vec!["coding in rust can be fun", "Rust is productive"], search(args.query, contents, args.case));
    }
}

fn main() {
    let arg = env::args().collect::<Vec<String>>();
    
    let args = Config::parce_args(&arg);
    let s1= args.read_file();
    let query = args.query;
    let contents = s1.as_str();

    let results = search(query, contents, args.case);

    for i in results {
        println!("{i}");
    }
}
