use std::env;
use std::process;

use Chapter_12_CLI::Config; //name of the library crate (where the lib.rs is stored)


// Redirecting Output of command line output to file
// cargo run > output.txt
// for this we need the eprintln! macro

fn main() {
    // this command collects the arguments thar are being passed to cargo run 
    let args: Vec<String> = env::args().collect();
    // cargo run test sample.txt

    // Constructing our Config struct
    let config = Config::new(&args).unwrap_or_else(|err| {
        // This following closure will be executed in the Error case of the Result type
        // the Ok case will just return the result of the function
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1); // this will terminate the program with the status code 1
    });

    eprintln!("Searching for {}", config.query);
    eprintln!("In file {}", config.filename);
    eprintln!("{:?}", args);

    if let Err(e) = Chapter_12_CLI::run(config) { // if the code run(config) results in an error, then run the code inside the if block
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

}

