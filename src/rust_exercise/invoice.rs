use std::collections::HashMap;
use std::io;

struct StokeItem {
    quantity: i32,
    amount: i32,
}

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn add_stoke(stoke: &mut HashMap<String, StokeItem>) {
    let name = get_input("Enter the item name of the stock:");
    let quantity = get_input("Enter the quantity:");
    let amount = get_input("Enter the item price:");

    let stoke_item = StokeItem {
        quantity: quantity.parse().unwrap_or(0),
        amount: amount.parse().unwrap_or(0),
    };

    stoke.insert(name, stoke_item);
    println!("Stock item added successfully!");
}

fn view_stoke(stoke: &HashMap<String, StokeItem>) {
    if stoke.is_empty() {
        println!("No stock to display.");
    } else {
        println!("Existing stock:");
        for (name, item) in stoke {
            println!("Name: {}, Quantity:{}, Amount: {}", name, item.quantity, item.amount);
        }
    }
}

fn remove_stoke(stoke: &mut HashMap<String, StokeItem>) {
    view_stoke(stoke);

    let name = get_input("Please enter the name of the stock item you want to remove: ");

    if let Some(item) = stoke.remove(&name) {
        println!("Removed stock item: {} - Amount: {}", name, item.amount);
    } else {
        println!("Stock item '{}' not found.", name);
    }
}

fn update_stoke(stoke: &mut HashMap<String, StokeItem>) {
    view_stoke(stoke);

    let name = get_input("Enter the name of the stock item to update: ");

    if let Some(mut item) = stoke.remove(&name) {
        println!("Removed stock item: {} - Amount: {}", name, item.amount);

        if let Ok(updated_quantity) = get_input("Enter the updated quantity:").trim().parse() {
            if let Ok(updated_amount) = get_input("Enter the updated amount:").trim().parse() {
                item.quantity = updated_quantity;
                item.amount = updated_amount;
                stoke.insert(name.clone(), item);
                println!("Stock item updated successfully!");
            } else {
                println!("Invalid amount. Please enter a valid number.");
            }
        } else {
            println!("Invalid quantity. Please enter a valid number.");
        }
    } else {
        println!("Stock item '{}' not found.", name);
    }
}

fn add_bill(bill: &mut HashMap<String, i32>) {
    let name = get_input("Enter the item name of the bill:");
    let quantity: Result<i32, _> = get_input("Enter the stock quantity:").trim().parse();

    match quantity {
        Ok(quantity) => {
            bill.insert(name, quantity);
            println!("Item added to the bill successfully!");
        }
        Err(_) => println!("Invalid quantity. Please enter a valid number."),
    }
}

fn view_bill(bill: &HashMap<String, i32>, stoke: &HashMap<String, StokeItem>) {
    if bill.is_empty() {
        println!("No bill items to display.");
    } else {
        println!("Existing bill items:");
        for (name, quantity) in bill {
            if let Some(stoke_item) = stoke.get(name) {
                let total_amount = stoke_item.amount * *quantity;
                println!(
                    "Item: {}, Quantity: {}, Unit Price: {}, Total Amount: {}",
                    name, quantity, stoke_item.amount, total_amount
                );
            } else {
                println!(
                    "Item: {}, Quantity: {}, (Item not found in stock)",
                    name, quantity
                );
            }
        }
    }
}

fn remove_bill(bill: &mut HashMap<String, i32>) {
    view_bill(bill, &HashMap::new()); // Passing an empty stock HashMap for consistency

    let name = get_input("Please enter the name of the bill item you want to remove: ");

    if let Some(quantity) = bill.remove(&name) {
        println!("Removed bill item: {} - Quantity: {}", name, quantity);
    } else {
        println!("Bill item '{}' not found.", name);
    }
}

fn update_bill(bill: &mut HashMap<String, i32>) {
    view_bill(bill, &HashMap::new()); // Passing an empty stock HashMap for consistency

    let name = get_input("Enter the name of the bill item to update: ");

    if let Some(mut quantity) = bill.remove(&name) {
        println!("Removed bill item: {} - Quantity: {}", name, quantity);

        if let Ok(updated_quantity) = get_input("Enter the updated quantity:").trim().parse() {
            quantity = updated_quantity;
            bill.insert(name, quantity);
            println!("Bill item updated successfully!");
        } else {
            println!("Invalid quantity. Please enter a valid number.");
        }
    } else {
        println!("Bill item '{}' not found.", name);
    }
}

fn print_invoice(stoke: &HashMap<String, StokeItem>, bill: &HashMap<String, i32>) {
    if bill.is_empty() {
        println!("No items in the bill. Invoice cannot be generated.");
        return;
    }

    println!("Invoice:");

    let mut total_amount = 0;

    for (name, bill_quantity) in bill {
        if let Some(stoke_item) = stoke.get(name) {
            let item_total = stoke_item.amount * *bill_quantity;
            total_amount += item_total;
            println!(
                "Item: {}, Quantity: {}, Unit Price: {}, Item Total Amount: {}",
                name, bill_quantity, stoke_item.amount, item_total
            );
        } else {
            println!(
                "Item '{}' not found in stock. Cannot generate invoice for this item.",
                name
            );
        }
    }

    println!("Total Amount: {}", total_amount);
}

fn main() {
    let mut stoke = HashMap::new();
    let mut bill = HashMap::new();

    loop {
        println!("Main Menu:");
        println!("1. Stock Operations");
        println!("2. Bill Operations");
        println!("3. Print Invoice");
        println!("4. Quit");

        let choice = get_input("Enter your choice:");

        match choice.as_str() {
            "1" => stock_menu(&mut stoke),
            "2" => bill_menu(&mut bill, &stoke),
            "3" => print_invoice(&stoke, &bill),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}

fn stock_menu(stoke: &mut HashMap<String, StokeItem>) {
    loop {
        println!("Stock Menu:");
        println!("1. Add stock");
        println!("2. View stock");
        println!("3. Remove stock");
        println!("4. Update stock");
        println!("5. Return to Main Menu");

        let stock_choice = get_input("Enter your stock choice:");

        match stock_choice.as_str() {
            "1" => add_stoke(stoke),
            "2" => view_stoke(stoke),
            "3" => remove_stoke(stoke),
            "4" => update_stoke(stoke),
            "5" => break,
            _ => println!("Invalid stock choice. Please try again."),
        }
    }
}

fn bill_menu(bill: &mut HashMap<String, i32>, stoke: &HashMap<String, StokeItem>) {
    loop {
        println!("Bill Menu:");
        println!("1. Add bill");
        println!("2. View bill");
        println!("3. Remove bill");
        println!("4. Update bill");
        println!("5. Return to Main Menu");

        let bill_choice = get_input("Enter your bill choice:");

        match bill_choice.as_str() {
            "1" => add_bill(bill),
            "2" => view_bill(bill, stoke),
            "3" => remove_bill(bill),
            "4" => update_bill(bill),
            "5" => break,
            _ => println!("Invalid bill choice. Please try again."),
        }
    }
}
