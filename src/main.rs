fn main() {
    let config_max = Some(54u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
