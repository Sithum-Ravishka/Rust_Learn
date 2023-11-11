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

        for (name , age) in test.iter() {
            println!("Age for {:?} = {:?}", name, age)
        } //test.keys() ---- only for print name
    }     //test.value() ---- only for print age
}

// Output

// Age for "sithum" = 33
// Age for "ds" = 33
// Age for "sdsad3" = 33