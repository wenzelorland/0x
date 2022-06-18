// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is block-scoped language

pub fn run() {
    let name : &str = "Greg"; // non-mutable variable
    let mut age : i16 = 37;
    
    println!("{} - Age: {}", name, age);

    // changing value of variable
    age = 38;

    println!("{} - Age: {}", name, age);

    // Define Contants
    const ID:i32 = 001;
    println!("ID: {}", ID);

    // Assigning multiple variables at once
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}",  my_name, my_age);

}