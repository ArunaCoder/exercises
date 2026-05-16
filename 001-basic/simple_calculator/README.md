# Simple Calculator

Source: Exercise 11 from Rust Basic Exercises

### Objective

Build a basic calculator that can perform addition, subtraction, multiplication, and division. Parse a string expression like "5 + 3" or "10 / 2" and return the result. Use an enum to represent operations and pattern matching to execute the correct operation. Handle division by zero appropriately.

### Step-by-Step

1. [ ] Define an enum `Operation` with variants: Add, Subtract, Multiply, Divide
2. [ ] Create a function `parse_expression(expr: &str) -> Option<(f64, Operation, f64)>` to parse "5 + 3" format
3. [ ] Split the expression by whitespace to extract: number1, operator, number2
4. [ ] Match the operator string ("+", "-", "*", "/") to the appropriate Operation enum variant
5. [ ] Create a function `calculate(num1: f64, op: Operation, num2: f64) -> Result<f64, String>` to perform the calculation
6. [ ] Use pattern matching on the Operation enum to execute the correct arithmetic
7. [ ] Return an error for division by zero instead of panicking

### How to test

Run the following command in your terminal:
`cargo test -p simple_calculator`

Or run the program:
`cargo run -p simple_calculator`
