// Advanced Traits
// - Associated Types
// --> Placeholders which can be added to the trait and then methods can use that placeholder
// --> when implementing the trait itself, we will define a concrete type for item which we do not know at the definition of the Trait itself until we implement it
// --> Difference vs Generics: -> with associated type one can have only one concrete type per implementation
// --> with Generics one can have multiple concrete types per implementation


pub trait Iterator {
    // associated type -> Item will be the thing we return
    type Item;
    // we do not know the concrete type of Item, but only once the Iterator trait is implemented (e.g. on a string, then Item is a character)
    fn next(&mut self) -> Option<Self::Item>;
}
struct Counter {}

// since we have this implementation of Iterator, we cannot have another implementation, where Item will be of a different type than u32
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

// Same as above but with Generics
pub trait Iterator_Gen<T> {
    fn next(&mut self) -> Option<T>;
}

struct Counter_Ex{}

impl Iterator_Gen<u32> for Counter_Ex {
    fn next(&mut self) -> Option<u32> {
        Some(0)
    }
}

// Default Generic Type Parameter
// -> this allows implementators not needing to specify a concrete type unless it is different from the Default Type

// Reasons for using Default Generic Type Parameters
// 1. Extending a type without breaking existing code 
// -> if you have a type and you want to add another type, then default types come in handy
// 2. Allow customization for specific cases which most users won't need


// Customizing operators are found in the standard library ops
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// overloading Add operator trait on the struct
impl Add for Point {
    // specifying a concrete type for associated type Output in Add trait
    type Output = Point;

    fn add(self, other:Point) -> Point {
        Point { x:self.x + other.x, 
            y: self.x +other.y, 
        }
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    // associated type from Add
    type Output = Millimeters;
    fn add(self, other:Meters) -> Millimeters{
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Calling Methods with the Same Name
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
    fn say_hello();
}

struct Human;

impl Human {

    // method implemented directly on struct
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
    // assocaited functiom
    fn say_hello(){
        println!("Hello, me Human!");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
    fn say_hello() {
        println!("Hello, me Wizard!");
    }
}

// SUPER TRAITS
// -> traits that are dependent on other traits
use std::fmt;

// defining the necessity to implement the Display Trait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // we don't know if self implements the to_string method
        // so we need to make sure that everything we pass implements this method
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *",output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Newtype Pattern
// Orphan Rule:
// --> we can implement a trait on a type when the trait or the type is defined within the crate
// --> Newtype pattern allows to get around this restriction
// --> by creating a tuple struct with one field which is the type we are wrapping
// .> this thin wrapper around the type is local to the crate
// .> so that any trait can be implemented on it

struct Wrapper(Vec<String>);
// downside to this is, that Wrapper is a Newtype and we cannot call methods
// on the vector type directly on a wrapper.
// However, if we want to access all the traits that are implemented on the wrapped type
// we can call the Deref trait such that derefencing Wrapper would return the inner value.
// If we want to access only a subset of method of the inner type and use them on the Newtype directly
// we would ahve to implement them directly.

impl fmt::Display for Wrapper {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // self.0 accesses the Vector as specified for the Wrapper struct
        write!(f, "[{}]", self.0.join(", "))
    }
}

pub fn run(){
    println!("Hi to Advanced Traits");
    Point{x:1, y:0} + Point{x:2,y:2};

    let person = Human;
    // this will call the method that is implemented on the struct directly
    person.fly();
    // this will call the method implemented through the Pilot trait on the Human struct
    Pilot::fly(&person);
    // this will call the method implemented through the Pilot trait on the Human struct
    Wizard::fly(&person);

    // Calling associated function from trait implemented on Human struct, without calling the associated function from the struct
    // which is the default, when calling the function directly from the struct
    // -> will call the function from the struct directly
    Human::say_hello();

    // -> will call the function from the implemented Trait on the struct
    <Human as Wizard>::say_hello();

    let w = Wrapper(
        vec![String::from("hello"), String::from("world")]
    );
    println!("w = {}", w);
}