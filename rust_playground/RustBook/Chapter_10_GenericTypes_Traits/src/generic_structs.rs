// generic Structs

struct Point_UNI<T> {  
    x:T,
    y:T
}

// x method will be available no matter what type has been specified for x and y
impl <T> Point_UNI <T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// Y method will only be available to Point_UNI Structs, where the Type is equal to f64
impl Point_UNI<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

// Multiple generic type Structs
struct Point<T, U> { // <T,U> specifies two different optional generic types
    // this means that x and y can be of the same type, BUT they also have a different type
    x:T, // of generic type T
    y:U, // of generic type U; but they can also be the same or be different
}

// implementation blcoks on structs with multiple generic types 
impl<T,U> Point<T, U> {
    // mixup has its own generic types, as we want to allow mixing different types which we provide as an input
    // the result will be Point<T,W>, as we swap the y value of the self Point with the y value of ther other (provided Point struct)
    fn mixup<V, W>(self, other:Point<V,W>) -> Point<T,W> {
        // return the "newly" created struct
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

/*
Performance Aspects of Generics
-> Generics will not impact performance, as durin compile time, the rust compiler will create the corresponding generic types, 
which are defined during code runtime
*/

pub fn run(){
    let p1: Point<i32, f64> = Point {x:5, y:10.4};
    let p2: Point<&str, char> = Point {x:"Hello", y:'c'};

    // The Option and Result Enums are implemented using generics
    enum Option <T> {
        Some(T),
        None,
    }
    // T as result tyoe, and E as Error type
    enum Result<T,E> {
        Ok(T),
        Err(E),
    }

    let p3: Point<i32, char> = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

}