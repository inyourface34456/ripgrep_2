mod utils;
use utils::*;
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let args = Config {query: "duct", file_path: "test.txt"};
        
        assert_eq!(vec!["safe, fast, productive."], args.search());
    }
}

fn main() {
    let arg = env::args().collect::<Vec<String>>();
    
    let args = Config::parce_args(&arg);
    let contents= args.read_file();
}
