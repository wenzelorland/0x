struct ImportantExcerpt<'a>{
    // the struct cannot outlive the reference passed into part
    part: &'a str,
}

// Implementation Block lifetimes
// (defined simlar to Generic Types)
impl<'a> ImportantExcerpt <'a> {
    // since we have a reference to self, then we do not have to include the lifetime annotations explicitly,
    // as all lifetimes will be the same as the lifetime of self
    fn return_part(&self, announcement:&str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn run(){
    let novel: String = String::from("Call me Ishmael. Some years ago...");
    let first_sentence: &str = novel.split('.').next().expect("Could not find Ishmael");
    let i: ImportantExcerpt = ImportantExcerpt { 
        part: first_sentence,
     };
     i.return_part("Ishmael"); 
}