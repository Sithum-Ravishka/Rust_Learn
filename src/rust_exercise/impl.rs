// Topic: Implementing functionality with the impl keyword

// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color

// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    Red,
    Yellow,
    Green
}

impl Color {
    fn print_out(&self){
        match self {
            Color::Red => println!("Red"),
            Color::Green => println!("Green"),
            Color::Yellow => println!("Yellow")
        };
    }
}

struct ShippingBox{
    dimensions: Dimensions,
    weight: i32,
    color: Color,
}


struct Dimensions{
    width: i32,
    height: i32,
    depth: i32
}

impl Dimensions {
    fn print_out(&self){
        println!("width {:?}", self.width);
        println!("height {:?}", self.height);
        println!("depth {:?}", self.depth);
    }
}

impl ShippingBox {
    fn create_new_box(weight: i32, color: Color, dimensions: Dimensions) -> Self{
        Self { 
            weight,            
            color,
            dimensions
        }
    }

    fn print_out(&self){
        self.color.print_out();
        self.dimensions.print_out();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let new_dimensions = Dimensions {
        width: 44,
        height: 55,
        depth: 3
    };

    let small_box = ShippingBox::create_new_box(5, Color::Red, new_dimensions);

    small_box.print_out();
}
