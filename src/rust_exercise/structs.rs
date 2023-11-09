// Topic: Organizing similar data using structs

// * Print the flavor of a drink and it's fluid ounces
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavors{
    Colo,
    Rose,
    Vanila
 }
struct Drinks{
   flavor:  Flavors,
   flavor_ou: f64
 }

 fn print_out(drink: Drinks){
    
    match drink.flavor{
        Flavors::Colo => println!("Flavor: Colo"),
        Flavors::Rose => println!("Flavor: Rose"),
        Flavors::Vanila => println!("Flavor: Vanila"),
    }
    println!("oz: {:?}", drink.flavor_ou);
 }

fn main() {
 let colo = Drinks {
    flavor: Flavors::Colo,
    flavor_ou: 0.9,
 };
 print_out(colo);

  let rose = Drinks {
    flavor: Flavors::Rose,
    flavor_ou: 10.2,
 };
 print_out(rose);  

 let vanila = Drinks {
    flavor: Flavors::Vanila,
    flavor_ou: 2.0,
 };
 print_out(vanila);
}