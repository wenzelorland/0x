/*
Primite Types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u126, i128 
Floats: f32, f64
Boolean: (bool)
Characters (char)
Tuples
Arrays (of fixed length)
*/

// Rust is a statically typed language, which means that it must know the types of variables at compile time, however, the compiler can usually infer what type we want based on the value and how we use it


pub fn run() {
    // Default i "i32"
    let x = 1;

    // Default is "f64"
    let z = 2.5;

    // Add explicit type
    let y: i64 = 454545454545;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean 
    let is_active: bool = true;

    // Boolean from expression
    let is_greater: bool = 10 > 5;

    // Character 
    let a1: char = 'a';
    let face: char = '\u{1F600}'; // smiley face character

    println!("{:?}", (x,y,z,is_active, is_greater, a1, face));


}