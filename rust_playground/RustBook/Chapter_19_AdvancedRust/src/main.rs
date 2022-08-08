// ToDo: Writing Unsafe Rust -> check chapter 19 book / youtube

mod advanced_traits;
mod advanced_types;
mod advanced_functions_and_closures;
mod declarative_macros;
fn main() {
    println!("Hello to Advanced Rust!");
    advanced_traits::run();
    advanced_types::run();
    advanced_functions_and_closures::run();
    declarative_macros::run();
}
