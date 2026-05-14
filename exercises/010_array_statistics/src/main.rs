// ARRAY STATISTICS
// This program calculates statistical measures for a vector of numbers:
// - Mean (average): sum of all values divided by count
// - Median: middle value when sorted
// - Mode: most frequently occurring value
// - Variance: average of squared differences from mean

fn main() {
    let numbers = vec![4.5, 2.3, 4.5, 7.8, 4.5, 3.2, 6.1];

    let stats = calculate_statistics(&numbers);
    println!("Statistics for {:?}", numbers);
    println!("  Mean: {:.2}", stats.mean);
    println!("  Median: {:.2}", stats.median);
    println!("  Mode: {:.2}", stats.mode);
    println!("  Variance: {:.2}", stats.variance);
}

/// Struct to hold statistical results
pub struct Statistics {
    pub mean: f64,
    pub median: f64,
    pub mode: f64,
    pub variance: f64,
}

/// Calculate mean, median, mode, and variance for a slice of numbers
pub fn calculate_statistics(numbers: &[f64]) -> Statistics {
    // Your implementation here
    Statistics {
        mean: 0.0,
        median: 0.0,
        mode: 0.0,
        variance: 0.0,
    }
}
