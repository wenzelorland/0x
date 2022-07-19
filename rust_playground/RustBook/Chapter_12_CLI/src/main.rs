use std::env;
use std::fs; // filesystem
use std::process;
use std::error::Error;

fn main() {
    // this command collects the arguments thar are being passed to cargo run 
    let args: Vec<String> = env::args().collect();
    // cargo run test sample.txt
    let config = Config::new(&args).unwrap_or_else(|err| {
        // This following closure will be executed in the Error case of the Result type
        // the Ok case will just return the result of the function
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // this will terminate the program with the status code 1
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!("{:?}", args);

    if let Err(e) = run(config) { // if the code run(config) results in an error, then run the code inside the if block
        println!("Application error: {}", e);
        process::exit(1);
    }

}

fn run(config:Config) -> Result<(), Box<dyn Error>> { // dyn Error means return any type of error
    // the ? takes care of the Error
    let contents: String = fs::read_to_string(config.filename)?; // the question mark will return the error in case the function call fails
        //.expect("Something went wrong reading the file"); // since read_to_string returns either a Ok(String) or an Error; if the Error is returned then we catch it with .expect
    
    println!("With text: \n{}", contents);
    Ok(())
}


struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query: String = args[1].clone(); // in order to avoid passing the ownership
        let filename: String = args[2].clone();
        Ok(Config{query, filename})
    }
}

