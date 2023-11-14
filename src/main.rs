use std::io;

fn main() {
    let mut first_no = String::new();
    let mut total :f32;
    let mut operator = String::new();

    // First number
    loop {
        println!("Enter First No: ");
        first_no.clear();
        io::stdin().read_line(&mut first_no).expect("Failed to read line");
        match first_no.trim().parse() {
            Ok(first_no) => {
                total = first_no;
                break;
            }
            Err(_) => {
                println!("Please enter a numbers.");
            }
        }
    }

    loop {
        // Arithmetic operator
        println!("Enter the operator (+, -, *, /) (or '=' to exit): ");
        operator.clear(); 
        io::stdin().read_line(&mut operator).expect("Failed to read line");
        let operator = operator.trim();

        if operator == "=" {
            println!("Total Result: {}", total);
            break;
        }

        if operator == "+" || operator == "-" || operator == "*" || operator == "/" {
            let mut second_no = String::new();
        
            loop {
                println!("Enter Second No: ");
                second_no.clear();
                io::stdin().read_line(&mut second_no).expect("Failed to read line");
                match second_no.trim().parse() {
                    Ok(second_no) => {
                        total = calculation_option(total, second_no, operator);
                        println!("Current Result: {}", total);
                        break;
                    }
                    Err(_) => {
                        println!("Please enter a numbers.");
                    }
                }
            }
            
        } else {
            println!("Invalid operator. Please enter a valid operator (+, -, *, /) or '=' to exit.");
        }
    }
}

fn calculation_option(first_no: f32, second_no: f32, operator: &str) -> f32 {
    match operator {
        "+" => first_no + second_no,
        "-" => first_no - second_no,
        "*" => first_no * second_no,
        "/" => {
            if second_no != 0.0 {
                first_no / second_no
            } else {
                println!("Division by zero is not allowed");
                first_no
            }
        }
        _ => {
            println!("Invalid operator");
            first_no 
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let result = calculation_option(2.0, 2.0, "+");
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_subtraction() {
        let result = calculation_option(5.0, 3.0, "-");
        assert_eq!(result, 2.0);
    }

    #[test]
    fn test_multiplication() {
        let result = calculation_option(3.0, 4.0, "*");
        assert_eq!(result, 12.0);
    }

    #[test]
    fn test_division() {
        let result = calculation_option(10.0, 2.0, "/");
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_division_by_zero() {
        let result = calculation_option(5.0, 0.0, "/");
        assert_eq!(result, 5.0);
    }
}

