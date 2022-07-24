pub fn run(){
    // simple infinite loop with break condition; result assigned to variable
  
    let mut counter:i32 = 0;
    let result:i32 = loop {
        counter +=1;

        if counter == 10 {
            break counter;
        }
    };

    println!{"The result is {}", result};


    // While Loop
    let mut i:i32 = 0;    
    while i < 100{
        println!{"{}", i};

        i += 1;
    }

    // For in Loop (loop over collection)
    let a: [i32; 5] = [10,20,30,40,50]; // _ means figure length out automatically

    for elem in a.iter() {
        println!{"The value is: {}", elem};

    }
    // Range loop
    for number in 1..4 { // exclusive range from 1 to 4 (not inlcusive), i.e. 1 to 3
        println!{"{}!", number};
    }

}