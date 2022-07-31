pub struct Post {
    // Trait Object definition
    // Attributes:
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // constructor function
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text:&str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        // state is going to be an Optional that owns the State Objects
        // instead we want the reference to the State Object
        // we can then call the unwrap method to receive the result in the Some() result
        // then we call the .content() method and we pass it the instance of the Post struct
        self.state.as_ref().unwrap().content(self)
    }

    // takes mutable reference to itself, since we want to modify the Post instance
    pub fn request_review(&mut self) {
        // the .take() method on the Optional will take the value of the Optional and leaving None in its place
        // the value is moved into the current_state (i.e. state before changing the state)
        // as take leaves None, when the value of the Option is transferred, self.state is equal to None
        if let Some(current_state) = self.state.take() {
            // here we change the value from None to current_state.request_review() result
            self.state = Some(current_state.request_review());
        }
    }
    
    pub fn approve(&mut self) {
        // the .take() method on the Optional will take the value of the Optional and leaving None in its place
        // the value is moved into the current_state (i.e. state before changing the state)
        // as take leaves None, when the value of the Option is transferred, self.state is equal to None
        if let Some(state) = self.state.take() {
            // here we change the value from None to current_state.request_review() result
            self.state = Some(state.request_review());

        }
    }
 }

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    
    // we need lifetime annotations here since the function takes two references and return a reference
    // --> need to tell the compiler which references are related
    fn content<'a>(&self, post:&'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    // since this takes ownership of self, we invalidate the old state as we overwrite with the new state
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // if we call request_view on a Draft state, we want to return a PendingReview state
        Box::new(PendingReview{}) 
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}


// new state as Pending Review
struct PendingReview{}

// implement Trait State for new State object PendingReview
impl State for PendingReview {
    // Since the state = PendingReview, we just return the current instance, as now request has already been submitted
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

// Published State
struct Published {}

impl State for Published {
    
    // when the state = Published, then we cannot request a review since the Post has already been published, thus we return the current instance
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    // as it is already approved, approve method returns only the instance itself
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    } 
    // since this function takes two references and also returns a reference, we need lifetime annotations
    // i.e. we need to tell the compiler the relationship of the variables
    fn content<'a>(&self, post:&'a Post) -> &'a str {
        println!("{}", &post.content);
        println!("Hello Published");
        &post.content
    }
}