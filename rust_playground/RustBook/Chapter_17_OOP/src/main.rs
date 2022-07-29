// Object Oriented Programming in Rust
// Objects 
// are made out of data and methods which can work on the data
// In Rust structs hold data and with the implementation block we can provide methods on structs an enums
// objects and enums thus provide the same functionality as objects

// Encapsulation
// Implementation details are hidden from the code using that object
// Code outside the object is only limited to operate with the object through its public API
// This enables to change the internals of an object without changing the code that uses that object

// In Rust everything is private by default.
// Fields can be private or publlic, thus encapsulation is also featured in Rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// If we want to the code outside of this object to alter the fields, then we can either make them public 
// or we can provide methods in an implementation block where we only allow for specific changes to be made

impl AveragedCollection {
    pub fn add(&mut self, value:i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // private method only to be called within the object
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

}

// Inheritance 
// -> Ability of an object to inherit form another object's defintion, gaining the data and behaviour of the other object
// -> w/o having to define the behaviour and data by itself
// Rust does NOT have this feature
// However, there are features in Rust which can achieve the same.
// -> using Default Trait Method Implementation (the only caveat right now is that Traits can not define fields, but this is proposed for future verions of Rust)
// -> right now they can only define methods

// Another reason for Using Inheritance is: POLYMORPHISM
// Polymorphism allows to substitute multiple objects for each other at runtime if they share certain characteristics
// In classical inheritance, that characteristics would be the parent class
// Example: You could have a base class called Vehicle which implements the driving method
// -> then you could have sub classes called Truck, Car, Bicycle which inherit from Vehicle
// -> then you can define a function which takes in a vehicle and then at runtime you can pass it a Truck, Car or Bicycle class since they all inherit from Vehicle, i.e. definitions and behaviour are same

// Rust takes a completely different approach
// In Rust one can use Generics to abstract away concrete types and use trait bounds to restrict the characteristics of those types
// In addition to Generics, Rust also provides Trait Objects, which are similar but they use dynamic dispatch (while Generics use static dispatch)


fn main() {
    println!("Hello to Rust's Object Orientation Programming Features.");
}
