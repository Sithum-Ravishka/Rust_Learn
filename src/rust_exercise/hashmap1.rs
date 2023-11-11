use std::collections::HashMap;
use std::io;

fn main() {
    let mut test = HashMap::new();

    loop {
        let mut name = String::new();
        println!("Enter name:");

        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        let name = name.trim().to_string();

        let mut input = String::new();

        println!("Enter number:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number. Please enter a valid number.");
                continue;
            }
        };

        test.insert(name.clone(), input);

        match test.get(&name) {
            Some(age) => println!("Age for {} = {:?}", name, age),
            None => println!("Name not found"),
        }
    }
}



// Find length in hashmap

use std::collections::HashMap;

fn main() {
    let mut a = HashMap::new();
    a.insert(1, "a");
    a.insert(2, "a");
    a.insert(3, "a");
    let lengths = a.len();  
    println!("{:?}", lengths);  
}
