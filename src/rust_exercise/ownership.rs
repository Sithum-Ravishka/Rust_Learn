// Topic: Ownership

// * Print out the quantity and id number of a grocery item

// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem{
    quantity: i32,
    id : i32
}

fn quantity(item: &GroceryItem){
    println!("grocery quantity: {:?}", item.quantity)
}

fn id_number(item: &GroceryItem){
    println!("grocery Id: {:?}", item.id)
}

fn main() {
    let myitem = GroceryItem{
        quantity : 3,
        id: 121,
    };
    quantity(&myitem);
    id_number(&myitem);
}
