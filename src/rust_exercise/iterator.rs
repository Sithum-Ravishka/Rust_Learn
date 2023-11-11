fn main() {
    let number = vec![1, 4, 5, 6, 3, 2];

    // Map and add value to 1
    let plus_one: Vec<_> = number.iter().map(|num| num + 1).collect();
    println!("{:?}", plus_one);

    // Filter Value (num > 3)
    let plus_two: Vec<_> = number.iter().filter(|num| *num >= &3).cloned().collect();
    println!("{:?}", plus_two);

    let number1 = vec![1, 4, 5, 6, 3, 2];
    // Find equal value
    let find: Option<&i32> = number1.iter().find(|num| *num == &9);
    println!("{:?}", find);


    let number_count = vec![1, 4, 5, 6, 3, 22, 10, 11];

    // Find count of Vec
    let count  = number_count.iter().count();
    println!("{:?}", count);

    // Find last value
    let last  = number_count.iter().last();
    println!("{:?}", last);

    // Find min value
    let min  = number_count.iter().min();
    println!("{:?}", min);

    // Find max value
    let max  = number_count.iter().max();
    println!("{:?}", max);

    // Take some value
    let take_value: Vec<_>  = number_count.iter().take(4).collect();
    println!("{:?}", take_value);

}
