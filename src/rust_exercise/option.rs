// Topic: Option

// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students

    // * Use a struct containing the student's name and locker assignment
    // * The locker assignment should use an Option<i32>
struct Student{
    name: String,
    assignment: Option<i32>
}

fn main() {
    let student_details = Student{
        name : "sithum".to_owned(),
        assignment : Some(12)
    };

    println!("name: {:?}", student_details.name);
    match student_details.assignment {
        Some(ans) => println!("assigment: {:?}", ans),
        None => println!("Not response")
        
    }
}
