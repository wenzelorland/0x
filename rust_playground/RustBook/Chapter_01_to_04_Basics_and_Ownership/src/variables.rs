pub fn run() {
    // variables are immutable by default; setting a mut in front of a variable, makes it mutable
    let mut x: i32 = 5;
    println!{"The value of x is : {}", x};
    
    // value of variable can be changed on the fly, once it's of mutable type
    x = 10; 
    println!{"The value of x is : {}", x};

    // Contants - Are immutable ( cannot be made mutable with mut)
    // contants must be type annotated - variables can also be inferred.
    // Constants cannot be assigned to be results of function calls at runtime.
    const SUBSCRIBER_COUNT : u32 = 100_000; // rust let's you format numbers with underscores for better readability
    
    println!{"The value of constant SUBSCRIBER_COUNT  is : {}", SUBSCRIBER_COUNT};
    
    // Shadowing 
    // Allows you to create a new variable by using an existing name
    let x: &str = "New value";
    println!{"The value of x is : {}", x};

    // Data Types
    // Integers
    let a : i32 = 98_222; // Decimal
    let b : i32 = 0xff;    // HEX
    let c : i32 = 0o77; // Octal
    let d : i32 = 0b1111_0000; // Binary
    let e : u8 = b'A'; // Byte (u8 only)
        
    println!{"The value of a is : {}", a};
    println!{"The value of b is : {}", b};
    println!{"The value of c is : {}", c};
    println!{"The value of d is : {}", d};
    println!{"The value of e is : {}", e};

    // integer overflow -> will take the minimum value of the range, if an overflow occurs
    let f: u8 = 255; 
    println!{"The value of f is : {}", f};
    
    // Floating-point numbers
    let f: f64 = 2.0;
    let g: f32 = 1.5;
    
    // Booleans
    let t: bool = true;
    let t_f: bool = false;
    println!{"The value of t is : {}, t_f is {}", t, t_f};
    
    // Character
    // with singe quotation marks
    let ch : char = 'a';

    println!{"The value of char is : {}", ch};


    // Compound Types //
    // Tuple
    let tup: (&str, i32) = ("Channel1", 20);
    // accessing values in tuple through unpacking
    let (channel, some_int) = tup;
    
    // accessing items in tuple through dot notation
    let tup_integer :i32 = tup.1;

    // Arrays - are of fixed length
    let error_codes : [i32; 3] = [200, 404, 500];
    let not_found : i32 = error_codes[1];
    println!{"Array error_codes: {:?}", error_codes};
    

    // Functions //+
    let sum :i32 = my_function(100, 101);
    
    println!{"Value of sum of x and y = {}", sum};
    // Statements vs Expression
    // Statements perform some code action, but do not return a value
    // Expressions return a value, after executing some code
 
}


fn my_function(x:i32, y:i32) -> i32{
    println!{"Value of x= {}, Value of y = {}", x,y};
    // can return a value in two types
    // with a specific expression
    return x+y;
    // or: last expression does not need a return statement and semicolon
    // x+y
}
