// DUPLICATE REMOVER
// This program removes duplicate elements from a vector while preserving order.
// Only the first occurrence of each element is kept.
// Example: [1, 2, 2, 3, 1, 4] → [1, 2, 3, 4]

use std::collections::HashSet;

fn main() {
    let test_cases = vec![
        vec![1, 2, 2, 3, 1, 4],
        vec![5, 5, 5, 5],
        vec![1, 2, 3, 4, 5],
        vec![10, 20, 10, 30, 20, 40],
    ];
    
    for numbers in test_cases {
        let unique = remove_duplicates(&numbers);
        println!("{:?} → {:?}", numbers, unique);
    }
}

/// Remove duplicates from a slice while preserving order
pub fn remove_duplicates(numbers: &[i32]) -> Vec<i32> {
    // Your implementation here
    vec![]
}
