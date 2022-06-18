// Vectors - Arrays of arbitrary size and amendable length
use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("Vector numbers: {:?}", numbers); 
    
    // Get single value
    println!("Single Value from Vector: {} of length: {}", numbers[0], numbers.len());

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    println!("{:?}", numbers);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Vectors are stack allocated 
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut(){
        *x *= 2; // access the element directly which sits in the Vector, i.e. with *x you get the respective reference to the element w/o having to directly select it
    }

    // Same as:
    (0..numbers.len()).for_each(|i: usize| {
        numbers[i] *= 2;
    });

    println!("Numbers Vec {:?}", numbers);
}