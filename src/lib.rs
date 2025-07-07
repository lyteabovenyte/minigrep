use std::env;
use std::error::Error;
use std::fs;
use std::io::Lines;

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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get file_path string"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}


fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensetive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
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

    #[test]
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
