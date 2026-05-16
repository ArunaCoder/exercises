# Array Statistics

Source: Exercise 10 from Rust Basic Exercises

### Objective

Calculate comprehensive statistics for a vector of floating-point numbers: mean (average), median, mode, and variance. Store numbers in a Vec<f64>. Sort for median calculation, use HashMap for mode counting, and apply statistical formulas for mean and variance.

### Step-by-Step

1. [ ] Create a function `calculate_statistics(numbers: &[f64]) -> Statistics` where Statistics is a struct
2. [ ] Define a struct `Statistics { mean: f64, median: f64, mode: f64, variance: f64 }`
3. [ ] **Mean**: Sum all numbers and divide by count
4. [ ] **Median**: Sort the numbers and take the middle value (or average of two middle values if even count)
5. [ ] **Mode**: Use a HashMap to count frequency of each number, find the most frequent
6. [ ] **Variance**: Calculate the average of squared differences from the mean: Σ(x - mean)² / n
7. [ ] Handle edge cases: empty vector, single element, all elements same

### How to test

Run the following command in your terminal:
`cargo test -p array_statistics`

Or run the program:
`cargo run -p array_statistics`
