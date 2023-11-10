// Topic: Advanced match

// * Print out a list of tickets and their information for an event
// * All tickets include the price


// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
enum Tickets {
    Backstage (f64, String),
    Standard (f64),
    Vip (f64, String)
}

fn main() {
    // * Use an enum for the tickets with data associated with each variant
    // * Create one of each ticket and place into a vector
    let tickets = vec![
        Tickets::Backstage(50.0, "sithum".to_owned()),
        Tickets::Standard(60.0),
        Tickets::Vip(100.0, "ravishka".to_owned())
    ];

    // * Use a match expression while iterating the vector to print the ticket info

    for info in tickets {
        match info{
            Tickets::Backstage(price, holder_name) => println!("Backstage tiket price: {:?} Holder name: {:?}", price, holder_name), 
            Tickets::Standard(price) => println!("Standard tiket price: {:?}", price), 
            Tickets::Vip(price, holder_name) => println!("Vip tiket price: {:?} Holder name: {:?}", price, holder_name), 
        }
    }
}
