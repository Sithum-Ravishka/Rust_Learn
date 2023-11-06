// Exercise 5: Vector of HashMaps

//     Create a vector of hashmaps where each hashmap represents a person with keys for "name" and "age."
//     Add several people to the vector.
//     Find and print the person with the highest age in the vector.
//     Sort the vector based on age in ascending order.

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    // Create a vector of people (HashMaps) with "name" and "age" keys.
    let mut people: Vec<Person> = vec![
        Person { name: "Alice".to_string(), age: 25 },
        Person { name: "Bob".to_string(), age: 30 },
        Person { name: "Charlie".to_string(), age: 40 },
    ];

    // Find and print the person with the highest age in the vector.
    let max_age_person = people.iter().max_by_key(|person| &person.age);
    match max_age_person {
        Some(person) => println!("Person with the highest age: {:?}", person),
        None => println!("Vector is empty"),
    }

    // Sort the vector based on age in ascending order.
    people.sort();

    // Print the sorted vector.
    println!("Sorted vector by age: {:?}", people);
}
