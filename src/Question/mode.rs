use std::collections::HashMap;

fn main() {
    let mode = [1, 22, 32, 32, 5, 6, 20];

    let mut map = HashMap::new();

    for &word in &mode {
        let count_map = map.entry(word).or_insert(0);
        *count_map += 1;
    }

    let mut max_count = 0;
    let mut mode_value = 0;

    for (&value, &count) in &map {
        if count > max_count {
            max_count = count;
            mode_value = value;
        }
    }

    println!("{:?}", mode_value);
}
