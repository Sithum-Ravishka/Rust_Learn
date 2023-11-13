use std::io::Write;
use std::io;

fn user_input()  -> io::Result<String>{
    
    let mut input = String::new();
    print!("Enter Your Input: ");
    io::stdout().flush()?; // get user input in one line
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}

fn main(){

    let mut all_input = vec![];

    let mut no_input = 0;
    while no_input < 5 {
        match user_input() {
            Ok(words) => {
                all_input.push(words);
                no_input += 1;
            }
            Err(e) => println!("{e}")
        }


    }
    let mut total_value = 0; 

    for input in &all_input {

        if let Ok(num) = input.parse::<i32>() {
            total_value += num;
        } else {
            println!("Invalid input: {}", input);
        }

        println!("Original Value: {:?}, After Upper Case: {:?}", input, input.to_uppercase());
    }

    println!("Total Value: {}", total_value);
}