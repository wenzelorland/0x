// Structs and Enums are building blocks for new type definitions in Rust
// are most similar to Enums impelemented in functional programming languages
// Enums allow to enumerate a list of variants 

enum IpAddrKind {
    V4(String), // enum variants can store different types of data - storing data inside the enum variant
    V3,
    V5(u8,u8,u8,u8)
}

enum Message{
    Quit,
    Move {x:i32,y:i32}, // stores anonymous struct
    Write(String),
    ChangeColor(i32,i32,i32)
}
impl Message {
    fn some_function(){
        println!("Associuated function within Message enum.");        
    }
}

// Rust does not have Null values!
// it uses the Option enum to handle null values
// Option enum is included in the scope by default
/* 
enum Option<T> {
    Some(T), // Some stores some value - is of generic type
    None,
}
*/

enum Coin {
    Penny, 
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin:Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i+1),   
        _ => None, // if it doesn't match with anythen else then do this
    }
}


struct IpAddr {
    kind: IpAddrKind,
    address: String

}

pub fn run(){
    let three: IpAddrKind = IpAddrKind::V3;

    let localhost: IpAddrKind = IpAddrKind::V4(String::from("127.0.0.01"));
    let localhost_uint: IpAddrKind = IpAddrKind::V5(127,0,0,1);

    let x: i8 = 5;
    let y: Option<i8> = None; // define default value as 5

    let sum: i8 = x + y.unwrap_or(0); // define default value of y to 0 if y is None 
    let coin_v: Coin = Coin::Nickel;
    let coin_value: u8 = value_in_cents(coin_v);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    // If-Let syntax instead of match expressionb
    let some_value = Some(3);
    if let Some(3) = some_value{
        println!("three");
    }
    // same as
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }

}
