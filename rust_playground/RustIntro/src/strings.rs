// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heal-allocated data structure - Use when you need to modify or own string data

pub fn run (){
    // Primitive str
    let hello = "Hello";

    // String (growable)
    let mut hello_growable = String::from("Hello (growable)");

    println!("{} - {}", hello, hello_growable);

    // Get length
    println!("{}", hello_growable.len());

    // Extending the string by single characters
    hello_growable.push(' ');

    // Extending the string by single characters
    hello_growable.push('W');

    // Extending the string by strings
    hello_growable.push_str("orld!");

    println!("{}", hello_growable);

    // Capacity in bytes
    println!("Capacity: {}", hello_growable.capacity());

    // Check if empty
    println!("Is Empty: {}", hello_growable.is_empty());

    // Contains
    println!("Contains 'World' {}", hello_growable.contains("World"));

    // Replace
    println!("Replace: {}", hello_growable.replace("World", "there"));

    // Loop through string by whitespace
    for word in hello_growable.split(' '){
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing

    assert_eq!(2, s.len()); // will only show error when it fails

    assert_eq!(10, s.capacity()); // will only show error when it fails

}