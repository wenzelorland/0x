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
    // instead of taking the mutable String slide, we pass in the env::Args iterator 
    // we need to annotate the lifetime here, since Args is an own type and we return a string slice
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // discarding the first element in the args iterator (it's only the path to the program)
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        // -> with this implementation we do not have to clone elements as the ownership of the 
        // itereator elements is automatically passed on


        // checks for the key "CASE_INSENSITIVE" in the set of environment variables and retrieves the corresponding value
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // if the key exists, the result will be OK containing the result, if key is not found then error is returned and value is set to false
        println!("{:?}", case_sensitive);

        // at this point Config takes ownership of the strings
        Ok(Config{query, filename, case_sensitive})
    }
}

// &str -> string slice
// We need to tie the lifetime of the result to one of the input variables´ lifetimes, since we are returning a reference here
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    /* NEW */
    contents
        .lines()
        .filter(|line| line.contains(query)) // filter only line that containt query string
        .collect() // Rust know which collection we want as it is specified in the return type

    // Since Rust applies lazy evaluation, the speed difference in using higher level concepts
    // such as Iterators vs loops is not meaningful, as they both compile donw to the same low level concepts

    /* OLD 
    let mut results = Vec::new();
    
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    */
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
