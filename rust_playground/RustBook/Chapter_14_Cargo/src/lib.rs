//! # Art
//! 
//! A library for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {

    /// The primary colors according to the RYP color model
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model
    pub enum SecondaryColor {
        Orange, 
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1:PrimaryColor, c2:PrimaryColor) -> SecondaryColor {
        // --snip--
        // ANCHOR_END: here
        SecondaryColor::Orange
        // ANCHOR: here
    }
}

/// Adds one to number given.
/// 
/// # Examples
/// 
/// ````
/// let arg = 5;
/// let answer = Chapter_14_Cargo::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ````

pub fn add_one(x: i32) -> i32 {
    x+1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}