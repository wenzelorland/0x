// Functions - Used to store block of code for re-use

pub fn run(){
    greeting("Hello", "Jane");

    // Bind function values to variables
    let get_sum = add(5, 5);
    println!("Sume: {}", get_sum);

    // Closures -> can also access vairables outside of scope blocks ->fn something () {this represents a scope block}
    let n3: i32 = 10;
    let add_nums = |n1:i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3,3));
}


fn greeting(greet:&str, name: &str){
    println!("{} {}, nice to meet you!", greet, name)

}

fn add(n1:i32, n2:i32) -> i32 {
    n1 + n2 // same as return n1 + n2; the missing semicolon indicates to rust that this should be the return value

}

