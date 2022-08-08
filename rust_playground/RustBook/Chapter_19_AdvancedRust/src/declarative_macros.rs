// Macros allows code to write other code -> MetaProgramming
// Macros can accept a variable amount of variables as inputs, while function have to declare a defintive amount of variable as inputs
// Functions are called at runtime; Macros are expanded at compile time
// Macros add complexity to the code overall

// Declarative Macros
// Most widely used form in rust.


pub fn run() {
    // we can pass a variable amount of arguments
    // and we can pass different types as inputs
    let v1: Vec<u32> = vec![1,2,3];
    let v2: Vec<&str> = vec!["a", "b", "c", "d", "e"];
}