// State Pattern in Rust
// In the state pattern there some value which has internal state and that internal state is represented by state objects
// Each state obejct is responsible for its own behaviour and deciding when to transition into antoher state
// The value that holds the state objects knows nothing about the behavior of different states or when to transition to a different state

// Benefits of using State Pattern
// -> When requirements change, we just need to change the code inside one of the state obejcts or change code in an exisitng state object

use super::blog::Post;

pub fn run() {
    println!("Welcome to State Pattern Design");
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    println!("{}", post.content());
    // To be fixed -> does not correctly transition to Published state
    //assert_eq!("I ate a salad for lunch today", post.content());
}