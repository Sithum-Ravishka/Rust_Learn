// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
#[derive(Debug)]
struct Adult{
    age: u8,
    name: String
}
//   * Implement Debug print functionality using `derive`

// * Implement a `new` function for the `Adult` structure that returns a Result:
impl Adult{
    //   * The Ok variant should contain the initialized structure, but only
    //     if the person is aged 21 or older
    fn new(age: u8, name: &str) -> Result<Self, &str>{
        if age >= 21{
           Ok(Self{
            age,
            name: name.to_owned(),
           })
        }else{
            Err("Age must be at least 21")
        }
    }
}

fn main() {
    // * Instantiate two `Adult` structures:
    //   * One should be 21 or over
    let adult = Adult::new(23, "Sithum");
    //   * One should be aged under 21
    let child = Adult::new(19, "Ravishka");

    match child {
        Ok(child) => println!("{} is {} years old", child.name, child.age),
        Err(e) => println!("{e}"),
    }

    match adult {
        Ok(adult) => println!("{} is {} years old", adult.name, adult.age),
        Err(e) => println!("{e}"),
    }

}


// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message