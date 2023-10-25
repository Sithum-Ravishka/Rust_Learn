use std::io;

fn main(){

    loop{
    println!("Enter number: ");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read_line");

    let n: u64 = n.trim().parse().expect("Please enter integer value");

    let result = f(n);
    println!("Fibonacci number at position {} is: {}", n, result);

}}


fn f(n: u64) -> u64 {
    if n <= 1 {
        return n as u64;
    } else {
        return f(n - 1) + f(n - 2);
        
    }
}