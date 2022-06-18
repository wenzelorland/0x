pub fn run(){
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age = true;

    // If - else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drin?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave.");
    } else {
        println!("Bartender: I'll need to see your ID.");
    }

    // Short.hand If
    let is_of_age: bool = if age >= 21 { true } else { false };
    println!("Is of Age: {}", is_of_age);

}