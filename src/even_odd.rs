fn even_or_odd(i: i32) -> &'static str {
    match i % 2 {
      0 => "Even",
      _ => "Odd",
    }
  }


  // another solution

  fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {
        return "Even";
    } else {
        return "Odd";
    }
}

//Get Nth Even Number

fn nth_even(n: u32) -> u32 {
    if n == 1 {
        return 0;
    } else {
        return (n - 1) * 2;
    }
}

fn nth_even(n: u32) -> u32 {
    (n - 1) * 2
}
