// Structs - Used to create custom data types
// similar to classes in python (though without inheritance)


pub fn run(){
    let mut c = Color {
        red:255,
        green:0,
        blue:0
    }; 
    
    c.green = 100;
    println!{"Color: {} {} {}", c.red,c.green,c.blue}
    
    // changing tuple struct attributes 
    let mut c_t = ColorT(255,0,0);
    c_t.1 = 100;
    println!{"Color: {} {} {}", c_t.0,c_t.1,c_t.2}


    // using the struct Person and its functions
    let mut p = Person::new("John", "Doe");
    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person {}", p.full_name());

    // changing attributes of the structs on the fly with repsective method
    let mut md = Person::new("Marry", "Doe");

    println!("Person {}", md.full_name());
    md.set_last_name("Williams");
    println!("Person {}", md.full_name());
    println!("Person Tuple {:?}", md.to_tuple());
}

// Traditional Struct
struct Color {
        red: u8,
        green:u8,
        blue:u8
    }

// Tuple Struct
struct ColorT(u8,u8,u8);

struct Person {
    first_name: String,
    last_name:String
}

// implement method for Person struct
impl Person {
    // construct Person
    fn new(first:&str, last: &str) -> Person{
        Person{
            first_name: first.to_string(), // this is since we used the :String type class before
            last_name: last.to_string()
        }
    }
    // Get full name method
    fn full_name(&self) -> String{
     // referencing the struct itself 
        return format!("{} {}", self.first_name, self.last_name); // same as without return and ;
    }
    // Get last name method
    fn set_last_name(&mut self, last:&str){ // &mut self since we are changing attributes here
        self.last_name = last.to_string();
    }
    // Name to tuple
    fn to_tuple(self) -> (String, String){
        (self.first_name, self.last_name)
    }
}