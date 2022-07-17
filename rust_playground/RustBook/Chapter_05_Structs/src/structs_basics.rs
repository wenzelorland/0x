// Structs allow you to 
// Structs allow you to group related data together (like object attributres in object oriented programming)
// STRUCTS allow to access these elements by name instead of index (vs. tuple)
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)] // enables to println!("{:?}", Rectangle(30,30)); 
// Debug here is a standard implemented Debug which we tie to Rectangle
struct Rectangle{
    width:u32,
    height:u32

    // Derived Traits 
    // standard functionalities that access specific traits; primitive types implement these traits by default 
    // same as with python __str__ or __repr__ or 
}

// implementation blogs house the functions and methods that are applied on the struct itself
// this will then reference the instance of struct
impl Rectangle {
    // methods
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

}

impl Rectangle {

    // associated functions - are not associated to a specific instance of the struct
    // associated function do not take a reference of self 
    fn square(size:u32) -> Rectangle {
        Rectangle{
            width:size,
            height:size
        }
    }

}


pub fn run(){
    let mut user1: User = User{
        email: String::from("bogdan@mail.com"),
        username: String::from("zero123"),
        active: true,
        sign_in_count: 1
    };

    let user2: User = build_user(String::from("muha@mail.com"), 
                                String::from("muha123")
    );

    let user3: User = User{email: String::from("james@mail.com"), 
                                username:String::from("james123"), 
                                ..user2 // take rest of user2 defined struct
    };
 
    let name: String = user1.username;
    user1.username = String::from("one123");
    println!("Hello world of Rust Structs.");
    

    // tuple structs
    struct Color(i32, i32, i32); // Struct which defines the type Color
    struct Point(i32, i32, i32); // Struct which defines the type Point


    let width1: u32 = 30;
    let height1: u32 = 50;
    println!("The area of the rectangle is {} square pixels.", area((width1, height1)));

    // now with tuple
    let rect: (u32,u32) = (width1, height1);
    println!("The area of the rectangle is {} square pixels - passed with tuple", area(rect));

    // now with user defined struct
    let rect: Rectangle = Rectangle{width:width1, height:height1};
    println!("The area of the rectangle is {} square pixels - passed as struct", area_struct(&rect)); // -> with type of newly defined Rectangle type
    println!("Used rectangle with dimensions: {:#?}", rect);

    println!("The area of the rectangle is {} square pixels - called from a method on the Rectangle instance rect.", rect.area()); // -> with type of newly defined Rectangle type


    let rect_2: Rectangle = Rectangle{width:20, height:20};

    let rect_3: Rectangle = Rectangle{width:45, height:45};

    println!("rect can hold rect_2 {}", rect.can_hold(&rect_2));
    println!("rect can hold rect_3 {}", rect.can_hold(&rect_3));

    // calling associated functions implemented on structs
    let rect_4: Rectangle = Rectangle::square(10);

    println!("rect_4 created from an associatied function on struct Rectangle: {:#?}", rect_4);

}

fn area(dimensions:(u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn area_struct(rectangle:&Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


fn build_user(email:String, username:String) -> User{
    
    User {
        // specifying default values; since function arguments have same name as the User struct we can use the filed init short syntax 
        email,
        username,
        active: true,
        sign_in_count: 1,
    }

    // same as this
    /*     
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
    */
}