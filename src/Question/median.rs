fn main() {
    let mut median = vec![1, 22, 32,43,  5, 6, 20];  
    median.sort();   //1 5 6 20 22 32 43 

    let len = median.len();
    let middle = (len - 1) / 2;

    let median_value = median.get(middle);

    match median_value {
        Some(&median) => println!("The median is {}", median),
        None => println!("There is no value."),
    }


}
