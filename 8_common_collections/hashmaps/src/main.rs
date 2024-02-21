fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new(); //creating hashmap

    scores.insert(String::from("Blue"), 10); //inserting in hashmap
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue"); //getting value
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores { //iteration in values
        println!("{key}: {value}");
    }

    //For types that implement the Copy trait, like i32, the values are copied into the hash map. 
    //For owned values like String, the values will be moved and the hash map will be the owner of those values

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    //If we insert a key and a value into a hash map and then insert that same key with a different value, 
    //the value associated with that key will be replaced.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); //10 is overwritten

    //Hash maps have a special API for this called entry that takes the key you want to check as a parameter.
    // The return value of the entry method is an enum called Entry that represents a value that might or might not exist.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); //will not overwrite

    //example of updating hm values
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
