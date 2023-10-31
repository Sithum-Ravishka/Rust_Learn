use std::io;

fn main(){
    println!("Enter First Number: ");

    let mut num1 = String::new();

    io::stdin()
    .read_line(&mut num1)
    .expect("msg");

    let num1: u32 = num1.trim().parse().expect("msg");

    println!("Enter secound Number: ");

    let mut num2 = String::new();

    io::stdin()
    .read_line(&mut num2)
    .expect("msg");

    let num2: u32 = num2.trim().parse().expect("msg");

    let total = num1 + num2;

    print!("Total value is:  {total}");


}