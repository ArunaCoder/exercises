// BINARY TO DECIMAL CONVERTER
// Convert between binary (base 2) and decimal (base 10) number systems
// Binary uses only digits 0 and 1
// Examples: "1010" → 10, 42 → "101010"

fn main() {
    println!("=== Binary to Decimal Converter ===\n");

    // Binary to Decimal
    let binaries = vec!["1010", "1111", "10000", "101010"];
    println!("Binary → Decimal:");
    for bin in binaries {
        match binary_to_decimal(bin) {
            Ok(dec) => println!("{} → {}", bin, dec),
            Err(e) => println!("{} → Error: {}", bin, e),
        }
    }

    // Decimal to Binary
    let decimals = vec![10, 15, 16, 42, 255];
    println!("\nDecimal → Binary:");
    for dec in decimals {
        let bin = decimal_to_binary(dec);
        println!("{} → {}", dec, bin);
    }
}

/// Convert binary string to decimal number
pub fn binary_to_decimal(binary: &str) -> Result<u32, String> {
    todo!("Validate input, iterate digits, calculate sum of digit × 2^position")
}

/// Convert decimal number to binary string
pub fn decimal_to_binary(decimal: u32) -> String {
    todo!("Repeatedly divide by 2, collect remainders, reverse")
}
