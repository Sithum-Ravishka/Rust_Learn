// Topic: Looping using the loop statement

// * Display "1" through "4" in the terminal

// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut number = 1;

    loop{
        println!("{number}");
        
        if number == 4 {
            break;
        }
        number += 1;
    }
}
