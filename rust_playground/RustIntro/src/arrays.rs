// Arrays - Fixed list where elements are the same data types

pub fn run() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Array numbers: {:?}", numbers); 
    
    // Get single value
    println!("Single Value from array: {} of length: {}", numbers[0], numbers.len());


    // creating a mutable array
    let mut numbers_mut: [i32; 6] = [1,2,3,4,5,6];

    numbers_mut[2] = 19;

    // Arrays are stack allocated 
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers_mut));
    // can also state -> use std::mem <- at the top of the file and then instead of std::mem, just use the function mem directly 
}