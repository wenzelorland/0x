/*  # Concept of Pointers
 A pointer is a general concept for a variable 
 that stores a memory address, which points to some data in memory.
 The most common pointer in Rust is the reference.

 Smart Pointers are data structures that act like a pointer that have meta data and extra capabilities.
 E.g. a reference counting smart pointer which allows a piece of data to have multiple owners by keeping track of the owners.

In many cases, smart pointers own the data which they point to.
Cases of smart pointers are: Strings and Vectors. As they own some data and allow to mutate it and hold metadata such as meta data and hold 
extra capabilities.

Smart Pointers are implemented using structs but with the Deref and Drop Traits.
The Deref Trait allows instances of the Smart Pointer struct to be treated as references, so you can write code that works with eiether references and smart pointers.
The Drop Trait allows to customise the code that is run when an instance of the Smart Pointer goes out of scope.


Box is such smart pointer.

*/ 

// Recursive Enum
// we do not know ex-ante how much space this Enum will take up
// Rust will not be able to determine how large the first variant will be, as the List can be of infinite length
// fixign this with Box<List> will fix this, as the Box will be a pointer to the list, instead of duplicating the lists
enum List {
    Cons(i32, Box<List>), // this is a specific data structure -> cons list where the first element of the variant is the value and the second value is the pointer to a preceding list of again the same pattern
    Nil, // denotes the end of the list; cons-lists are not frequently used in Rust and one is better of with Vectors
}

// Rust figuring out the size of the Enum Message with 4 variants:
// Going through each variant and determining what size each variant will take
// it will rank them and figure out what varaint will take up the most place
// the size of the Enum will be the size of the largest size variant
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

use List::{Cons, Nil};

fn main() {
    // Boxes do not have any meaningful overhead
    // One would typically use them when size of type cannot known at compile time 
    // and you want to use the type in another context and it may be of large size
    // then you want to make sure that when the ownership changes, that you do not have to copy that data

    // if you own a value and you only care that a specfic trait is implemented rather than it being a specific type -> Trait Object

    let b = Box::new(5); // 5 will be stored on the Heap, while b, the smart pointer will be stored on the Stack
    println!("Hello to Rust Smart Pointers!");
    println!("b = {}", b); // we use the Box value as if it would be a value on the Stack

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
// here b will be deallocated from the Stack and the value stored on the Heap will also be deallocated
