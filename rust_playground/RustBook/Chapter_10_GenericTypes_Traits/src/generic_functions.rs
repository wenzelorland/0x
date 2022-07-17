pub fn run(){
    println!("Hello world of Rust Generics");

    // Extracting Functions
    let number_list: Vec<i32> = vec![34,50,25,100,65];

    let mut largest: i32 = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    let number_list: Vec<i32> = vec![34,50,25,100,65, 700];
    let largest:i32 = largest_number(number_list);

    println!("The largest number is {}", largest);

    // Working with Generic Types and Functions
    let char_list: Vec<char> = vec!['y', 'm', 'z', 't'];

    let largest:char = get_largest_gen(char_list);
    
    println!("The largest element is {}", largest);
    let number_list: Vec<i32> = vec![34,50,25,100,65, 700, 999];
    let largest:i32 = get_largest_gen(number_list);
    println!("The largest element is {}", largest);
}
// Defining a function that can be called within main to perform same action as inline code and DRY
fn largest_number(number_list: Vec<i32>) -> i32 {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
// Same function as above, but now with generic types
// Generic types are defined in the <> brakcets <T> stands for <Type> 
fn get_largest_gen<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest: T = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}