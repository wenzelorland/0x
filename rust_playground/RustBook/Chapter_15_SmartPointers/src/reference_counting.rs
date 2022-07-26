// Reference Counting Smart Pointers are used when we are referencing a part on the memory heap and we want to know which parts of the program are still referencing this piece of data
// This way, one is always aware which data on the Heap can be safely deallocated without corrupting the program in case a reference still existed prior to deallocation and would still be needed later on

// This example is only valid from single threaded programs
// The implementation is using a cons list

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

pub fn run() {
    let a = Cons(5, Box::new(Cons(10, Rc::new(Nil))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    // b and c should both point to a
    // we cannot pass a directly, as the ownership would be moved 
    // we also cannot pass &a as we expect a specific type -> Cons
    // however, if we pass Box::new(a) two times, first for b and then c, then this would panic 
    // as the Cons variant owns the data it is passed
    let b = Cons(3, Rc::clone(&a)); // clone here does not perform a deep copy, it just increments the reference count on variable a
    
    println!("count after creating b = {}", Rc::strong_count(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("count after creating c = {}", Rc::strong_count(&a));
    {
        let d = Cons(4, Rc::clone(&a));
        println!("count after creating d = {}", Rc::strong_count(&a));
    }

    println!("count after d goes out of scope = {}", Rc::strong_count(&a));
}
