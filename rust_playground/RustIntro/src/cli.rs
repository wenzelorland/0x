
pub fn run(){
    // this collects the statements after the cargo run command; first entry of the vector will be the path, all next will be the commands provided through the cli 
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    let name = "Brad";
    let status : &str = "100%";

    println!("Args: {:?}", args);
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    }
}