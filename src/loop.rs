use std::io;

fn main() {
    let a = [10, 20, 30, 40, 50];

    println!("Please enter index number: ");

    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("msg");

    let mut index: usize = index.trim().parse().expect("not a number");

    

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    println!("Your entered index number is too high");
}







fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}





fn main() {
    loop {
        println!("again!");
        
    }
}







fn main() {
    let a = [10, 20, 30, 40, 50];
    for number in (1..5).rev() {
        if number == 1 || number == 4 {
            println!("{}", a[number as usize]);
        }
    }
}









fn main() {
    let mut count = 1;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 8 {
                break;
            }
            if count == 1 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}



fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
