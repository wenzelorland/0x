// --- Ownership Rules ---
// 1. Each value in Rust has a variable that's called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// --- Rules of References ---
// 1. At any given time, you can have either one mutable reference
// or any number of immutable references.
// 2. References must always be valid.

pub fn test_scope(){

    { // s is not valid here, it's not yet declared
        let s: &str = "Hello"; // s is valid from this point forward - since this is a string literal, it will be allocated to the Stack
        // do stuff with s
        let y: String = String::from("Hello"); // allocated to the heap, as this String is of variable size and mutable
    } // this scope is now over, and s is no longer valied

    let x: i32 = 5;
    let y: i32 = x; // Copy, since for primitive types, this assignment corresponds to a copy, not a move

    let s1: String = String::from("Hello");
    //let s2: String = s1; // Move (not shallow copy); s1 is not valid anymore 
    let s2: String = s1.clone(); // This would be a copy of the variable s1; 

    println!("{},  world!", s1); // if s1 is only moved, then this would result into an error, as s1 is now moved to s2
}


pub fn run(){
    // Rust memory Stack and Heap
    // Stack is of fixed size and stores stack frames for functions and the local variables of the functions of which size is known
    // - during runtime, the program has access to the stack, as well as the heap
    
    // Size of stackframe is calculated at compile time; memory of stack frame lives only as long as it is executed; once execution is done it is popped from the stack
    // Heap can grow or shrink at runtime; the data that resides in the heap can be of variable size and can resize for any time, as long as we want

    // Once a scope is being left after execution, Rust automaticcaly deallocates the memory Heap that was allocated to that scope

    fn a() {
        let x: &str = "hello"; // stored in local stack frame within the Stack, as this is a string literal, which is directly stored in binary and on the Stack of fixed size
        // with x being the reference of the fixed binary length string 
        let y: i32 = 22; // stored in local stack frame
        b();
    }

    fn b(){
        let x: String = String::from("world"); // stored within the Heap, as String type is of variable size and 
        // this will ask the Heap to allocate memory to this String and the Heap will return a pointer to x which points to the String on the Heap

        // Allocating space in the Heap takes longer than defining a Stack allocation
        // Accessing the stack stored data is much faster than on the Heap, as for the Heap one needs to follow the pointer

    }

    // Ownership and Functions
    let s: String = String::from("hello");
    //takes_ownership(s); // this does not work, as the variable s is moved into the function (as providing functions with input is the same as assigning new variables)
    // since s is not a primite type, it is moved and once it leaves the scope of takes_ownership, it is never returned, thus it is not accessible in the previous scope after takes_ownership has been executed
    // this works, since we pass a copy of s, so it is maintained within the current scope after execution
    takes_ownership(s.clone());
    
    // this works, as we pass a reference to s, which enables borrowing ownership, before passing it back once execution is done
    borrows_ownership_by_reference(&s);
    // this works
    println!{"{}", s}; 

    let i: i32 = 100;
    takes_copy(i); // this works, as this is of primitve type and i is being copied to the scope of takes_copy

    let s1: String = gives_ownership();
    let s2: String = String::from("hello back");
    let s3: String = takes_and_gives_back(s2);
    println!("s1 = {}, s3= {}", s1,s3);

    let len: usize = calculate_length(&s1); // send the reference to s1, where s1 is the pointer to the String which is stored on the Heap to the function 
    let mut s4: String = String::from("hello");
    change(&mut s4); // this will change the value of s4 without taking over ownership of s4

    let r1 = &s4;
    //let r2 = &mut s4; // not possible, as Rust only allows once mutable reference to a mutable variable within one scope: this is because avoiding race conditions
    let r2 = &s4;

    // we cannot mix mutable and immutable references on the same variable if they are in the same execution scope
    println!("{}, {}", r1,r2);
    let r3 = &mut s4; // this works, as this is now after the scope of r1 and r2, which are immutable references

}

fn takes_ownership(some_string:String){
    println!("{}", some_string);
}

fn borrows_ownership_by_reference(some_string:&String){
    println!("{}", some_string);
}

fn takes_copy(some_integer:i32){
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}

// takes in a reference to a String
fn calculate_length(s: &String) -> usize {
    // the referenced variable will be borrowed for this scope and once it is executed, the borrowed variable will remain in the original scope
    // !!! referenced (i.e. borrowed) variables are immutable within the scope where they are borrowed !!!
    let length: usize = s.len();
    //s.push_str(", world"); // this cannot be done, as s is only borrowed within this scope
    length
}

fn change(some_string: &mut String) {
    // chaning the value of some_string which is borrowed, w/o taking ownership of some_string
    some_string.push_str(", world"); // here we passed a mutable reference of a String, which can then be altered within the scope it is borrowed to

}