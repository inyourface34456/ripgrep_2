mod utils;
use utils::*;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result<'a>() {
        let args = Config {query: "duct", file_path: "test.txt"};
        let s1 = args.read_file();
        let contents = s1.as_str();

        assert_eq!(vec!["safe, fast, productive."], search(args.query, contents));
    }
}

fn main() {
    let arg = env::args().collect::<Vec<String>>();
    
    let args = Config::parce_args(&arg);
    let s1= args.read_file();
    let query = args.query;
    let contents = s1.as_str();

    let results = search(query, contents);

    for i in results {
        println!("{i}");
    }
}
