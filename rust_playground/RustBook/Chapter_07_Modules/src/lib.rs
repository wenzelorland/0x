/*
A package - cargo new
-> stores crates 
(library crate and binary crate) 
-> crates contain modules, which allow to organise code and authenticate code (public vs. non-public)

A default crate is created when the main.rs sits within the src folder -> binary crate-
In order to create more binary crates, create a bin folder in src and create the corresponding .rs files.

If the lib.rs sits within the src folder, then a library crate is also referenced by default.
cargo new —lib


Workspaces
Allow you to store interrelated packages inside a workspace

Modules can contain other modules, structs, functions, enums etc.

‘Crate’ is module that is created by default for lib.rs, which then represents the root crate.
*/

// specifying paths to external dependencies (crates and housed modules) and bringing them into scope of this code
//use rand::Rng;
//use rand::EntropyRng;

// use nested paths, as both modules come from the same  
use rand::{Rng, CryptoRng, ErrorKind::Transient};

//use std::io;
//use std::io::Write;
// is same as
use std::io::{self, Write};


// Loading the modules that reside in the front_of_house.rs file
// the front_of_house.rs modules can have child module, which can be found in the front_of_house folder, as long as the name correspond correctly
mod front_of_house;

fn serve_order() {}

mod back_of_house{
    fn fix_incorrect_order(){
        // Accessing function from within a module
        cook_order();
        // Accessing functions from the top crate module
        // super allows us to reference the parent crate (which is the root crate in this case)
        super::serve_order();
    }
    fn cook_order(){}

    pub struct Breakfast{
        // even though struct is public, fields of structs are by default private
        // if you want to make it public, then you need to declare it as public
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast:&str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // By default, if am enum is declared public, then all its variants are also public
    pub enum Appetizer {
        Soup,
        Salad,
    }

}

// using the USE keyword in order to bring Paths into scope
// as relative path
use self::front_of_house::hosting;

// making external code have also access to the scope of front_of_house
//pub use crate::front_of_house::hosting;

pub(crate) fn eat_at_restaurant(){
    // Absolute Path
    // starts at root of the module tree 
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative Path
    // can start with front_of_house as it is housed within this module directly
    front_of_house::hosting::add_to_waitlist();    
    
    // with help of USE keyword
    hosting::add_to_waitlist(); 
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal: back_of_house::Breakfast = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    let order1: back_of_house::Appetizer = back_of_house::Appetizer::Salad;
    let order2: back_of_house::Appetizer = back_of_house::Appetizer::Soup;
    let secret_number: i32 = rand::thread_rng().gen_range(1, 101);
}
