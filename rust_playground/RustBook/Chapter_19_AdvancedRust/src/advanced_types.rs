// Advanced Types
// Newtype Pattern
// Recap: Orphan Rule: Once can implement a trait on a given type if either the trait or the type is defined within the crate
// For implementing a trait on a type within a crate which are both outside this crate, one can use the newtype pattern
// In general, the new type pattern is a lightweight way to implement encapsulation

use std::fmt;

struct Wrapper(Vec<String>);
// implementing the Display Trait on a Vector Type, however, both the display trait and the vector type are defined outside the crate
impl fmt::Display for Wrapper {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}



pub fn run (){
    let w = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );
    println!("w = {}", w);

    // TYPE ALIASES
    // Main use-case for type aliases is to reduce repetition
    // defining the type Kilometers as a synonym to i32 type
    type Kilomenters = i32;

    let x: i32 = 5;
    let y: Kilomenters = 5;
    // this works, as Kilometers works as a synonym of i32, as it is defined of this type
    println!("x + y = {}", x+y);

    // this describes a type alias for a cloasure 
    type Thunk = Box<dyn Fn() + Send +'static>;

    let f: Thunk = Box::new(|| println!("hi"));
    fn take_long_type(f: Thunk) {
        // some code   
    }
    
    fn return_long_type(f: Thunk) -> Thunk {
        // some code
        return f
    }
    // Dynamically Sized Types
    // the size of the str type cannot be known at compile time as the Trait Sized is not implemented for str
    // --> need to use &str, i.e. use a borrowed version of str
    // the &str type will store two values -> the address pointing to the location of the string in memory and the length of the string
   
    // the Sized element is important, as str may be dynamic of its size and we need to track the corresponding size to be aware of it
    // at compile time -> we always must put it behing some sort of pointer; either a simple reference pointer or a smart pointer of type Box or so
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";

    // traits are also dynamically sized types, as they have to always be behind some sort of Pointer
    // Rust will also automatically add the Sized trait bound to Generic
    fn generic<T: Sized>(t: T) {
        // some code
    }

    // By default generic functions will only work on types of which size is known at compile time
    // -> one can relax this restriction, by putting a ? before : Sized -> T: ?Sized
    // -> and since we do not know the size of T (as it could be unsized) at compile time, we have to put it behind some pointer in the arguments.
    fn generic_unsized<T: ?Sized>(t: &T) {
        // some code
    }
}

/*
// Never Type -> denoted with ! and means that the function never returns
fn bar () -> ! {
    // some code
}

*/

