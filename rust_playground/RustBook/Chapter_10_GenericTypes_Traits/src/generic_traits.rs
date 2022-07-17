use std::fmt::Display;


pub struct NewsArticle {
    pub author : String,
    pub headline : String,
    pub content : String,
}

// Implementing the Trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} {}", self.headline, self.content)
    }
    
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

}

// Implementing the Trait
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
    fn summarize_gen(&self) -> String {
        format!("{} {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,

}

// Traits as parameters, where the function takes in a reference to something (item) that implement the trait Summary
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// <- this is shorthand for Trait bound, where we can specfiy the types inputs 
//pub fn notify<T: Summary>(item1:&T, item2: &T) {
//    println!("Breaking news! {}", item1.summarize());
//}

// Specifying mutiple implemented traits
pub fn notify_2(item1: &(impl Summary+ Display), item2: &impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}


// Defining a Trait
// Traits are used to define a shared behaviour between different types (Structs)

pub trait Summary {
    fn summarize(&self) -> String; // we do not define the function here; we just say that whatever type (Struct) is associated with this trait, also has this summarize method
    // here we expect that every type needs to summarize its own body for summarize
    
    
    fn summarize_author(&self) -> String;
    
    // Defining a default implementation of summarize
    fn summarize_gen(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }// we can call methods within traits directly, although they have not been defined/specified yet
}

// Returning Types Implement Traits
fn return_summarizable() -> impl Summary {
    Tweet{
        username: String::from("horse_eboks"),
        content: String::from(
            "of course, as you probably already know, people.",
        ),
        reply: false,
        retweet: false,
    }
}

// Conditionally Implement Methods
struct Pair<T> {
    x:T,
    y:T,
}

// In this implementation every Pair struct type wll have the new function
impl<T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self {x,y}
    }
}

// This cmp_display will only be available wthin Pair structs, where 
// type of x and y implement the Trait Display and Trait PartialOrd
impl<T:Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}



pub fn run() {

    println!("Hello, world of Generic Rust Traits");
    let tweet:Tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Hello World!"),
        reply: false,
        retweet: false
    };

    let article: NewsArticle = NewsArticle{
        author: String::from("John Doe"),
        headline: String::from("The Sky is Falling!"),
        content: String::from("The sky is not actually falling.")
    };

    println!("With specific method defined: Tweet summary: {}", tweet.summarize());
    println!("With specifc method defined: Article summary: {}", article.summarize());
    println!("With default methods: Tweet summary: {}", tweet.summarize_gen());
    println!("With default methods: Article summary: {}", article.summarize_gen());

    notify(&article, &tweet);
    // We can call this generic method on return_summarizable, since we know that the function returns something that has the Summary Trait
    println!("{}", return_summarizable().summarize());
}
