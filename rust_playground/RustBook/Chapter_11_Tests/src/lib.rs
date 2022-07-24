// To run the tests, type in "cargo test" in the command line
// if one test fails, then the whole test routine fails

// more option on cargo test:
// cargo test --help
// cargo test -- --help // what command line options can we give to the resulting binary output that is passed to cargo test 

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a:i32) -> i32{
    a+2
}

pub fn greeting(name:&str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value:i32) -> Guess {
        if value < 1  {
            panic!("Guess value must be greater than or equal to 1, got {}", value)
        } else if value > 100 {
            panic!("Guess value must be less or equal to 100, got {}", value)
        }
        Guess {value}
    }
}

fn prints_and_returns_10(a:i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// test module with cfg attribute
#[cfg(test)]
mod tests {
    // tests need always need to be marked with the #[test] attribute
    #[test]
    fn it_works() {
        // functions are tests if they have the #[test] attribute
        let result: i32 = 2 + 2;
        assert_eq!(result, 4);
    }

    // since the scope of the tests module is outside the default module scope (which is lib.rs)
    // we need to bring the default module into scope here with a relative path
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger: Rectangle = Rectangle { width: 8, height: 7 };
        let smaller: Rectangle = Rectangle { width: 5, height: 1 };
    
        assert!(larger.can_hold(&smaller));
    }

    #[test] 
    fn smaller_cannot_hold_larger() {
        let larger: Rectangle = Rectangle { width: 8, height: 7 };
        let smaller: Rectangle = Rectangle { width: 5, height: 1 };
        // assert will panic if the expression in the assert returns false
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two(){
        // compares autoamtically two values for equality
        // assert_eq! will panic when the check fails
        assert_eq!(4, add_two(2));
    }
    #[test]
    fn it_adds_not_two(){
        // compares autoamtically two values for equality
        // assert_eq! will panic when the check fails
        assert_ne!(5, add_two(2));
        // both function arguments in assert_ne and assert_eq need to implement the Debug and PartialEq traits
        // for own structures and enumes
    }

    // Custom Tests messages
    #[test]
    fn greeting_contains_name() {
        let result:String = greeting("Carol");
        // Asserts second place argument is the default error mesasge, and all arguments after that are the placeholder values in the message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
        result);
    }

    // Asserting that a function panics
    #[test]
    // panic attribute that defines that the test is passed when underlying function panics
    #[should_panic(expected= "Guess value must be less or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Tests that return a Result type
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2+2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
    // In general, all tests are run in parallel within multiple threads
    // The main thread collects all threads and checks if all have been succsessfully executred

    // setting cargo test -- --test-thread=1 will result in serially running tests on one single thread

    // Printing Test Outputs
    // passing tests print output are not send to stdout 
    // failing tests print output are captured and send to stdout
    #[test]
    fn this_test_will_pass() {
        let value: i32 = prints_and_returns_10(4);
        assert_eq!(10, value);
    }
    #[test]
    fn this_test_will_fail() {
        let value:i32 = prints_and_returns_10(8);
        assert_ne!(5, value);
    }
    // setting cargo test to also print output on passing tests
    // cargo test -- --show-output

    // Running subset of tests by name
    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two(){
        assert_eq!(5, add_two(3));
    }
    // Running only specific test
    // cargo test add by specifying only part of the name or the specific name

    // can also specifiy to only run tests from a specific module cargo test tests

    // Ignoring Tests
    #[test]
    #[ignore] // ignore in normal cases, and only run occasionally
    // the test will be ignored by default
    // invoking the run of the test -> cargo test --ignored 
    // this will run only the ignored tests
    fn expensive_test() -> (){
        // code that takes an hour to run
        ()
    }

    // Test Organization
    // Unittests (test only one module and private interfaces)
    // Integration tests (are external to the library and tests the external interfaces to the library)

    // Unittests (as all the tests above, live with the corresponding production code)
    // typically, one has a module tetst that tests the production functions directly within the same parent module
    // the module tests will ahve the attribute #[cfg(test)] which tells cargo to only run it with cargo test
    // the 'use super::*;' command enables the child module to access all function from the parent module

    // Integration Test
    // they live in the folder /tests folder in the root directly i.e. next to src folder, in rust file tests/integration_test.rs
    // Rust knows to look for the file tests/integration_test.rs for integration tests

}
