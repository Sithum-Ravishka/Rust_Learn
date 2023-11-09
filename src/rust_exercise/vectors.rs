// Topic: Vectors

// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector

// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    let number = vec![10,20,30,40];

    for num in &number{

        match num{
            30 => println!("number: thirty"),
            _ => print!("number: {:?}\n", num)
        }
    }

    println!("Number of element: {:?}", number.len());
}
