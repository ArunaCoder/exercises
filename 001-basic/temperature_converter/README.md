# Temperature Converter

Source: Exercise 04 from Rust Basic Exercises

### Objective

Create a command-line program that converts temperatures between Fahrenheit and Celsius. Accept user input specifying the temperature value and the direction of conversion (F to C or C to F). Use functions for each conversion formula and display the result with appropriate formatting.

### Step-by-Step

1. [ ] Create a function `fahrenheit_to_celsius(f: f64) -> f64` that converts Fahrenheit to Celsius using the formula: (F - 32) \* 5/9
2. [ ] Create a function `celsius_to_fahrenheit(c: f64) -> f64` that converts Celsius to Fahrenheit using the formula: (C \* 9/5) + 32
3. [ ] In the `main()` function, prompt the user to enter a temperature value
4. [ ] Ask the user to specify the conversion direction (1 for F→C, 2 for C→F)
5. [ ] Call the appropriate conversion function and display the result with 2 decimal places

### How to test

Run the following command in your terminal:
`cargo test -p temperature_converter`

Or run the program interactively:
`cargo run -p temperature_converter`
