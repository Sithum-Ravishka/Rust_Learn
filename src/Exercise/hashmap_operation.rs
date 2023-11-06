// Exercise 4: HashMap Operations

//     Create two hashmaps with similar keys but different values.
//     Merge the two hashmaps into a single hashmap, resolving conflicts by choosing the value from the first hashmap.
//     Find the key with the highest value in the merged hashmap.
//     Remove a specific key-value pair from the merged hashmap.

use std::collections::HashMap;

fn main() {

    let mut map1: HashMap<String, i32> = HashMap::new();
    map1.insert(String::from("value1"), 10);
    map1.insert(String::from("value2"), 30);

    let mut map2: HashMap<String, i32> = HashMap::new();
    map2.insert(String::from("value1"), 20);
    map2.insert(String::from("value3"), 40);

   
    let mut merged_map: HashMap<String, i32> = map1.clone();
    for (key, value) in map2 {
        merged_map.entry(key).or_insert(value);
    }


    let (max_key, max_value) = merged_map.iter().max_by_key(|&(_, value)| value).unwrap();
    println!("Key with the highest value: {} => {}", max_key, max_value);

    let key_to_remove = "value1";
    merged_map.remove(key_to_remove);

    println!("Merged HashMap after removing key '{}': {:?}",key_to_remove, merged_map);
}
