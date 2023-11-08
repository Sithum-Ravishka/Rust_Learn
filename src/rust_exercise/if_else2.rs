// Topic: Flow control using if..else if..else

// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
// * Use an if..else if..else block to determine which message to display

fn main() {

    let number = 5;

    if number > 5{
        println!(">5");
    }else if number < 5 {
        println!("<5");
    }else {
        println!("=5");
    }
}
