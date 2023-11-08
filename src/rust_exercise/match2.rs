// Topic: Decision making with match

// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
// * Use an underscore (_) to match on any value

fn main() {

    let value = 1;

    match value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }
}
