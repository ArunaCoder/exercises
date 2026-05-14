// MEDIAN AND MODE
// This program processes a list of integers to find two statistical measures:
// 1. The Median: The middle value of a list sorted in ascending order (if n is odd) or the arithmetic mean of the two middle values (if n is even).
// 2. The Mode: The value that appears most frequently using a HashMap.
// Use this Vec: [42, 1, 3, 42, 10]

fn main() {
    let mut my_vec = vec![42, 1, 3, 42, 10];
    let (median, mode) = solve(my_vec);

    println!("The median is {median}, and the mode is {mode}!")
}

/// Your implementation:
pub fn solve(vec: &mut Vec<i32>) -> (i32, i32) {
    vec.sort();

    let len = &vec.len();

    match len {
       % 2 = 0 {let median = 0}
    }

    println!("{len}");
    (0, 0)
}
