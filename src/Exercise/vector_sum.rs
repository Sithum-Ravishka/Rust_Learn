use std::io;

fn main() {
    let mut v: Vec<i32> = Vec::new();

    loop {
        let mut input = String::new();

        println!("Enter an integer or '00' to exit: ");
        io::stdin().read_line(&mut input).expect("Invalid Read_line");

        // Parse the input to an integer.
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                continue;
            }
        };

        if input == 00 {
            break; // Exit the loop if the input is 0.
        }

        v.push(input);

        println!("Vector elements: {:?}", v);

        // Find the sum of all elements in the vector.
        let sum: i32 = v.iter().sum();
        println!("Sum of all elements: {}", sum);
    }

    println!("Final Vector: {:?}", v);
    let final_sum: i32 = v.iter().sum();
        println!("Final Sum of all elements: {}", final_sum);
}

// Exercise 1: Vector Basics

//     Create an empty vector of integers.
//     Add elements to the vector using the push method.
//     Iterate through the vector and print each element.
//     Find the sum of all elements in the vector.