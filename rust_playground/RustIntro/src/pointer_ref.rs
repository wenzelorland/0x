// Reference Pointers - Point to a resource in memory

pub fn run(){
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values {:?}", (arr1, arr2));

    // Non-Primitives
    // With non-primitives, if you assign another variable to a piece of data, the variable
    // will no longer hold that value. You'll need to use a reference (&) to point to the resource


    let vec1 = vec![1,2,3];
    let vec2 = &vec1; // pointing to a reference of vec1, since otherwise ownership would be transferred and vec1 would be undefined 

    println!("values {:?}", (&vec1, vec2));
    // non-primitive types cannot be pointed directly towards, you need to reference them if you borrow ownership (assign to other variables)
}