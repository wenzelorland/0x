pub fn run() {
    println!("Hello World to Rust Lifetimes");

    // Rust borrow checker runs at compile time and thus checks all references before eecuting the code
    // this is important as this avoids dangling references, i.e. references that interefere when looking at different scopes

    // since this is in the same scope, we do not have a dangling reference
    let x: i32 = 5;
    let r: &i32 = &x;
    println!("r: {}", r);

    // Generic Lifetime Annotations where string1 and string2 have the same lifetime
    let string1: String = String::from("abcd");
    let string2: String = String::from("xyz");

    let result: &str = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    // Now with different lifetimes of string3 and string4
    let string3: String = String::from("abcd"); // this will have lifetime until end of main function
    
    // let result; // this would not work, we need to define result within the scope, as we have annotated its lifetime in the longest function to be the same 
    {
        let string4: String = String::from("abcd");
        let result: &str = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result);
        // as this scope ends, the string4 lifetime also ends here
    }
}

// Lifetime Ellision Rules -> when to annotate lifetimes
// 1. Each parameter that is a reference gets its own lifetime parameter

// 2. If there is exactly one input lifetime parameter, that lifetime is
// assigned to all output lifetime parameters;

// 3. If there are multiple input lifetime parameters, but on eof them is 
// &self or &mzt self the lifetime of self is assigned to all output 
// lifetime parameters and we do not need to specify the limetime annotations for all parameters

// -> the compiler will try to follow these 3 rules and if it can, then we do need to specify lifetime annotations; 
// if not, then one has to specify the lifetime annotations

// &i32         // a reference
// &'a i32      // a reference with an explicit lifetime
// &'a mut i32  // a mutable reference with an explicit lifetime 

// since we do not know the lifetime of x and y ex-ante, we cannot just return type &str, as we need to annotate the lifetime
// we need to know which of the variable's lifetime will be used
// Generic lifetime annotations explain how the lifetimes of variables behave
// this always starts with an apostrophe or just  tick ' -> typically used as 'a


fn longest<'a>(x:&'a str, y:&'a str) -> &'a str { // return type will be a reference to a string 
    // with this annotation, we tell the borrow checker that x, y and the return reference have the same lifetime, i.e. the smallest one of them because of the if-else statement
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// Here we do not need to annotate the lifetime of y, since the return result reference is not dependant on y
// The lifetime of the return value, since it is a reference, needs always to be tied to one of the lifetime of the input values, as it will return the reference
fn longest_alt<'a>(x:&'a str, y:&str) -> &'a str { // return type will be a reference to a string 
    println!("{}", y);
    x
    
}