use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = args[1].clone();
        let filepath = args[2].clone();
        let ignore_case = match args.get(3) {
            Some(flag) if flag == "--ignore_case" || flag == "-i" => true,
            Some(_) => return Err("Unknown Arguments"),
            None => env::var("IGNORE_CASE").is_ok(),
        };
        Ok(Config {
            query,
            filepath,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_case_insenstive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insenstive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        let new_query = query.to_lowercase();
        let new_line = line.to_lowercase();

        if new_line.contains(&new_query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }

    #[test]
    fn case_insenstive() {
        let query = "rUsT";
        let contents = "
Rust:
safe, fast, productive.
Pck three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insenstive(query, contents)
        )
    }
}
