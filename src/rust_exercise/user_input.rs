// Topic: User input

// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed

// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

enum Option {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn power_msg(option: Option){

    match option {
        Option::Off => println!("Turning off"),
        Option::Sleep => println!("Putting to sleep"),
        Option::Reboot => println!("Rebooting"),
        Option::Shutdown => println!("Shutting down"),
        Option::Hibernate => println!("Hibernating"),
    }
}

fn main() {

    println!("Enter your Option: ");
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("msg");

    let user_input = user_input.trim().to_lowercase();


    let option_match =  match user_input.as_str(){
        "off" => Option::Off,
        "sleep" => Option::Sleep,
        "reboot" => Option::Reboot,
        "shutdown" => Option::Shutdown,
        "hibernate" => Option::Hibernate,

        _ => {
            println!("Invalid power option");
            return;
        }
    };

    power_msg(option_match);

}
