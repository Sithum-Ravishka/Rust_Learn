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