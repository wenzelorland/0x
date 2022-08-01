// << Patterns >>

// Patterns are a special syntax in Rust for matching against a structure of types
// They consist of 
// Literals, Destructed {Arrays, Enums, Structs, Tuples}, Variables, Wildcards and Placholders

// These components describe the shape of the data we are working with
// (x,y) = (2,8)

#[derive(Debug)]
enum Language {
    English,
    Spanish,
    Russian,
    Japanese,
}


fn main() {
    println!("Hello to patterns in Rust.");
    
    let language = Language::English;
    // match variable_to_match
    match language {
        // match arms consist of a pattern which maps to an expression
        // match expressions need to be exhaustive, i.e. we need to at least define a catch-all pattern
        Language::English => println!("Hello world!"),
        Language::Spanish => println!("Hola mundo!"),
        Language::Russian => println!("Privet mir"),
        // catch-all pattern; the underscore will not store the repsective value
        // _ => println!("Unsupported language!") 
        // we can also catch the exhaustive case and then print it
        // whatever variable was used will be stored in lang
        lang => println!("Unsupported language! {:?}", lang)
    }

    // Conditional if Let Expression
    let authorization_status: Option<&str> = None;
    let is_admin: bool = false;
    // contains either an u8 or an Error as Result Type
    let group_id: Result<u8, _> = "34".parse();

    // assigning a value to variable status 
    // if authorization_status is a Some() variant, then we want to take the string slice stored in authorization_status
    // and want to store it in stats and print it
    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    // if group_id is an Ok() variant, then we take group_id and map it to g_id and then we check if its larger than 30
    // if the variable inside the Pattern is named the same as the variable we are matching on, then the variable inside the pattern
    // will shadow the variable we are matching on; if if instead of g_id, we took group_id, then group_id in the pattern would shadow group_id 
    } else if let Ok(g_id) = group_id {
        if g_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }

    // ======================================
    //       While let conditional Loops
    // ======================================

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // pop will return an Optional()
    // at the end of the vector, stack.pop will return a None variant, which will stop the while loop
    while let Some(top) = stack.pop() {
        // if stack.pop is of variant Some, store the value inside top and print it
        // it will run as long as the specified Pattern amtches
        println!("{}", top);
    }

    // ======================================
    //              For Loops
    // ======================================
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // ======================================
    //          let Statements
    // ======================================
    // x is a pattern that matches any expression, so we set 
    let x :i32 = 5;

    // let PATTERN = EXPRESSION;
    // this pattern deconstructs a tuple
    // Rust will compare the tuple against the Pattern which is being used and then binding the corresponding values
    let (x,y,z) = (1, 2, 3);

    // ignoring values in pattern deconstruction 
    // the underscore will be disregarded
    let (x,y,_) = (1, 2, 3);

    // ======================================
    //       Function Parameters
    // ======================================
    let point = (3,5);
    print_coordinates(&point);

    // ======================================
    //   Irrefutable and Refutable Patterns
    // ======================================
    
    // Irrefutable
    // Pattern which will always match
    let x = 5;

    // Refutable
    // Patterns which may not match
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

    // can only accept irrefutable patterns:
    // function parameters
    // let statements
    // for loops

    let x: Option<&str> = None;
    // this would result in a refutatable pattern
    // however, the let statement expects an irrefutable pattern, as x above is described as refutable
    //let Some(x) =x;

    // this will always match, meaning the if let is useless here 
    if let x = 5 {
        println!("{}",x);
    }

}

fn print_coordinates(&(x,y):&(i32,i32)) {
    println!("Current location: ({}, {})", x, y);
}
