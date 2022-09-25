// << TRAIT OBJECTS >>

// Trait objects are useful when we do not know the full scope of objects that are going to be used at compile time.
// E.g. a web gui with mutliple elements that can be drawn on the user side
// -> the array of elements can be of any length
// -> the only thing we know is that all these objects implement one common method, which is 'draw'
// In classical inheritance, one would implement a VisualBase class that has the draw method
// and other objects would inherit from this class and would either just take the default draw method
// or implement and overwrite it with their own version of draw

// In rust one defines shared behaviour throug traits

pub trait Draw {
    // draw method
    fn draw(&self);
}

pub struct Screen {
    // Trait Objects Definition
    // !!! At compile time, the Rust compiler will ensure that any object in this Vector implements the Draw Trait!!!
    pub components: Vec<Box<dyn Draw>>, // Vector of any type that implements the Draw trait -> and we put it inside the Box Smart Pointer
    
    // we define trait objects by first specifying a pointer such as a reference or a smart pointer (here Box); 
    // then using the dyn keyword and the trait
    // the dyn keyword stands for dynamic dispatch and is used to highlight that calls to methods on the associated Trait are dynamically dispatched
}

// Why not using Gereneric Types?
// Generic Types are type bound, this means that they can only work with a homogenous type of object at any given time
// If you have a heterogenous set of items, such as here (e.g. Buttons, Boxes, Text), then using Trait Objects is the only way to accomplish this
// as Generic types cannot use a mix of items
// If the list of items is homogenous, then using Generics instead of Trait Objects is preferable, since there is no runtime cost in evaluating the types

// If you want the flexibility in executing code on any type that implements a certain trait, then trait objects are the way to go
// which do have an computation cost

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // draw button
        println!("drawing button... (not implemented)");
    }
}