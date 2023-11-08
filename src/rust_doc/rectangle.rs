#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:#?}", rect1);
}


// [[bin]]
// name = "rectangle"
// path = "src/rectangle.rs"



#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Without `self` keyword as its first parameter, this is an associated function that returns a new instance of `Rectangle`.
    // `Self` is an alias for the type `Rectangle`.
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
        }
    }

    // `&self` borrows an immutable instance of the Rectangle type and reads its fields
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // `&mut self` borrows a mutable instance of the Rectangle type and can mutate its fields
    fn change_width(&mut self, new_width: u32) {
        self.width = new_width;
    }
}

fn main() {
    let mut rect = Rectangle::new(30, 50);

    rect.change_width(60);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    // => The area of the rectangle is 3000 square pixels.
}