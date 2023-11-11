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
