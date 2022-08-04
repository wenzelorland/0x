// Valid Pattern Syntax

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    Changecolor(i32, i32, i32),
}

enum Message_Adv {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    Changecolor(Color),
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32,i32,i32),
}



pub fn run() {
    // Literal Match
    let x = 1;
    // when you want to execute your code on a given concrete value
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // matching on named variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // here if x is Some other variant, then y will take this value, but only in this scope, as the match statement creates its own scope
        // y will shadow y outside the scope and may have another value
        // printing y will not result in 10, as it will take any value that is defined in the Some variant other than Some(50)
        Some(y) => println!("Matched, y = {;?}", y),
        // if x is not of Some type, then it will just default to its own value
        _ => println!("Default case, x = {:?}", x),
    }

    // matching multiple pattern
    let x = 1;
    match x {
        1 | 2 | 3 => println!("one or two or three"),
        4 => println!("four"),
        5..=10 => println!("5 to 10"),
        _ => println!("anything"),
    }


    // can also use the range operator for character
    let y = 'c';
    match y {
        'a'..='j' => println!("early ASCII letter"),
        _ => println!("all other ASCII"),
    }

    let p = Point{x:0, y:7};
    // deconstructing the struct through patterns, where the names of the fields in the pattern do not have to match the names of the fields inside the struct
    let Point {x: a, y:b} = p;
    // however, it is commong to have the names in the name pattern match the names in the struct fields
    // a and b are being construcuted here and assigned to the correpsonding field values of the deconstructed struct
    println!("a: {}, b: {}", a, b);

    let p = Point{x:0, y:7};
    let Point {x, y} = p;
    println!("x: {}, y: {}", x, y);


    let p = Point{x:0, y:7};

    match p {
        Point {x, y:0} => {
            println!("On the x axis at {}", x)
        },
        Point {x:0, y} => {
            println!("On the y axis at {}", y)
        },
        Point {x, y} => {
            println!("On neither axis at ({}, {})", x, y)
        },
    }

    let msg = Message::Changecolor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("Quit");
        }
        // destruct the fields into local variables x and y and print them
        Message::Move {x, y} => {
            println!(
                "Move to x: {} y: {}", x,y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text)
        },
        // if variant is Changecolor, then we destruct the fields into local varaibles r, g and b
        Message::Changecolor(r, g, b) => {
            println!(
                "Change color: red {}, green {}, and  blue {}", r,g,b
            )
        }

    }
    
    // Nested Deconstruction
    let msg_adv = 
        Message_Adv::Changecolor(
                Color::Hsv(0, 160, 255));

    match msg_adv {
        // first variant matches the nested Color RGB variant, and we bind the fields inside the Rgb variant to r, g, b
       Message_Adv::Changecolor(Color::Rgb(r, g, b)) => {
            println!(
                "Change color: red {}, green {}, and  blue {}", r,g,b
            )
       }
       Message_Adv::Changecolor(Color::Hsv(h, s, v)) => {
            println!(
                "Change color: hue {}, saturation {}, and value {}", h,s,v
            )
        }
        _ => (),
    }

    // Ignoring Values in Pattern
    foo(3,4);

    // Ignorign part of a value
    let mut setting_value = Some(5);
    // Optionals containing an integer
    let new_settings_value = Some(10);

    match (setting_value, new_settings_value) {
        
        // A tuple of two Some variants that matches to a Some variant that
        // if both setting_value and new_settings_value are of Some variant then this match statement is corrent and we print the print statement 
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        // catch-all arm
        _ => {
            setting_value = new_settings_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // pre-fixing the variable name with an underscore still binds the value, and the compiler won't complain about the variable if it is not used
    let _x = 5;

    // Optional variable
    let s = Some(String::from("Hello"));

    // if s is Some variant we want to print a string
    // but we don't care about the content of the Optional
    // thus using an _ we can avoid moving ownership of s, since otherweise, the String contained in s -> String::from("Hello") would be moved into the variable in the Some(_s) statetment
    // -> instead of let Some(_s) -> Some(_)
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // Only interested in one part of a pattern
    // using the range syntax ..
    let origin = Point {x:0 , y:0, z:0};
    match origin {
        Point{x, ..} => println!("x is {}", x),
    }

    let numbers = (2,3,4,5);
    match numbers {
        // binding first and last values to variables first and last while discarding all others
        (first, .., last) => {
            println!("Some numbers: first {} last {} ", first, last);
        }
    }

    // Match Guard
    // this represents an additional if-condition specified after the pattern in a match arm that must also match along with the pattern for the arm to be chosen
    let num = Some(4);

    match num {
        // matching on a Some variant and matching the value to x
        Some(x) if x<5 => println!("less than five: {}", x),
        Some(x) => println!("{}",x),
        None => (),
    }

    // Optional containing the integer 5
    let x = Some(5);
    let y = 10;

    match x {
        // execute some piece of code, when x equals y
        // we cannot use the y inside the pattern block, as the y will shadow the y outside this block
        // i.e. we cannot do Some(y) and that's it since then y will shadow the y outside the block -> and we would thus bind the matching value
        // match on any Some variant, bind the value to n and make sure that n is equal to y
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    println!("Value of x : {}",x);


    // BINDINGS and the @ operator
    // the @ operator allows to test the variable and also bind the value to a variable 
    enum Message_small {
        Hello {id:i32},
    }
    
    let msg = Message::Hello {id: 5};
    match msg {
        Message::Hello {
            // the pattern sattisifes that the id is between 3 and 7 and we want to store that id in the variable id_variable
            // -> syntax of binding: matching arm variable : to_be_stored_into @ pattern
            // we can also use the same name of binding variable 
            id: id @ 3..=7,
        } => println!("Found an id in range : {}", id),
        Message::Hello {id: 10..=12} => {
            println!("Found an id in another")
        }
        Message::Hello{id} => {
            println!("Found some other id: {}", id)
        }
    }
// when a function is provided with an _ as an function argument, this argument will be ignored when passed in
fn foo(_: i32, y:i32) {
    println!("This code only uses the y parameter: {}", y);
}
}