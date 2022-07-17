mod variables;
mod control;
mod loops;
mod ownership;
mod slice;

fn main() {
    println!("Hello, world!");
    
    // chapter on variables and elemental function declaration
    variables::run();

    // control flow
    control::run();

    // Loops
    loops::run();

    // Rust Ownership
    ownership::run();

    //
    slice::run();
}
