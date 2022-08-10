use Chapter_14_Cargo::PrimaryColor;
use Chapter_14_Cargo::SecondaryColor; // only possible since we made those availbe on the top level of lib.rs
// use Chapter_14_Cargo::kinds::PrimaryColor; // would need to use this otherwise
use Chapter_14_Cargo::mix;
use rand;
fn main() {
    println!("Hello to Rust Cargo Handling and Workspaces - Please read the Readme.txt file for further information");
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    let mix = mix(red, yellow);
}
