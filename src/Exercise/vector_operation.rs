// Exercise 2: Vector Operations

//     Create two vectors of integers.
//     Concatenate the two vectors into a single vector.
//     Remove all duplicate elements from the concatenated vector.
//     Sort the resulting vector in ascending order.
//     Calculate the average of the elements in the sorted vector.

fn main(){

    let  a = vec![20,33,34];

    let  b = vec![22,33,44];

    let mut concatenated = a.clone();
        concatenated.extend(b.iter());

        concatenated.sort();
        concatenated.dedup();

        let sum: i32 = concatenated.iter().sum();
        let average = sum as f64 / concatenated.len() as f64;

    println!("Single Vector{:?}", concatenated);
    println!("Average: {:.2}", average);
}
