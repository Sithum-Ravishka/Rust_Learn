// Topic: Browsing standard library documentation

// * Print a string in lowercase and uppercase

// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

fn main() {
    let s = "hello";
    let l ="WORLD";

    println!("Change to Uppercase: {:?}", s.to_uppercase());
    println!("Change to Lowercase: {:?}", l.to_lowercase());
}
