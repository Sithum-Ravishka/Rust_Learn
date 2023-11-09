// Topic: Working with expressions

fn main() {
    // * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100

    let value = 800;
    let value_100 = value > 100;

    print_out(value_100);
}

// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn print_out(get_value: bool){
    match get_value{
        true => println!("its big"),
        false => println!("its small")
    }
}

