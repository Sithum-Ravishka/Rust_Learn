use std::collections::HashMap;
use std::io;

fn main() {
    let mut eng = HashMap::new();
    let mut sal = HashMap::new();

    loop {
        println!("What is department? (Engineering/Sales)");
        let mut department = String::new();
        io::stdin()
            .read_line(&mut department)
            .expect("Failed to read input");

        let department = department.trim().to_lowercase();

        if department == "engineering" {
            let mut names: Vec<String> = Vec::new();

            loop {
                let mut input = String::new();

                println!("Enter a name: ");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read input");

                let input_trimmed = input.trim().to_lowercase();

                names.push(input_trimmed.to_string());

                println!("Do you want to add another name? (yes/no)");
                let mut add_more = String::new();
                io::stdin()
                    .read_line(&mut add_more)
                    .expect("Failed to read input");

                let add_more_trimmed = add_more.trim().to_lowercase();

                if add_more_trimmed != "yes" {
                    eng.entry("Engineering".to_string()).or_insert(Vec::new()).extend(names);
                    break; // Exit the inner loop
                }
            }
        } else if department == "sales" {
            let mut sales_dep: Vec<String> = Vec::new();

            loop {
                let mut input_sales = String::new();

                println!("Enter a name: ");
                io::stdin()
                    .read_line(&mut input_sales)
                    .expect("Failed to read input");

                let input_trimmed = input_sales.trim().to_lowercase();

                sales_dep.push(input_trimmed.to_string());

                println!("Do you want to add another name? (yes/no)");
                let mut add_more = String::new();
                io::stdin()
                    .read_line(&mut add_more)
                    .expect("Failed to read input");

                let add_more_trimmed = add_more.trim().to_lowercase();

                if add_more_trimmed != "yes" {
                    sal.entry("Sales".to_string()).or_insert(Vec::new()).extend(sales_dep);
                    break; // Exit the inner loop
                }
            }
        } else {
            println!("Invalid department. Please enter 'Engineering' or 'Sales'.");
        }

        println!("Do you want to add names for another department? (yes/no)");
        let mut add_department = String::new();
        io::stdin()
            .read_line(&mut add_department)
            .expect("Failed to read input");

        let add_department_trimmed = add_department.trim().to_lowercase();

        if add_department_trimmed != "yes" {
            break; // Exit the outer loop
        }
    }

    loop {
        println!("Options:");
        println!("1. Retrieve a list of all people in Engineering (sorted alphabetically)");
        println!("2. Retrieve a list of all people in Sales (sorted alphabetically)");
        println!("3. Retrieve a list of all people in the company by department (sorted alphabetically)");
        println!("4. Exit");

        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read input");

        match option.trim() {
            "1" => {
                if let Some(names) = eng.get("Engineering") {
                    let mut sorted_names = names.clone();
                    sorted_names.sort();
                    println!("People in Engineering (sorted alphabetically): {:?}", sorted_names);
                } else {
                    println!("No names in Engineering.");
                }
            }
            "2" => {
                if let Some(names) = sal.get("Sales") {
                    let mut sorted_names = names.clone();
                    sorted_names.sort();
                    println!("People in Sales (sorted alphabetically): {:?}", sorted_names);
                } else {
                    println!("No names in Sales.");
                }
            }
            "3" => {
                let mut all_names: Vec<(String, Vec<String>)> = Vec::new();
                all_names.push(("Engineering".to_string(), eng.get("Engineering").cloned().unwrap_or_default()));
                all_names.push(("Sales".to_string(), sal.get("Sales").cloned().unwrap_or_default()));
                all_names.sort_by(|a, b| a.0.cmp(&b.0));
                for (dept, names) in all_names {
                    let mut sorted_names = names.clone();
                    sorted_names.sort();
                    println!("People in {} (sorted alphabetically): {:?}", dept, sorted_names);
                }
            }
            "4" => {
                println!("Goodbye!");
                return;
            }
            _ => {
                println!("Invalid option. Please select a valid option.");
            }
        }
    }
}
