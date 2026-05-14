// FIZZBUZZ
// This is the classic FizzBuzz problem.
// Print numbers from 1 to N, but:
// - For multiples of 3, print "Fizz"
// - For multiples of 5, print "Buzz"
// - For multiples of both 3 and 5, print "FizzBuzz"
// - Otherwise, print the number

fn main() {
    println!("FizzBuzz from 1 to 100:");
    fizzbuzz(100);
}

/// FizzBuzz from 1 to n
pub fn fizzbuzz(n: u32) {
    // Your implementation here
}

/// Alternative: return Vec<String> instead of printing
pub fn fizzbuzz_vec(n: u32) -> Vec<String> {
    // Your implementation here
    vec![]
}
