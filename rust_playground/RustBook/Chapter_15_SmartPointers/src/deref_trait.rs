use std::ops::Deref;
struct MyBox<T>(T); // Generic Smart Poiunter

impl<T> MyBox<T> {    
    // here x is not stored on the Heap, as opposite to the Box Smart Pointer of the Standard Library
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // associated type

    // when deref operator is used we want to return the reference to the item stored in our tuple struct
    // a reference is returned since otherwise the ownership of the value would be moved outside of the Smart Pointer
    // this method is called whenever the deref operator is used -> *variable
    // once the reference is returned, the compiler automatically knows how to handle it
    fn deref(&self) -> &Self::Target { // same as &Self::T
        &self.0
    }
}

pub fn run(){
    let x = 5;
    let y = MyBox::new(x); // Box holds the reference of x, as it is a Smart Pointer

    assert_eq!(5, x);
    // here we want the actual value behind the reference, that is why we want to dereference it
    // the deref operator will follow the memory address that is stored in y and will access the corresponding value
    assert_eq!(5, *y); // dereference operator -> *variable *y actually means *(y.deref())

    // Deref Coercions are automatically performed
    // This means although the function expects a specific type, rust automatically figures out the derefenencing in order to access the correct passed value reference
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // otherwise we would need to write this when automatic Deref Coercions would not be implemented
    hello(&(*m)[..]);


    // Deref Coercion and Mutability
    // Rust cannot perform deref coercion when going from immutable reference to a mutable reference because of the borrowing rules

}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}