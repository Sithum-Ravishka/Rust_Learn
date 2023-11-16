use std::collections::HashMap;
use std::io;

fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed Read line");
    input.trim().to_string()
}
enum MainMenu{
    StockMenu,
    BillMenu,
    PrintInvoice
}

enum BillMenu{
    AddBill,
    ViewBill,
    UpdateBill,
    RemoveBill
}

enum StokeMenu{
    AddStoke,
    ViewStoke,
    UpdateStoke,
    RemoveStoke
}

fn add_bill(bill: &mut HashMap<String, i32>, stoke: &mut HashMap<String, StokeItem>){
    view_stoke(stoke);
    let name = get_input("Enter item name buyer want buy: ");
    let quantity: Result<i32, _> = get_input("Enter quantity buyer want buy: ").trim().parse();

    match quantity {
        Ok(quantity) => {
            if let Some(stoke_item) = stoke.get_mut(&name){
                if stoke_item.quantity >= quantity{
                    stoke_item.quantity -= quantity;
                    bill.insert(name , quantity);
                    println!("✅✅  Bill item added successfully ✅✅");
                }else {
                    println!("❌❌ Not enough stoke available for '{}' ❌❌", name);
                }
            }else {
                    println!("❌❌ Item '{}' not found in stock. ❌❌", name);
            }
        }
        Err(_) => println!("❌❌ Invalid quantity. Please enter a valid number ❌❌"),            
    }

}
fn view_bill(bill: &HashMap<String, i32>){
    if bill.is_empty(){
        println!("❌❌  No bill added to display ❌❌");
    }else{
        println!("✴️ ✴️ ✴️  Added bill item Lists ✴️ ✴️ ✴️");
        for(name, item) in bill{
            println!("⬜⬜⬜ Item name: {} Quantity: {}", name, item);
        }
    }
}

fn update_bill(bill: &mut HashMap<String, i32>, stoke: &mut HashMap<String, StokeItem>){
    view_bill(bill);

    let name =  get_input("Enter name of item went update: ");

    if let Some(quantity) = bill.get_mut(&name){
        println!("Currently buyer want buy {} quantity of {} item ", quantity, name);

        if let Ok(updated_quantity) = get_input("Enter new quantity: ").trim().parse(){
            if let Some(stoke_item) = stoke.get_mut(&name){
                if stoke_item.quantity + *quantity >= updated_quantity{
                    stoke_item.quantity += *quantity - updated_quantity;
                    *quantity = updated_quantity;
                    println!("✅✅  Bill item updated successfully ✅✅");
                } else {
                    println!("❌❌ Not enough stock available for '{}'. ❌❌", name);
                }
            }else {
                println!("❌❌ Item '{}' not found in stoke ❌❌", name);
            }
        }else {
            println!("❌❌ Invalid quantity. Please enter valid number. ❌❌")
        }
    }else {
        println!("❌❌ Item '{}' not found in bill ❌❌", name);
    }
}

fn remove_bill(bill: &mut HashMap<String, i32>, stoke: &mut HashMap<String, StokeItem>){
    view_bill(bill);

    let name = get_input("Enter item name want remove: ");

    if let Some(quantity) = bill.remove(&name){
        if let Some(stoke_item) = stoke.get_mut(&name){
            stoke_item.quantity += quantity;
            println!("Removed bull item: {} and Quantity: {}", name, quantity);
        }else {
            println!("❌❌ Item '{}' not found in stoke ❌❌", name);
        }
    } else {
        println!("❌❌ Item '{}' not found in bill ❌❌", name);
    }
}


fn main() {
    let mut stoke = HashMap::new();
    let mut bill = HashMap::new();

    loop {
        println!("🔶🔶🔶  Main Menu 🔶🔶🔶");
        println!("1. Stock Management");
        println!("2. Bill Add");
        println!("3. Print Invoice");
        println!("4. Exit");

        let option = get_input("Enter Your Option: ");

        match option.as_str() {
            "1" => stock_menu(&mut stoke),
            "2" => bill_menu(&mut bill, &mut stoke),
            "3" => print_invoice(&bill, &stoke),
            "4" => {
                println!("Exiting......");
                print_invoice(&bill, &stoke);
                break;
            }
            _ => println!("❌❌ Invalid Option! Please Enter Valid Option ❌❌"),
        }
    }
}

fn bill_menu(bill: &mut HashMap<String, i32>, stoke: &mut HashMap<String, StokeItem>) {
    loop{
        println!("🔷🔷🔷  Bill Menu 🔷🔷🔷");
        println!("1. Add bill");
        println!("2. View bill");
        println!("3. Update bill");
        println!("4. Remove bill");
        println!("5. Return to main-menu ↩️");

        let bill_option = get_input("Enter Your Option: ");

        match bill_option.as_str(){
            "1" => add_bill(bill, stoke),
            "2" => view_bill(bill),
            "3" => update_bill(bill, stoke),
            "4" => remove_bill(bill, stoke),
            "5" => break,
            _ => println!("❌❌ Invalid Option! Please Enter Valid Option ❌❌")
        }
    }
}

fn stock_menu(stoke: &mut HashMap<String, StokeItem>) {
    loop{
        println!("🔷🔷🔷  Stoke Menu 🔷🔷🔷");
        println!("1. Add stoke");
        println!("2. View stoke");
        println!("3. Update stoke");
        println!("4. Remove stoke");
        println!("5. Return to main-menu ↩️");

        let bill_option = get_input("Enter Your Option: ");

        match bill_option.as_str(){
            "1" => add_stoke(stoke),
            "2" => view_stoke(stoke),
            "3" => update_stoke(stoke),
            "4" => remove_stoke(stoke),
            "5" => break,
            _ => println!("❌❌ Invalid Option! Please Enter Valid Option ❌❌")
        }
    }
}

struct StokeItem{
    quantity: i32,
    price: i32,
}

fn add_stoke(stoke: &mut HashMap<String, StokeItem>){
    let name = get_input("Enter item name: ");
    let quantity = get_input("Enter no of quantity: ");
    let price = get_input("Enter item price: ");

    let stock_item = StokeItem{
        quantity: quantity.parse().expect("msg"),
        price: price.parse().expect("msg"),
    };

    stoke.insert(name, stock_item);
    println!("✅✅  Stoke item added successfully ✅✅");
}

fn view_stoke(stoke: &HashMap<String, StokeItem>){
    if stoke.is_empty(){
        println!("❌❌  No stoke item to display ❌❌");
    }else {
        println!("📦📦  Available Item List in Stoke 📦📦");
        for (name, item) in stoke{
            println!("🟩🟩🟩🟩🟩 Item Name: {}, Quantity: {}, Price: {}", name, item.quantity, item.price);
        }
    }
}

fn update_stoke(stoke: &mut HashMap<String, StokeItem>){
    view_stoke(stoke);

    let name = get_input("Enter name of Item, You want to update: ");

    if let Some(item) = stoke.get_mut(&name){
        println!("🔰🔰 Current quantity {} and price {}", item.quantity, item.price);

        if let Ok(updated_quantity) = get_input("Enter new quantity: ").trim().parse(){
            if let Ok(updated_price) = get_input("Enter new price: ").trim().parse(){
                item.quantity = updated_quantity;
                item.price = updated_price;
                println!("✅✅  Stoke item updated successfully ✅✅");
            }else {
                println!("❌❌ Invalid price. Please enter valid number. ❌❌");
            }
        }else {
            println!("❌❌ Invalid quantity. Please enter valid number. ❌❌");
        }
    }else {
            println!("Stoke item ❌{}❌ Not found", name);
        }
    }

fn remove_stoke(stoke: &mut HashMap<String, StokeItem>){
    view_stoke(stoke);

    let name = get_input("Please enter item name, You want remove: ");

    if let Some(item) = stoke.remove(&name){
        println!("Removed stoke item: {}, quantity: {}, price: {}", name, item.quantity, item.price);
    }else{
        println!("Stoke item ❌{}❌ Not found", name);
    }
}

fn print_invoice(bill: &HashMap<String, i32>, stoke: &HashMap<String, StokeItem>) {
    if bill.is_empty(){
        println!("NO item in bill. Invoice not create");
    }

    println!("🔰🔰🔰🔰🔰 Invoice 🔰🔰🔰🔰🔰");

    let mut total_price = 0;

    for(name, bill_quantity) in bill{
        if let Some(stoke_item) = stoke.get(name){
            let item_price = stoke_item.price * *bill_quantity;
            total_price += item_price;
            println!("Item Name: {} Quantity: {}, Price: {}, Total Price for item: {}", name, bill_quantity, stoke_item.price, item_price);
        }else {
            println!("❌❌ Item name '{}' not found in stock ❌❌", name)
        }
    }
    println!("Total Price for bill: {}", total_price);
}
