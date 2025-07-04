use std::env;
use std::fs;
fn main() {
    let config = Config::new(&env::args().collect::<Vec<String>>());
    let contents = fs::read_to_string(&config.file_path)
                            .expect("Something went wrong reading the file");
    println!("Text:\n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}