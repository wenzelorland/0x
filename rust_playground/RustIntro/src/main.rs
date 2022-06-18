
mod print;
mod vars;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointer_ref;
mod structs;
mod enums;
mod cli;

fn main() {
    let test: i32 = 132;
    let test_2: f32 = 132.0; 
     

    print::run();
    vars::run();   
    types::run();
    strings::run();
    tuples::run();
    arrays::run(); 
    vectors::run();
    conditionals::run();
    loops::run();
    functions::run();
    pointer_ref::run();
    structs::run();
    enums::run();
    cli::run();


    println!("{} {}", test, test_2);
    println!("{}", try_me(152));
}

pub fn try_me(n:i32) -> i32 {
    let mut f: i32 = 0;
    
    for k in n..n+100 {
        f += k;
        if f % 50 == 0  {
            return f;
        }
    }
    return f;
}
