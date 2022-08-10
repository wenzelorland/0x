// Combining Rc and RefCell to get multible mutable owners of the data

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>), // Rc SmartPointer for multiple owners and RefCell for mutable referencens
    Nil,
}

use crate::rc_and_refcell::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

pub fn run() {
    // since it is wrapped into Rc and RefCell, value can be mutated and also have multiple owners
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}