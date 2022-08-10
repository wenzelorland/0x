use std::cell::{RefCell, Ref};
use std::rc::{Rc, Weak};

use crate::reference_cycles::List::{Cons, Nil};
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), // Rc used for multiple owners, and RefCell for mutability
    Nil,
}

impl List {
    // convenience method
    fn tail(&self) -> Option <&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,

    // Weak is version of Rc that holds a non-owning reference to the managed allocation
    // adding reference from child to its parent
    parent: RefCell<Weak<Node>>, // for the ability of parents owning the children
    // children should be dropped, when parent is dropped out of scope
    children: RefCell<Vec<Rc<Node>>>,
}


pub fn run() {
    println!("Hello to Rust Reference Cycles.");
    // a is a reference pointing to a Cons List saved on the Heap
    // this creates a smart pointer to a cons list on the Heap with value 5 and reference Nil
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));

    println!("a next item = {:?}", a.tail()); // tail method returns Some type or None

    // b is a reference pointing to a Cons List saved on the Heap 
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    // this creates a cons list with value 10 and a reference to list a

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));

    println!("b next item = {:?}", b.tail()); // this creates a reference to list b within a 

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // here stack memory of a and b will be cleaned up
    // however, since we have two lists on the heap that are still referencing each other, those will not be cleaned up from the Heap!
    // -> there will be no more stack items pointing at them and thus they will reside in memory

    // If you uncomment the next line, you will encounter a an overflow in the stack
    // println!("a next item = {:?}", a.tail()); 

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // creating a Weak point w/o allocating memmory
        children: RefCell::new(vec![]),
    });

    // we want a mutable reference to the value stored inside the RefCell smart pointer
    // we are using the deref operator to actually change the value the pointer is pointing to
    // which is our branch node and we change it to point to the branch node
    // since the branch node is a Rc smart pointer pointer and the parent field expects a 
    // Weak smart pointer, we have to call the Rc::downgrade function to transform the Rc to Weak smart pointer and pass in the Rc smart pointer
    
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // whenever we want to see or mutate the value inside the Weak smart pointer we ahve to upgrade it to the Rc smart pointer
    // since the Weak smart pointer has no idea if the has been augmented or not

    let branch = Rc::new(Node{
        value:5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]), // storing 
    });

    // we want a mutable reference to the value stored inside the RefCell smart pointer
    // we are using the deref operator to actually change the value the pointer is pointing to
    // which is our branch node and we change it to point to the branch node
    // since the branch node is a Rc smart pointer pointer and the parent field expects a 
    // Weak smart pointer, we have to call the Rc::downgrade function to transform the Rc to Weak smart pointer and pass in the Rc smart pointer
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); 

    // Weak and Strong Count Rc::strong_count(&ref), Rc::weak_count(&ref)
    // strong_counts are number references that have ownership of the data
    // weak_counts are number of references that have no ownership of the data
 
    let leaf_2 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // creating a Weak point w/o allocating memmory
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf_2 strong = {}, weak = {}",
        Rc::strong_count(&leaf_2),
        Rc::weak_count(&leaf_2),
    );
 
    {
        
        let branch_2 = Rc::new(Node{
            value:5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf_2)]), // storing 
        });
        // parent field to be reference to branch_2        
        *leaf_2.parent.borrow_mut() = Rc::downgrade(&branch_2);
        
        // branch_2 will have one strong and one weak count, as leaf_2 as its child has ownership reference to branch_2
        // and leaf_2 has a weak reference to branch_2
        println!(
            "branch_2 strong = {}, weak = {}",
            Rc::strong_count(&branch_2),
            Rc::weak_count(&branch_2),
        );
        
        // strong count should be 2, since leaf_2 is a child of branch_2
        // weak count should be 0
        println!(
            "leaf_2 strong = {}, weak = {}",
            Rc::strong_count(&leaf_2),
            Rc::weak_count(&leaf_2),
        );
        
   
    // since branch_2 is dropped here, it's strong_count drops to zero
    // weak count remains of 1 since leaf_2 has a reference to branch_2, which remains on the Heap if branch_2 leaves the scope
    }
    
    println!(
            "leaf_2 parent = {:?}", leaf_2.parent.borrow().upgrade()
    );
    
    println!(
            "leaf_2 strong = {}, weak = {}",
            Rc::strong_count(&leaf_2),
            Rc::weak_count(&leaf_2),
        );
}