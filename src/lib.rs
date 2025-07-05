use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    if config.ignore_case {
        if search_case_insensetive(&config.query, &content).is_empty() {
            println!("No matches found for query: {}", config.query);
            return Ok(());
        }

        for (index, line) in content.lines().enumerate() {
            if line.to_lowercase().contains(&config.query.to_lowercase()) {
                println!("Found match at line {}: {}", index + 1, line);
            }
        }
    } else {
        if search(&config.query, &content).is_empty() {
            println!("No matches found for query: {}", config.query);
            return Ok(());
        }

        for (index, line) in content.lines().enumerate() {
            if line.contains(&config.query) {
                println!("Found match at line {}: {}", index + 1, line);
            }
        }
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.\n\tUsage: cargo run -- <query> <file_path>");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// TODO: could be better with iterators.
fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            v.push(line);
        }
    }
    v
}

pub fn search_case_insensetive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase(); // !risky, won’t be 100% accurate.
    let mut v = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            v.push(line);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensetive() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    fn case_insensetive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensetive(query, content)
        );
    }
}