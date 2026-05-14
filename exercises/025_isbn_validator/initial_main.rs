// ISBN VALIDATOR
// Validate ISBN-10 and ISBN-13 book numbers using checksum algorithms
// ISBN-10: Sum of (digit × position) % 11 == check digit
// ISBN-13: Alternating 1×, 3× multipliers, sum % 10 == 0
// Examples: 0-306-40615-2 (valid ISBN-10), 978-0-306-40615-7 (valid ISBN-13)

fn main() {
    let test_isbns = vec![
        "0-306-40615-2",     // Valid ISBN-10
        "0-19-853453-1",     // Valid ISBN-10
        "0-123-45678-9",     // Invalid ISBN-10
        "978-0-306-40615-7", // Valid ISBN-13
        "979-10-90636-07-1", // Valid ISBN-13
        "978-0-123-45678-0", // Invalid ISBN-13
    ];

    println!("=== ISBN Validator ===\n");

    for isbn in test_isbns {
        let is_valid = validate_isbn(isbn);
        println!(
            "{}: {}",
            isbn,
            if is_valid { "✓ Valid" } else { "✗ Invalid" }
        );
    }
}

/// Validate ISBN (detects type by length and validates accordingly)
pub fn validate_isbn(isbn: &str) -> bool {
    todo!("Detect ISBN type (10 or 13) and call appropriate validation function")
}

/// Validate ISBN-10 using checksum algorithm
pub fn validate_isbn10(isbn: &str) -> bool {
    todo!("Remove hyphens, validate length, compute checksum: sum(digit × position) % 11")
}

/// Validate ISBN-13 using alternating multiplier algorithm
pub fn validate_isbn13(isbn: &str) -> bool {
    todo!("Remove hyphens, validate length, compute checksum: alternate 1× and 3×, sum % 10 == 0")
}
