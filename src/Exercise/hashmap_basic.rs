use std::collections::HashMap;

fn main() {
    // Create an empty HashMap
    let mut map: HashMap<String, i32> = HashMap::new();

    // Add key-value pairs to the HashMap
    map.insert(String::from("value1"), 10);
    map.insert(String::from("value2"), 20);

    // Retrieve and print the value associated with a specific key
    let key_to_retrieve = String::from("value1");
    match map.get(&key_to_retrieve) {
        Some(&value) => println!("Value associated with key {} is: {}", key_to_retrieve, value),
        None => println!("Key {} not found in the HashMap", key_to_retrieve),
    }

    // Check if a key exists in the HashMap
    let key_to_check = String::from("value3");
    if map.contains_key(&key_to_check) {
        println!("Key {} exists in the HashMap", key_to_check);
    } else {
        println!("Key {} does not exist in the HashMap", key_to_check);
    }

    println!("{:?}", map);
}
