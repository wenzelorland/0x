// << MESSAGE PASSING >> 
// Using message to pass data between Threads 
// This is considered a safe way of passing data between thread in a concurrent setup
// This is achieved through CHANNELS

// A channel has two hubs, the transmitter and the receiver
// Transmitter -> Upstream -> where the data is being sent to to be transmitted
// Receiver -> Downstream -> where one listens to to receive the data being transmitted

// A channel is closed when either of the hubs is closed.

// Important SLOGAN:
// Do not communcate by sharing memory - Instead: Share memory by communicating!

// mutli-producer-single-consumer mpsc
// you can have multiple producers of the messages, but only one receiver
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::channel(); // returns the tuple, (Sender<T>, Receiver<T>))

    // spawning a second transnmitter handle for the same channel
    let tx2 = tx.clone()    ;
    // we need to move ownership here, as it is not safe to share the sender -> tx (transmitter) between threads
    // only one thread at a time should have the ownership of the sender
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        let msg = String::from("Hello");
        // we can pass in any type to send
        tx.send(msg).unwrap(); // send method returns a result type -> need unwrap() to also catch errors
        // ownership of msg is passed on to the channel as send takes ownership of the value
        
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }        
    });
    
    // here we cannot use the tx variable, as it has been already moved into the first thread
    thread::spawn(move || {
        let vals = vec![
            String::from("hi_2"),
            String::from("from_2"),
            String::from("the_2"),
            String::from("thread_2"),
        ];
        let msg = String::from("Hello");
        // we can pass in any type to send
        tx2.send(msg).unwrap(); // send method returns a result type -> need unwrap() to also catch errors
        // ownership of msg is passed on to the channel as send takes ownership of the value
        
        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }        
    });
    
    // now we have 2 channels passing the message to two channel which the main thread listens to

    // recv will block main threads execution, while waiting for a message to be transmitted over the channel
    // try_recv will not block the main thread execution and it will return a value if there are value in the channel available
    // otherwise it will just return an error if no values are there at that time
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    for received in rx {
        println!("Got: {}", received);
    }
}