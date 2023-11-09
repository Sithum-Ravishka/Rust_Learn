// Topic: Data management using tuples

// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5

// * Use a function that returns a tuple

fn coordinate() -> (i32, i32){
    (1,5)
}

// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn main() {
    let (x, y) =coordinate();

    if y > 5{
        println!("y value grater than 5");
    }else if y < 5 {
        println!("y value less than 5");
    }else{
        println!("y value is equal to 5")
    }
}
