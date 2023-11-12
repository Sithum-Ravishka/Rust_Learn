// Topic: External crates

// * Display the current date and time

// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

// [dependencies]
// chrono = "0.4"

use chrono::prelude::*;

fn main() {
    let local: DateTime<Local> = Local::now();

    println!("Current date and time: {}", local);

    let custom_format = local.format("%Y-%m-%d %H:%M:%S");
    println!("Custom formatted date and time: {}", custom_format);
}
