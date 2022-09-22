pub struct Counter {
    count: u32, // field is private, since we only want the field to be accessed by the implementation block
}

impl Counter {
    // construction function
    // default implementation
    pub fn new() -> Counter {
        Counter {count : 0}
    }
}

// The standard library implements further methods on iterators by default, like filter, map, zip
impl Iterator for Counter {
    // Associated type -> Iterator will return Items of type u32
    type Item = u32;

    // iterator default method
    // implementing the next method automatically grants access to all methods that are implemented for iterators in the standard Rust library^
    fn next(&mut self) -> Option <Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }

    }
}
pub fn run(){
    println!("Own Iterators");
}

