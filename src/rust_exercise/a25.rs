// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes

// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Cal {
    fn calculation(&self) -> i32;
}

struct Square{
    length: i32
}

impl Cal for Square {
    fn calculation(&self) -> i32{
        self.length * 4
}}

struct Triangle{
    len_a: i32,
    len_b: i32,
    len_c: i32
}

impl Cal for Triangle {
    fn calculation(&self) -> i32{
        self.len_a * self.len_b * self.len_c
}}


fn calculation_print (shape : impl Cal){
    let Cal = shape.calculation();

    println!("{:?}", Cal)
}

fn main() {
    let square = Square{
        length: 10
    };

    let triangle = Triangle{
        len_a: 3,
        len_b: 2,
        len_c: 8
    };
    calculation_print(square);
    calculation_print(triangle);
}
