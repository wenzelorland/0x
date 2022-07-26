// Interior Mutability is a design pattern in rust that allows 
// you to mutate data, when there are immutable references to that data 
// by using unsafe code inside a data structure to bypass borrowing and mutation rules

// unsafe code is code that is not checked at compile time
// we can enforece borrowing rules at runtime instead of compile time

// RefCell SmartPointer -> RefCell is similar to Box but it does not enforce borrowing rules at compile time but at runtime
// checking borrowing rules at compile time is the default for Rust

// When to use borrowing rules at runtimes:
// Specifically when you are sure that your program is fine with the borrowing rules at runtime
// however, they cannot be checked at compile time, so that the compiler would otherwise panic

// RefCell SmartPointer can only be used in a single threaded program

// RefCell can call methods to get a mutable reference for immutable values

// Remember: There can only be one mutable reference to a mutable data structure at any given time
// Thus, even with RefCell we need to abide by these rules, as they won't be catched at compile time but at runtime which will take a hit at performance

pub trait Messenger {
    fn send(&self, msg:&str); // we expect this method to be iumplemented on the structs directly, when applying the trait on the struct
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T, // reference to a generic type, since we are borrowing T, we need a lifetime annotation
    value: usize, // usize is pointer-sized unsigned integer type -> could also use u32 or u16
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messenger, // making sure the generic has the same trait bound
{
    pub fn new(messenger: &T, max:usize) -> LimitTracker<T> {
        LimitTracker { messenger, value: 0, max}
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You have exceeded 90% of your quota.");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You have used up 75% of your quota!");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    // Mock messenger
    struct MockMessenger {
        sent_message: RefCell<Vec<String>>, // for mutable referencing of immutable fields
    }

    impl MockMessenger{
        // constructor function
        fn new() -> MockMessenger {
            MockMessenger { 
                sent_message: RefCell::new(vec![]),
           
             }
        }
    }

    impl Messenger for MockMessenger {
        // the Messenger trait requires send method requires a immutable reference, this is why we cannot use a mutable reference here
        // however, in order to alter the sent_messages field in the MockMesseger struct, we need a mutable reference
        // in this situation we need to use the interior mutability design pattern by wrapping the sent_messages inside a RefCell SmartPointer
        fn send(&self, message: &str) {
            self.sent_message.borrow_mut().push(String::from(message)); // this will inboke a mutable reference which we can push to 
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();

        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_message.borrow().len(), 1);
    }
}


pub fn run() {
    // let a = 5;
    // let b = &mut a; // cannot borrow a as mutable when a is not defined as mutable

    // at compile time the borrowing rules do not allow a mutable reference to immutable data
    // let mut c = 10;
    // let d = &c;
    // *d = 20; // deref operator to change underlying value of d to 20``
}