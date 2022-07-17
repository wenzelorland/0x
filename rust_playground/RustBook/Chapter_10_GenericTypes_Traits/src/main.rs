mod generic_functions;
mod generic_structs;
mod generic_traits;
mod lifetimes;
mod lifetime_structs;

use std::fmt::Display;
// Takes references of string and ownership of ann, which is of Generic type,
// where we limit the Genric Type to Display, i.e. a Trait Type bound 

// since we have two different references, we need to specify the lifetime annotations
fn longest_with_an_argument<'a, T> (x: &'a str,  y: &'a str, ann: T, 
                    ) -> &'a str where T: Display, {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("Hello, world!");
    generic_functions::run();
    generic_structs::run();
    generic_traits::run();
    lifetimes::run();
    lifetime_structs::run();
}
