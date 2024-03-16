use std::env;
use std::error::Error;
use std::fs;
// config structure
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // creates a config instance
    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();
        // initing fields, throws error if no arg
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        // if is OK we ignore case while searching
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        // return a Config
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
// run a program according due to config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // read file, throws error on any problem
    let contents = fs::read_to_string(config.file_path)?;

    // get results according to ignore_case value
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    // print every line of result
    for line in results {
        println!("{line}");
    }

    Ok(())
}
// search query in lines of text and returns theese lines 
// case sensetive
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// search query in lines of text and returns theese lines 
// case insensetive
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    // query to lowercase
    let query = query.to_lowercase();
    let mut results = Vec::new();
    // push line to results vector if it contains a query
    for line in contents.lines() {
        // line to lowercase
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    // test if search function is sensetive to case
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // test if search_insensative function is insensetive to case
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}