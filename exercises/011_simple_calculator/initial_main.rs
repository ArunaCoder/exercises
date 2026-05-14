// SIMPLE CALCULATOR
// This program performs basic arithmetic operations: +, -, *, /
// It parses expressions like "5 + 3" and returns the result.
// Use an enum for operations and pattern matching for execution.

fn main() {
    let expressions = vec!["10 + 5", "20 - 8", "6 * 7", "15 / 3", "10 / 0"];
    
    for expr in expressions {
        match calculate_expression(expr) {
            Ok(result) => println!("{} = {}", expr, result),
            Err(e) => println!("{} → Error: {}", expr, e),
        }
    }
}

/// Enum representing calculator operations
#[derive(Debug)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

/// Parse and calculate a simple arithmetic expression
/// Format: "number operator number" (e.g., "5 + 3")
pub fn calculate_expression(expr: &str) -> Result<f64, String> {
    // Your implementation here
    Err("Not implemented".to_string())
}
