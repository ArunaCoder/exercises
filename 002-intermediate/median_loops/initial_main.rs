// MEDIAN AND MODE
// This program processes a list of integers to find two statistical measures:
// 1. The Median: The middle value of a list sorted in ascending order (if n is odd) or the arithmetic mean of the two middle values (if n is even).
// 2. The Mode: The value that appears most frequently using a HashMap.
// Use this Vec: [42, 1, 3, 42, 10]

fn main() {
    let numbers = vec![42, 1, 3, 42, 10];

    println!("Original list: {:?}", numbers);

    match find_median(&numbers) {
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
pub fn find_median(numbers: &[i32]) -> Option<f64> {
    // Your implementation here
    todo!()
}

/// Find the mode (most frequent value) using a HashMap
/// Returns None if the list is empty
pub fn find_mode(numbers: &[i32]) -> Option<i32> {
    // Your implementation here
    todo!()
}
