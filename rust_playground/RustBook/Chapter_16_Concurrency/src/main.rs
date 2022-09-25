// One of the main goals of Rust is to handle safe concurrency
// Parallel Programming: Different parts of a program execute at the same time
// Concurrent Programming: Different parts of a program execute independently 

// Here concurrent means either parallel or concurrent -> as the concept are overlapping in some aspects

// Because of Rust's Ownership model, most concurrency bugs can already be caught at compile time

// In most OS, an executed program's code is run in a process and the OS manages running multiple processes at once
// In the executed program there may be parts that are running independently, which are called threads

// Because threads run simultaneously and independently, one has to take care of Race Conditions, where multiple threads can alter a specific data
// in an inconsistent order.
// Deadlocks can also be an issue where two threads are both waiting for the other thread's resource and are stuck waiting as neither passes the resource
// Lastly, because execution order is non-deterministic, bugs can appear that would otherwise not appear in single threaded design and which are hard to reproduce.

// Types of Threads:
// - 1 to 1 threads -> OS threads, native threads, system threads
//    -> when you create a thread in the program, it maps to a thread in the OS, so there is a 1 to 1 mapping
// - green threads -> user threads, program threads
//    -> no 1 to 1 mapping; there may be 20 program threads that map to 5 OS threads or vice versa -> M to N model
// The most important trade-off of these is runtime support. Because green threads require more runtime, that is why Rust only includes 1 to 1 threads in its standard library
// Green threads mean that the code of the binary (runtime) is increased and thus the binary gets larger in size vs 1 to 1 threads.

use std::{thread, time::Duration};

// IMPORTANT
// When the main thread ends, any execution of a spawned thread will be stopped
// In order to avoid this, we need to work with Join in order to wait for all spawned threads until they are finished to continue / leave the execution

mod message_passing;
mod state_sharing;

fn main() {
    println!("Hello to Rust Concurrency!");
    // running a closure in the thread without an input variable to the closure
    
    // SPAWNED THREAD    
    // returns a JoinHandle
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // since threads in Rust are non blocking (no GIL!), the program will not halt here, but continue with other code execution below
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if you move handle.join().unwrap() here, the main thread will wait until the thread behind the handle has finished executing and only then will commense
    // -> one way to introduce blocking code when working with concurrency 
    
    // small details where the JoinHandle is placed decide whether your program runs concurrently or not

    // MAIN THREAD: by default every program has a main thread 
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
    // calling join on the JoinHandle will block the thread currently running, until the thread associated with the handle terminates
    // in this case the Main thread is blocked so long until the spawned thread is done executing
    // the main therad 
    
    // need to call the unwrap since .join() returns a Return type
    // USING MOVE CLOSURES WITH THREADS
    let v = vec![1,2,3];

    // Rust figures out by itself what it needs for v -> i.e. a reference to v inside the closure
    // Rust will try to infer the reference of the value by itself.
    // In order to be on the safe side, we want to make sure that we pass the ownership of v to the closure, so that no other thread may alter v
    // while this thread is still busy with v
    // this is why we need the move keywork in front of the closure

    // we tell Rust that the closure takes the ownership of any provided variable
    // this way, no other thread can have ownership of v as long as this thread is busy with v
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();

    message_passing::run();
    state_sharing::run();
}
