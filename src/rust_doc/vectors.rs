fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}


fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];  // code not run beacome error
    println!("{does_not_exist}");




    let does_not_exist = v.get(100); // this code run beacuse handle error with some / none 

    match does_not_exist {
        Some (does_not_exist) => println!("{does_not_exist}"), // if you don't know array index surely use this method
        None => println!("dsvasjd"),
    }
}




fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    v.push(6);

    let first = &v[5];

    // v.push(6);           // if you add here it is error because  ownership rule 


    println!("The first element is: {first}");
}



fn main() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        
        *i += 50;

        println!("{i}")
    }


}



fn main() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    println!("{:?}",row)
}