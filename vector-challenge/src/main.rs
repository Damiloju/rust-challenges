use std::collections::HashMap;

fn main() {
    let mut vector: Vec<i32> = vec![1, 2, 3, 4, 1, 23, 8, 5, 6, 4, 3, 2];
    vector.sort();
    let mut total_value: i32 = 0;
    let size: i32 = vector.len() as i32;
    let median: usize = (size / 2) as usize;

    for x in &vector {
        total_value += x;
    }
    println!("The mean is {}", (total_value / size));
    println!("The median is {}", vector[median]);

    let mut map = HashMap::new();

    for x in &vector {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    }

    let mut count = 0;
    let mut mode = 0;

    for (key, value) in map {
        if key > &count {
            count = *key as i32;
            mode = value as i32;
        }
    }

    println!("The mode is {}", mode);
}
