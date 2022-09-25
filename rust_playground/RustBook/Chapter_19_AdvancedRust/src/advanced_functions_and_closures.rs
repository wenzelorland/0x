// Advanced Functions
// FUNCTION POINTERS
fn add_one(x: i32) -> i32 {
    x +1
}

// Passing a function as an argument 
// This is especially useful, when one wants to recycle already definined functions instead of redefining closures all over again

// The function Pointer is specified with the lower case ´fn´

// Unlike with closures, fn is a type rather than a trait, which is why we can specify fn directly
fn do_twice(f: fn(i32) -> i32, arg:i32) -> i32 {
    f(arg) + f(arg)
}

// So we don't have to use one of function traits as trait bounds as for closure
// Fn -> captures the variables in its environment inmutably
// FnMut -> closure captues variables in it enviroment mutably
// FnOnce -> closure takes ownership of the variables in it environment, thus consuming the variables

// fn Pointers implement all 3 closure traits

// Since the fn Pointer has every Trait, it is best practice to write functions that allow for functions and closures as arguments
// -> Rewriting above code, so that also closures can be passed to the function
fn do_twice_gen<T>(f: T, arg:i32) -> i32
where T : Fn(i32) -> i32 { // where T is anything the implement the Fn trait bound
    f(arg) + f(arg)
}

// -> this way we allow not only function pointers to be passed, but also closures which implement the Fn trait bound
// -> we could also use other Trait bounds here, like FnMut, FnOnce


// CLOSURES
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x +1)
}

pub fn run(){
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);
    println!("The answer is: {}", do_twice_gen(add_one, 5));


    // Enums initializers are implemented as functions that take in argument and return instance of the tuple enum
    enum Status {
        Value(u32),
        Stop,
    }

    // initializing a vector of Status enums
    // -> this will return a vector of Status Enumes with initialized values ranging from 0 to 20
    let list_of_statuses: Vec<Status> =
        // a Value variant
        (0u32..=20)
        // passing in the Value variant to .map;
        .map(Status::Value)
        // .map is going to treat Value like a function pointer, where the argument will be an unsigned integer u32 
        .collect();
}