fn main() {
    let mut data = Some(3);

    while let Some(i) = data {
        println!("loop"); 
        data = None;
    }

    let vec = vec![2, 3, 4];

    let mut number_iter = vec.iter();

    while let Some(num) = number_iter.next() {  // runing one by one all element inside vec using iter and next
        println!("num = {:?}", num); // element print one by one
    }

    println!("Done");
}
