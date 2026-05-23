// LEAP YEAR CHECKER
// Determine if a year is a leap year using Gregorian calendar rules:
// - Divisible by 4: leap year
// - Except if divisible by 100: not a leap year
// - Unless also divisible by 400: leap year
// Examples: 2000 (yes), 1900 (no), 2024 (yes), 2023 (no)

fn main() {
    let test_years = vec![2000, 1900, 2024, 2023, 2100, 2400, 1984, 2001];

    println!("Leap Year Checker:\n");

    for year in test_years {
        let is_leap = is_leap_year(year);
        println!(
            "{}: {}",
            year,
            if is_leap {
                "Leap Year ✓"
            } else {
                "Not Leap Year ✗"
            }
        );
    }
}

/// Check if a year is a leap year
pub fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
