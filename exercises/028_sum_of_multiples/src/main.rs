// SUM OF MULTIPLES
// Calculate sum of all numbers below N that are multiples of 3 or 5
// Example: N=10 → multiples are 3, 5, 6, 9 → sum = 23
// Example: N=1000 → sum = 233168

fn main() {
    let test_values = vec![10, 20, 100, 1000];

    println!("=== Sum of Multiples (3 or 5) ===\n");

    for n in test_values {
        let sum = sum_of_multiples(n);
        println!("Sum of multiples below {}: {}", n, sum);
    }

    // Bonus: generalized version
    println!("\n=== Generalized Sum of Multiples ===\n");
    println!("Multiples of 2 or 3 below 20: {}", sum_of_multiples_generic(20, &vec![2, 3]));
    println!("Multiples of 7 or 11 below 100: {}", sum_of_multiples_generic(100, &vec![7, 11]));
}

/// Sum of all multiples of 3 or 5 below n
pub fn sum_of_multiples(n: u32) -> u32 {
    todo!("Loop from 1 to n-1, check if divisible by 3 or 5, accumulate sum")
}

/// Generalized: sum of multiples of any numbers in the divisors list
pub fn sum_of_multiples_generic(n: u32, divisors: &[u32]) -> u32 {
    todo!("Loop and check if divisible by any divisor in the list")
}
