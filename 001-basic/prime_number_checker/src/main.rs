// PRIME NUMBER CHECKER
// This program checks if a number is prime and finds all prime numbers up to a given limit.
// A prime number is a natural number greater than 1 that has no positive divisors other than 1 and itself.
// Examples: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29...

fn main() {
    let number = 17;
    let limit = 50;

    println!("Is {} prime? {}", number, is_prime(number));
    println!("Prime numbers up to {}: {:?}", limit, find_primes_up_to(limit));
}

/// Check if a number is prime
pub fn is_prime(n: u32) -> bool {
    // Your implementation here
    false
}

/// Find all prime numbers up to a given limit
pub fn find_primes_up_to(limit: u32) -> Vec<u32> {
    // Your implementation here
    vec![]
}
