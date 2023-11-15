// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io;
use std::collections::HashMap;

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn add_bill(bills: &mut HashMap<String, i32>) {
    let name = get_input("Enter the name of the bill:");
    let amount: Result<i32, _> = get_input("Enter the amount owed:").parse();

    match amount {
        Ok(amount) => {
            bills.insert(name, amount);
            println!("Bill added successfully!");
        }
        Err(_) => println!("Invalid amount. Please enter a valid number."),
    }
}

fn view_bills(bills: &HashMap<String, i32>) {
    if bills.is_empty() {
        println!("No bills to display.");
    } else {
        println!("Existing bills:");
        for (name, amount) in bills {
            println!("Name: {}, Amount: {}", name, amount);
        }
    }
}

fn remove_bill(bills: &mut HashMap<String, i32>) {
    view_bills(bills);

    let name = get_input("Please enter the name of the bill you want to remove: ");

    if let Some(amount) = bills.remove(&name) {
        println!("Removed bill: {} - Amount: {}", name, amount);
    } else {
        println!("Bill '{}' not found.", name);
    }
}

fn update_bill(bills: &mut HashMap<String, i32>) {
    view_bills(bills);

    let name = get_input("Please enter the name of the bill you want to update: ");

    if let Some(amount) = bills.remove(&name) {
        println!("Removed bill: {} - Amount: {}", name, amount);

        let updated_amount: Result<i32, _> = get_input("Enter the updated amount:").parse();

        match updated_amount {
            Ok(updated_amount) => {
                bills.insert(name, updated_amount);
                println!("Bill updated successfully!");
            }
            Err(_) => println!("Invalid amount. Please enter a valid number."),
        }
    } else {
        println!("Bill '{}' not found.", name);
    }
}


fn main() {
    let mut bills = HashMap::new();

    loop {
        println!("Menu:");
        println!("1. Add a bill");
        println!("2. View bills");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("5. Quit");

        let choice = get_input("Enter your choice:");

        match choice.as_str() {
            "1" => add_bill(&mut bills),
            "2" => view_bills(&bills),
            "3" => remove_bill(&mut bills),
            "4" => update_bill(&mut bills),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
