use std::io;

fn main() {
    let mut first_no = String::new();
    let mut total = 0; // Initialize a total variable
    let mut operator = String::new();

    // First number
    println!("Enter first No : ");
    io::stdin().read_line(&mut first_no).expect("Failed to read input");
    let first_no: i32 = first_no.trim().parse().expect("Please enter an integer value");

    loop {
        // Arithmetic operator
        println!("Enter the operator (+, -, *, /) (or '=' to exit): ");
        operator.clear(); // Clear the previous operator input
        io::stdin().read_line(&mut operator).expect("Failed to read input");
        let operator = operator.trim();

        if operator == "=" {
            break;
        }

        if operator == "+" || operator == "-" || operator == "*" || operator == "/" {
            loop{
                let mut second_no = String::new();
                println!("Enter Second No: ");
                io::stdin().read_line(&mut second_no).expect("Failed to read input");
                let second_no: i32 = match second_no.trim().parse(){
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid input. Please enter a valid number.");
                        continue;
                    }
                };
            }


            total = calculation_option(total, second_no, operator); // Update the total
            println!("Total Result: {}", total);
        } else {
            println!("Invalid operator. Please enter a valid operator (+, -, *, /) or '=' to exit.");
        }
    }
}

fn calculation_option(first_no: i32, second_no: i32, operator: &str) -> i32 {
    match operator {
        "+" => first_no + second_no,
        "-" => first_no - second_no,
        "*" => first_no * second_no,
        "/" => {
            if second_no != 0 {
                first_no / second_no
            } else {
                println!("Division by zero is not allowed");
                first_no // Return the original value when dividing by zero
            }
        }
        _ => {
            println!("Invalid operator");
            first_no // Return the original value on an invalid operator
        }
    }
}
