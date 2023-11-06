///Calculate average
/// 

fn find_average(slice: &[f64]) -> f64 {
    // Check if the input slice is empty
    if slice.is_empty() {
        return 0.0;
    }

    // Calculate the sum of the numbers in the slice
    let sum: f64 = slice.iter().sum();

    // Calculate the average by dividing the sum by the number of elements in the slice
    let average = sum / slice.len() as f64;

    average
}
fn main() {
    let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let average = find_average(&numbers);
    println!("The average is: {}", average);

    let empty_slice: Vec<f64> = vec![];
    let average_empty = find_average(&empty_slice);
    println!("The average of the empty slice is: {}", average_empty);
}


///new **********************************//
/// 

fn find_average(xs: &[f64]) -> f64 {
    match xs.len() {
        0 => 0.,
        n => xs.iter().sum::<f64>() / n as f64
    }
}


//ssssssssssss

fn find_average(slice: &[f64]) -> f64 {
    let total: f64 = slice.iter().sum();
    if slice.len() > 0 {
        return total / slice.len() as f64;
    }
    0.0
}