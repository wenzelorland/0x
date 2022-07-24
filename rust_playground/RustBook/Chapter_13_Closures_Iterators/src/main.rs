// Closures are like functions but they remain anonymous, i.e. they don't have names
// They can be passed around as variables and provided as inputs to function arguments
// They capture the variable inside the scope they are defined

use std::thread;
use std::time::Duration;

mod struct_and_closures;
mod rust_iterators;

fn simulated_expensive_calculation(intensity:u32) ->u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity:u32, random_number:u32) {
    
    // Defining a closure
    // A closure is defined by these pipesm | var |, where in between the pipes the input variables for the closure are defined
    // closures are anonymous functions, i.e. w/o any name associated to them
    
    // Type Inferences of Closures (not necessary to annotate)
    // you can leave out the type defintion of the inputs and outputs, as the Rust compiler will infer them directly
    // you can nonetheless specify them
    // if you leave them out, then you can only use the same type as input if calling the closure function multiple times  
    // i.e. closure defintions can only have one type inferred
    // not annotating them may have consequences for performance

    // Normal function are part of an explicit interface exposed to users
    // Closure are usually short and of narrrow context, so that the compiler is able to infer the types  
    println!("Less efficient implementation with closures \n");
    let expensive_closure = | num : u32| -> u32 {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    }; // the let statement stores the closure itself, not the return value of the closure!

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            //expensive_result
            expensive_closure(intensity)
        );
        println!(
            "Today, do {} situps!",
            //expensive_result
            expensive_closure(intensity)
        );
        
    }

    else {
        if random_number == 3 {
            println!("Take a break today!")
        } else {
            println!(
                "Today, run for {} minutes",
                //expensive_result
                expensive_closure(intensity)
            );
        }
    }

}

fn main() {
    println!("Hello to Closures of Rust");
    generate_workout(20, 2);
    struct_and_closures::generate_workout(20, 2);

    // Scoping of closures
    let x: i32 = 4;
    // x will be in the scope for the closure as it is defined in the same scope
    let equal_to_x = |z| z == x;
    // closures capture the environment they are defined in

    // closures are capturing the information from the environment in three different types
    // 1. Taking Ownership -> FnOnce Trait
    // 2. Borrowing Mutably -> FnMut Trait
    // 3. Borrowing Immutably -> Fn Trait
    // Rust automatically infers based on the logic of the closure which trait to associate to the closure

    // We can force the closure to take ownership of a variable    
    let w: i32 = 12;
    let equal_to_w = move |z: i32| z == w;

    let y = 4;
    assert!(equal_to_x(y));

    rust_iterators::run();

}
