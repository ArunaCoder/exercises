// COLLATZ CONJECTURE (3n+1 Problem)
// Generate sequence: if even divide by 2, if odd multiply by 3 and add 1
// Continue until reaching 1
// Example: 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

fn main() {
    let test_numbers = vec![1, 5, 13, 27, 97];

    println!("=== Collatz Conjecture ===\n");

    for n in test_numbers {
        let sequence = collatz_sequence(n);
        println!("Starting with {}: {} steps", n, sequence.len() - 1);
        println!("Sequence: {:?}\n", sequence);
    }

    // Bonus: find number below N with longest sequence
    let limit = 100;
    let (number, length) = longest_collatz_sequence(limit);
    println!("Number below {} with longest sequence: {} (length: {})",
        limit, number, length);
}

/// Generate the Collatz sequence starting from n
pub fn collatz_sequence(n: u64) -> Vec<u64> {
    todo!("Loop: if even n/2, if odd 3n+1, continue until 1, collect all values")
}

/// Find the starting number below limit that produces the longest sequence
pub fn longest_collatz_sequence(limit: u64) -> (u64, usize) {
    todo!("Test each number from 1 to limit, track which produces longest sequence")
}
