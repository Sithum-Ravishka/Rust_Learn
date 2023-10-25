use std::io;

fn main(){

    println!("Enter your fahrenheit: ");

    let mut f = String::new();

    io::stdin()
        .read_line(&mut f)
        .expect("Failed to read_line");

    let f: f32 = f.trim().parse().expect("Please enter integer value");

    let c: f32 = (f - 32.0) * 5.0/9.0;
    

    println!("Celsius is: {:.2}", c);
}