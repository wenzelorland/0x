use std::thread;
use std::time::Duration;

// Cacher Struct
// In order to use Closures in Structs, Traits and Enums we need to use Generic Types and Trait bound
struct Cacher<T> // of Generic Type T
// Trait bounds
where 
    T: Fn(u32) -> u32, // Fn trait is part of the standard library
{
    calculation: T, // stores the generic type T, it can be any closure that meets the trait bound above T: Fn(u32) -> u32
    value: Option<u32>, // optional u32 integer, the return value of the closure will be stored here
    // such construct is also used for regular functions
}

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32, // Fn trait is part of the standard library
{   
    // constructor function -> create a new Cacher
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    // method since first parameter is a reference to self, here a mutable reference
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v: u32 = (self.calculation)(arg);
                self.value = Some(v); // mutating the value field of the current instance to the current value -> this is where the caching happens
                v
            }
        }
    }
}
pub fn generate_workout(intensity:u32, random_number:u32) {
    
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

    // wrapping the closure definition isnide the Cacher struct
    // the closure is passed to the "new" function within the Cacher struct to implement it for the Cacher struct
    // this will be set for the calculation field
    // the varaible cached_results will always hold the Struct Cacher with implement closure function as self.calculation
    // need the mut since the attribute self.value is also mutable.
    // cached_results then represents an instance of Cacher
    
    println!("Efficient implementation with closures using structs - Applying Memorization Patterns \n");
    let mut cached_results = Cacher::new(| num : u32| -> u32 {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    }); // the let statement stores the closure itself, not the return value of the closure!
// 
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            //expensive_result
            cached_results.value(intensity)
        );
        println!(
            "Today, do {} situps!",

            cached_results.value(intensity)
        );
        
    }

    else {
        if random_number == 3 {
            println!("Take a break today!")
        } else {
            println!(
                "Today, run for {} minutes",
                cached_results.value(intensity)
            );
        }
    }

}
