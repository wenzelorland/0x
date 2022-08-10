// Static vs Dynamic Dispatiching

// ----- > Static Dispatching < -----
// Static dispatch means the compiler knows at compile time the concrete functions
// i.e. when at compile time the compiler infers based on the types the functions are returning, what concrete functions to use in the backend
// it will then, for each type create a concrete function, e.g. add_integer, add_float when using an add method on float and integer values
// and execute the respective concrete functions at compile time -> no cost at runtime

// ----- > Dynamic Dispatching < -----
// In contrast to this, when at compile time it is impossible to know which types are passed in, as they may be dynamically entered / executed
// then the inference of types and the respective concrete function creation occuurs at runtime

// Object Safety for Trait Objects
// A trait is object safe when all the methods implemented on the trait have these 2 properties:
// 1) Return type is not self, i.e. soes not return an instance of itself 
// 2) There are no generic parameters

// concrete type cannot be inferred by the compiler otherwise

pub fn run() {
    println!("Welcome to OOP in Rust with Trait Objects.");
}