// LARGEST PRODUCT IN SERIES
// Find the largest product of N consecutive digits in a string of digits
// Example: "1234" with span 2 → max is 3×4 = 12
// Example: "63915" with span 3 → max is 9×1×5 = 45

fn main() {
    let test_cases = vec![
        ("1234", 2),
        ("63915", 3),
        ("73167176531330624919225119674426574742355349194934", 6),
        ("0123456789", 4),
    ];

    println!("=== Largest Product in Series ===\n");

    for (digits, span) in test_cases {
        match largest_product(digits, span) {
            Ok(product) => println!("'{}' (span {}): {}",
                &digits[..digits.len().min(20)], span, product),
            Err(e) => println!("'{}' (span {}): Error - {}",
                &digits[..digits.len().min(20)], span, e),
        }
    }
}

/// Find the largest product of `span` consecutive digits
pub fn largest_product(digits: &str, span: usize) -> Result<u64, String> {
    todo!("Convert to Vec<u32>, use windows(span), calculate products, track maximum")
}
