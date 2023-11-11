fn main() {
    let number = 1..=3;
    for num in number {
        println!("{:?}", num);
    }

    let number1 = 1..6;
    // println!("{:?}", number1);  -----> 1..6
    for num in number1 {
        println!("{:?}", num);
    }

    let abc = 'a'..='f';
    // println!("{:?}", abc); -----> 'a'..='f'
    for ch in abc {
        println!("{:?}", ch);
    }

    let maybe_user = Some("jerry");

    // match maybe_user{
    //     Some(user) => println!("user = {:?}", user),
    //     None => println!("No user")
    // }

    if let Some(user) = maybe_user {
        println!("user = {:?}", user);
    } else {
        println!("No user");
    }

    let red = Color::Red; // check to equal use this

    if let Color::Red = red {
        println!("It's Red");
    } else {
        println!("It's not Red");
    }

    if let Color::Blue = red {
        println!("It's Blue");
    } else {
        println!("It's not Blue");
    }
}

enum Color {
    Red,
    Blue,
}
