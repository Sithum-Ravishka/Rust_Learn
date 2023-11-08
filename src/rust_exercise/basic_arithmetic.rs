// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers

// * Use a function to add two numbers together

fn number(x :i32,  y:i32) -> i32 {
    x + y

}
// * Use a function to display the result

fn display_result(result :i32){
   println!("{:?}", result);
}

fn main() {
let result = number(20, 10);
display_result(result)
}
