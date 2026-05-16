// SOLUTION: MEDIAN AND MODE

// main function:
// let mut numbers = vec![42, 1, 3, 42, 10]; ...
// match find_median(&mut numbers) { ...

/// This is the ideal implementation for instructor reference.
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
