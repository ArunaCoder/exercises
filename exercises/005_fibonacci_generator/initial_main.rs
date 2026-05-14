// FIBONACCI GENERATOR
// This program generates the first N numbers of the Fibonacci sequence.
// The sequence starts with 0, 1, and each subsequent number is the sum of the previous two.
// Example: For N=8, the sequence is: [0, 1, 1, 2, 3, 5, 8, 13]

fn main() {
    let n = 10;
    let fibonacci_sequence = generate_fibonacci(n);

    println!("First {} Fibonacci numbers: {:?}", n, fibonacci_sequence);
}

/// Generate the first N Fibonacci numbers
pub fn generate_fibonacci(n: usize) -> Vec<u64> {
    // Your implementation here
    vec![]
}
