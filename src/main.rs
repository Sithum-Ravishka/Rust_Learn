use std::io;

fn main() {
    
    println!("Please enter number: ");
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read_line");

    let number: u32 = number .trim().parse().expect("Input value is not a number");

    if ({number} < 5) {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}









// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }