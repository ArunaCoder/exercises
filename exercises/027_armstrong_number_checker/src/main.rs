// ARMSTRONG NUMBER CHECKER
// Armstrong number: sum of digits raised to power of digit count equals number
// Examples: 153 (1³+5³+3³=153), 9474 (9⁴+4⁴+7⁴+4⁴=9474), 1 (1¹=1)
// Non-examples: 123, 10, 100

fn main() {
    let test_numbers = vec![0, 1, 9, 153, 370, 371, 407, 123, 9474, 9475, 54748];

    println!("=== Armstrong Number Checker ===\n");

    for num in test_numbers {
        let is_armstrong = is_armstrong_number(num);
        println!("{}: {}", num, if is_armstrong { "✓ Armstrong" } else { "✗ Not Armstrong" });
    }

    // Bonus: find all Armstrong numbers up to 10000
    println!("\nArmstrong numbers up to 10000:");
    let armstrong_list = find_armstrong_numbers(10000);
    println!("{:?}", armstrong_list);
}

/// Check if a number is an Armstrong number
pub fn is_armstrong_number(n: u32) -> bool {
    todo!("Count digits, extract each digit, sum digit^count, compare with n")
}

/// Find all Armstrong numbers up to limit
pub fn find_armstrong_numbers(limit: u32) -> Vec<u32> {
    todo!("Test each number from 0 to limit and collect Armstrong numbers")
}
