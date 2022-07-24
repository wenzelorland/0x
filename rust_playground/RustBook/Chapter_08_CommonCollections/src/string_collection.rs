    // Strings are stored as a collection of UTF-8 encoded bytes
    // strings in Rust are very complicated in low level languages
    // Strings are stored as lists of zeros and ones (binary)
    // In order to make them human readable, one needs to perform a corresponding encoding set 
    // e.g. ASCII or Unicode 
    // Unicode stands for any characters, not just the english language 
    // UTF-8 is a variable width character encoding
    // each character can represented in 1,2,3 or 4 bytes
    // in ASCII, a character can only be represented by one byte 

    // A string thus is just a collection of bytes!

pub fn run(){

    let s1: String = String::new();
    let s2: &str = "Initial String"; //string slices
    let s3: String = s2.to_string(); // turn string slice into an own String
    let s4: String = String::from("initial contets"); 

    let mut s: String = String::from("foo");
    s.push_str("bar");
    s.push('!');
    println!{"{}",s};

    let s2: String = String::from("world!");
    
    // String Formatting with format macro
    let s5: String = format!("{}{}", s1,s2); // since format! macro does not take ownership, we can still use s1 and s2 after this statement
    
    
    // taking the reference of s2 and concatenating it with s1, where we take ownership of s1 string
    let s3: String = s1 + &s2;
    println!("{}", s3);

    // not possible since s1 ownership has been moved to s3!
    //println!("{}", s1);  

    // String Indexing
    let hello: String = String::from("Hello");
    // let c: char = hello[0]; // this tries to reference the first entry in the string, however, since it is represented in bytes we try to pick the first byte
    for b in hello.bytes() {
        println!("byte {}", b);
    }    
    
    for c in hello.chars() {
        println!("char {}", c);
    }    




}