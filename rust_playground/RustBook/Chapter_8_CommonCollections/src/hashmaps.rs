use std::collections::HashMap;

pub fn run(){
    let blue : String = String::from("Blue");
    let yellow: String = String::from("Yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    // moving the values of blue and yellow (OWNERSHIP) into the hashmap as keys 
    // one could also use references, so to not move ownership of blue and yellow, however, this will require lifetime mamangement which will be introduced later
    scores.insert(blue, 10);
    scores.insert(yellow, 50);

    let team_name : String = String::from("Blue");
    let score = scores.get(&team_name);

    // Looping over the hashmap
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating Hashmaps
    scores.insert(String::from("Blue"), 20);
    scores.insert(String::from("Yellow"), 150);

    // More careful updating: If exists, do not insert
    // .entry returns the enum, i.e. the value at that key if the key exists, otherwise default value
    scores.entry(String::from("Blue")).or_insert(40); // this will insert 40 as the value for key Blue, but only if Blue is not already in the HashMap
    // or_insert returns a mutable reference to the value in the HashMap, regardless if it has been inserted or already found


    // Updating HashMap based on old values od Hashmapo

    let text: &str = "hello world wonderful world"; // binary string reference
    
    let mut map = HashMap::new();

    // ["helloe", "world". "wonderful", "world"] -> collection of words which will be iterated in the following
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        // since or_insert() returns a reference of the value, we can dereference it to alter it with the *count += 1; statement
        *count += 1;
    }

    println!("{:?}", map);

}