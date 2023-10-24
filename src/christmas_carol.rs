fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth",
        "sixth", "seventh", "eighth", "ninth", "tenth",
        "eleventh", "twelfth"
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "Two Turtle Doves",
        "Three French Hens",
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a-Laying",
        "Seven Swans a-Swimming",
        "Eight Maids a-Milking",
        "Nine Ladies Dancing",
        "Ten Lords a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming"
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me:", days[day]);
        
        for i in (0..day + 1).rev() {
            if day == 0 && i == 0 {
                println!("{}", gifts[i]);
            } 
            
            else if i == 0 {
                println!("And {}", gifts[i]);
            } 
            
            else {
                println!("{}", gifts[i]);
            }
        }
        
        println!();
    }
}
