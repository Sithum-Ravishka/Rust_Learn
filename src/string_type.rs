use std::io;

fn main() {
    let mut s: String = String::new();

    println!("enter s value: ");

    io::stdin().read_line(&mut s).expect("msg");

    let mut s1: String = String::new();

    println!("enter s1 value: ");

    io::stdin().read_line(&mut s1).expect("msg");

    // the method also works on a literal directly:
    let s = "initial contents".to_string();    

    let s1 = s1.to_string();

    println!("{s} \n");    // output >>>>  initial contents
    println!("{s1}");      // output >>>>  sithum ( what user type )
}






fn main() {

    let s: String = String::from("initial contents");


    println!("{s} \n");    // output >>>>  initial contents

}




fn main() {
    let mut s = String::from("foo");
    s.push_str(" bar");


    println!("{s}");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s1}");

}



fn main() {
    let mut s = String::from("lo");
    s.push('l');


    println!("{s}");
}



use std::io;

fn main() {
    let mut s = String::new();
    println!("Enter your name : ");
    

    io::stdin().read_line(&mut s).expect("msg");


    let s1 = String::from("Hello, ");

    let s3 = s1 + {&s}; // note s1 has been moved here and can no longer be used

    println!("{s3}");
}




fn main() {
    let hello = "Здравствуйте";

    let s = &hello[0..8]; // this work but not work[0..1] and [0..5] like this because there 2 space need string value
    
    println!("{s}");
}



fn main() {
    for c in "Зд".chars() {
        println!("{c}");
    }
}



fn main() {
    for b in "Зд".bytes() {
        println!("{b}");
    }    
}