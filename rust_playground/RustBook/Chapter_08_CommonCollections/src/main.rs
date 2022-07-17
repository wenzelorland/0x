/*
Common Collection allow you to store multiple values
They are allocated on the Heap, which means that they are variable in size
-> Vectors, Strings and Hashmaps
*/

mod string_collection;
mod hashmaps;

fn main() {
    // Vectors
    let _a: [i32; 3] = [1,2,3];
    let mut v: Vec<i32> = Vec::new(); // vectors can hold any data types

    v.push(1);
    v.push(2);
    v.push(3);
    
    {
    // Since vectors are allocated on the Heap, they are disallocated, once the scope has been left
    let _v2: Vec<i32> = vec![1,2,3];
    }

    // Accessing elements of a vector
    let mut v3: Vec<i32> = vec![1,2,3,4,5];
    let mut third: &i32 = &mut v3[2]; // since at compile time, we do not know what size a vector will have, you can enter any index here and will only throw an error at runtime

    // we need mutable references of vector, when we change the size of the vector along the runtime
    v3.push(6);
    // need to move the indexing of the element after the altering of the vector, since we are working with references 
    // and since the vector is allocated on the Heap, when changed the reference will also change -> need to reassign

    let mut third: &i32 = &mut v3[2]; // since at compile time, we do not know what size a vector will have, you can enter any index here and will only throw an error at runtime
    println!("The third element is {}", &mut third);
    // safer handling of element indexation of vectors
    //let fourth: Option<&i32> = v3.get(2);
    // this is same as
    match v.get(20) {
        Some(mut third) => println!("The third element is {}", &mut third),
        None => println!("There is no third element")
    }

    println!("The third element is {}", &mut third);

    let mut v: Vec<i32> = vec![1,2,3,4,5];
    println!("{:#?}", v);

    for i in &mut v{
        *i += 50; // using the dereference operator * in order to change the underlying value in the vector (not only the reference)
    }

    println!("{:#?}", v);

    string_collection::run();
    hashmaps::run();
}

