fn is_leap_year(year: i32) -> bool {
    match year {
        x if x % 400 == 0 => true,
        x if x % 100 == 0 => false,
        x if x % 4 == 0 => true,
        _ => false,
    }
}

//******************************** */

fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            return year % 400 == 0;
        } else {
            return true;
        }
    }
    false
}
