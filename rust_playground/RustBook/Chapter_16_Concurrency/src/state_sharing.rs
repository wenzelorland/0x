// Transferring data using shared state
// -> one way data flow
// -> one thread passes data to another thread of which the receiving thread takes ownership of

// <<MUTEX>> Mutual Exclusion 
// At any given time, only one thread is able to access that data at any given time
// Mutex uses a locking system -> threads need to acquire the corresponding lock of a piece of data to be able to access it
// A Lock is a specific data structure that keeps tracks of which thread has access to a particular piece of data.
// Once the thread is done, it can unlock the data and other thread can acquire it.

// Rust Ownership and Borrowing rules guarantuee that one cannot get the locking wrong

// The same way an RefCell smart pointer allows you to mutate a value inside an Rc smart pointer -> single threading
// -> RefCell comes with the risk of creating ciruclar references

// a Mutex Smart Pointer allows you to alter / mutate a value / piece of data inside the Arc smart pointer -> multi threading
// -> Mutex comes with the risk of creating deadlocks

// The standard library already offers all necessary tools for concurrency in rust and implements the Send / Sync traits
// There are multiple libraries which focus on the concurrency aspect of Rust and offer significant value and options in thise domain.

use std::thread;
use std::sync::{Arc, Mutex};

pub fn run() {
    // the value 5 has now a lock assigned
    let m = Mutex::new(5); // call assocaited function new to create a mutex


    {
        // acquiring the lock
        // this will block the current thread until the lock is acquired
        // unrwap makes sure that if a thread already has the lock and it panics then lock will return an error and we will capture it accordingly
        // by calling unwrap meaning that if lock fails, then panic
        
        // if the thread acquire the lock, then it will receive a SmartPointer the MutexGuard, which deref trait points to the underlying data behind the Mutex, 
        // in this case the integer passed into the Mutex
        let mut num = m.lock().unwrap();
        *num = 6;

        // MutexGuard implements the drop trait such that if it leaves the scope, then it releases the lock so that the lock goes back to the data
        // this will be done automatically
    }
    println!("m = {:?}", m);

    // Sharing a value between threads
    // Mutex uses interior mutability, this is why we can alter counter even when it is not declared as mutable
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        
        
        // moving counter into the closure thread
        // since counter will be moved multiple times in the iteration, we need to allow counter to have multiple owners
        // this counter will be shadowing the original counter variable as a Rc Smart Pointer 
        let counter = Arc::clone(&counter);
        // Rc is not thread safe, this means we need an alternative to Rc -> Atomic Reference Counting Smart Pointer Arc
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    // waiting for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter Arc Mutex = {:?}, Counter value = {}", counter, *counter.lock().unwrap());


}