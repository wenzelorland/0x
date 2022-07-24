use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs::{self, File};


pub fn run(){
    // Panic Macro - this will stop the program running when reaching this line
    a();

    /* 
    // Recoverable Error Handling
    // This is where the Result Enum comes into play
    // and it looks like this
    // it is similar to the Option Enum, where the Option represents Some value or Null value
    // The Result Enum represents a Success Case with Ok and the Failure case with Err
    // Result Enum is brought into Scope by default
    
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    */
    let f: Result<File, std::io::Error> = File::open("hello.txt"); // Open returns a Result Type to handle errors


    let f: Result<File, std::io::Error> = Ok(File::open("hello.txt").unwrap()); // Open returns a Result Type to handle errors

    let f: Result<File, std::io::Error> = Ok(File::open("hello.txt").expect("Failed to open the file")); // Open returns a Result Type to handle errors

    // Handling the success case and failure case
    // shadowing f to redeclare f
    let f: File = match f {
        Ok(file) => file,
        
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        // Instead of panicking, we create a file if it does not exists yet
         Err(error) => match error.kind(){
            // since creating a file returns a Result Enum as well, we need to handle the success and failure cases again with a match statement
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {:?}", e),
            },
            // since match statements are conclusive, we need to define the handling of all other match types
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
         }
    };
    
    // Using Closures to achieve the same as with above nested match statements
    let f: File = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            // this is an expression, since there is no semicolon at the end
            File::create("hello.txt").unwrap_or_else(|erorr|{
                panic!("Problem creating the file {:?}", error);
            })
            }
            else {
                panic!("Problem opening the file: {:?}", error);
            }
    });

    /* Error Propagation and the ? Operator */
    read_username_from_file();
}

fn read_username_from_file() -> Result<String, io::Error> {
    /* Eroor Propagation and the ? Operator -> Automataed Error Handling */ 
    // let f: File::open("hello.xtx");
    //
    // let mut f = match f {
    // Ok(file) => file,
    // Err(e) => return Err(e),
    // };
    
     
    let mut s = String::new();
    let mut f = File::open("hello.txt")?; // this will return an error if the function call failes

    f.read_to_string(&mut s)?;
    //Ok(s)
    

    // same as above but with chained expressions
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    //Ok(s) // Ok() since we need to account for the success and failure event

    // or even simpler
    let s: Result<String, io::Error> = fs::read_to_string("hello.txt");
    s // we do not need the Ok() here, as we directly declare the type of s in the let statement
}
    
    // shadowing f to 

fn a() {
    b();
}

fn b(){
    c(21);
}

fn c(num: i32){
    if num == 22 {
        panic!("Don't pass in 22!");
    }

}


