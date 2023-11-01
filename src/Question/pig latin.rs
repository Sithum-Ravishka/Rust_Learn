use std::io;

fn main() {
    let mut letter = String::new();

    println!("Enter your letter: ");

    io::stdin().read_line(&mut letter).expect("Failed to read input");

    let letter = letter.trim(); 

    let vowel = &letter[0..1];

    if vowel == "a" || vowel == "e" || vowel == "i" || vowel == "o" || vowel == "u" {
        let mut result = letter.to_string();
        result.push_str("-hay");
        println!("{}", result);

    } else {
        let first = &letter[0..1];
        let end = &letter[1..];

        let result1 = format!("{end}-{first}ay");

        println!("{}", result1);

    }
}
