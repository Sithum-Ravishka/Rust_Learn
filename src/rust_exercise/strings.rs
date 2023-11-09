// Topic: Strings

// * Print out the name and favorite colors of people aged 10 and under

// * Use a struct for a persons age, name, and favorite color
struct Persons{
    age: i32,
    // * The color and name should be stored as a String
    name: String,
    fav_color: String
}


fn main() {
// * Create and store at least 3 people in a vector
    let persons1 = vec![
        Persons {
            age : 12,
            name: String::from("sithum"),
            fav_color: String::from("black")
        },
        Persons {
            age : 9,
            name: String::from("ravishka"),
            fav_color: String::from("red")
        },
        Persons {
            age : 5,
            name: String::from("baby"),
            fav_color: String::from("green")
        },
    ];

    // * Iterate through the vector using a for..in loop

    for list in persons1 {
    // * Use an if expression to determine which person's info should be printed
        if list.age <= 10{
            // * The name and colors should be printed using a function
            print(&list.name);
            print(&list.fav_color)
        }
    }

    fn print(data: &str){
        println!("{:?}", data);
    }
    
}
