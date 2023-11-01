use std::io;
use std::collections::HashMap;

fn main() {

    let v: Vec<i32> = Vec::new();
    let mut no = String::new();
    println!("Enter your went add name (enter 1) ");
    io::stdin().read_line(&mut no).expect("Failed to read input");
    let no : u32 = no.trim().parse().expect("msg");

  
  
    if no == 1{
        inser_data();
    }else {

    }

}

fn inser_data(){
    let mut scores = HashMap::new();

    let mut department = String::new();
    println!("Enter your department[1 or 2]: ");
    io::stdin().read_line(&mut department).expect("Failed to read input");
    let department: u32 = department.trim().parse().expect("msg");

    if department == 1{
        let mut name = String::new();
        println!("Enter your name: ");
        io::stdin().read_line(&mut name).expect("Failed to read input");
        scores.insert(String::from("Sales"), name.trim().to_string());
    }else if department  == 2{
        let mut name1 = String::new();
        println!("Enter your second name: ");
        io::stdin().read_line(&mut name1).expect("Failed to read input");
        scores.insert(String::from("Engineering"), name1.trim().to_string());
    }
    for (key, value) in &scores {
        println!("Department: {}, Name: {}", key, value);
    }
}


