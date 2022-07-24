// -- Slices
// Let you reference a contigous sequence of elements within a collection, instead of referencing the entire collection
// Slices do not take ownership 

pub fn run(){
    let s: String = String::from("hello world");
    let s: &str = "hello world"; // string literals are actually String slice to the location in the binary which is saved in the Stack
    // introducing slices
    let hello: &str = &s[0..5]; // same as [..5] from the beginning
    let world: &str = &s[6..11]; // same as [6..] till the end
    // [..] will reference the entire string

    let word: &str = first_word(&s); // this is a references slice


    let a:[i32; 5] = [1,2,3,4,5];
    let slice: &[i32] = &a[..2]; // take slice reference of all elements 0 and 1 

    println!("{:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return where slice where the space if found
        }
    }

    &s[..]
}
