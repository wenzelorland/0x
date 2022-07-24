// This is the root of the library
use std::fs; // filesystem
use std::error::Error;
use std::env; // for environment variables handling


// Need the pub attribute as everything in Rust is private by default in order to use it in another crate
pub fn run(config:Config) -> Result<(), Box<dyn Error>> { // dyn Error means return any type of error
    // the ? takes care of the Error
    let contents: String = fs::read_to_string(config.filename)?; // the question mark will return the error in case the function call fails
        //.expect("Something went wrong reading the file"); // since read_to_string returns either a Ok(String) or an Error; if the Error is returned then we catch it with .expect
    
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    
    Ok(())
}


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool, 
}

impl Config {

    // constructor function
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // cloning the strings
        let query: String = args[1].clone(); // in order to avoid passing the ownership
        let filename: String = args[2].clone();

        // checks for the key "CASE_INSENSITIVE" in the set of environment variables and retrieves the corresponding value
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // if the key exists, the result will be OK containing the result, if key is not found then error is returned and value is set to false
        println!("{:?}", case_sensitive);
        Ok(Config{query, filename, case_sensitive})
    }
}

// &str -> string slice
// We need to tie the lifetime of the result to one of the input variables´ lifetimes, since we are returning a reference here
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results    
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


// Test Driven Development 
// Stage 1: Create a Failing test
// Stage 2: Implement logic so that test does not fail
// Stage 3: If necessary, restructure code base


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }


    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
    }
