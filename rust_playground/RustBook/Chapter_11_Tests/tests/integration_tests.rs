use Chapter_11_Tests; // importing all contents of the root crate

// Executing the integration tests 
// cargo test --test integration_test

// Note:
// If you place a .rs file within the /tests folder, they will be compiled as crates
// If you then run cargo test, then this file will be treated as a tests case automatically
// In order to avoid this, create a folder within tests/ and save your additional .rs files here
// These files won't be compiled to crates
// These files are accessbile as modules

// looks for common.rs or a file named common with a mod.rs file inside and 
// will take make all the content avaialble to the scope of this module
mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, Chapter_11_Tests::add_two(2));
}

// Since we have a lib.rs file, this represents a library crate
// if we have a main.rs file, we have a binary crate
// Binary crates cannot be tested with integration tests
// This is why it is common to have thin wrappers of binary crates around library crates