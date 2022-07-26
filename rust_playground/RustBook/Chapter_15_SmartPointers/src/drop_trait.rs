// The drop trait can be implemented on any type and allows oyu to customise when a value leaves the scope
// The drop trait takes care of the deallocation of memory on the Heap when a value leaves the scope

struct CustomerSmartPointer {
    data: String,
}

impl Drop for CustomerSmartPointer {
    
    // this method is required when implementing the Drop Trait
    // takes a mutable reference of self
    fn drop(&mut self) {
        println!("Dropping CustomerSmartPointer with data `{}`", self.data);
    }

}

pub fn run (){
    let c = CustomerSmartPointer {
        data: String::from("my stuff"), 
    };

    let d = CustomerSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomerSmartPointers created.");

    // here the drop method will be called on the created variables in order to remove them from the Heap 
    // as they are leaving the scope in reverse order of creation, i.e. LIFO 

    // customizing the order of deallocation and manually cleaning up memory
    drop(c); // this function is different from the drop method and it is contained within the standard rust library
    // now c will be deallocated first, then d
}