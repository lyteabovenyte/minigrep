use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    if search(&content, &config.query).is_empty() {
        println!("No matches found for query: {}", config.query);
        return Ok(());
    }
    
    for (index, line) in content.lines().enumerate() {
        if line.contains(&config.query) {
            println!("Found match at line {}: {}", index+1, line);
        }
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.\n\tUsage: cargo run -- <query> <file_path>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

// TODO: could be better with iterators.
fn search<'a> (query: &str, content: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            v.push(line);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], 
                   search(query, content));
    }
}