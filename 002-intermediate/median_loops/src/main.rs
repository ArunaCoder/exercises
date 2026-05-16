// MEDIAN AND MODE
// This program processes a list of integers to find two statistical measures:
// 1. The Median: The middle value of a list sorted in ascending order (if n is odd) or the arithmetic mean of the two middle values (if n is even).
// 2. The Mode: The value that appears most frequently using a HashMap.
// Use this Vec: [42, 1, 3, 42, 10]
// Expected median: 10; expected mode: 42

use std::collections::HashMap;

fn main() {
    let mut numbers = vec![42, 1, 3, 42, 10];

    println!("Original list: {:?}", numbers);

    match find_median(&mut numbers) {
        Some(median) => println!("Median: {}", median),
        None => println!("Cannot calculate median of empty list"),
    }

    match find_mode(&numbers) {
        Some(mode) => println!("Mode: {}", mode),
        None => println!("Cannot calculate mode of empty list"),
    }
}

/// Calculate the median of a list of integers
/// Returns None if the list is empty
pub fn find_median(numbers: &mut [i32]) -> Option<f64> {
    let len = numbers.len();
    if len == 0 {
        return None;
    }

    numbers.sort_unstable();

    let mid = len / 2;
    if len % 2 == 0 {
        Some((numbers[mid - 1] as f64 + numbers[mid] as f64) / 2.0)
    } else {
        Some(numbers[mid] as f64)
    }
}

/// Find the mode (most frequent value) using a HashMap
/// Returns None if the list is empty
pub fn find_mode(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let mut map = HashMap::new();

    for &num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    let mode = map
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(&num, _)| num);

    mode
}
