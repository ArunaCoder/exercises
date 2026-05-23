// TEMPERATURE CONVERTER
// This program converts temperatures between Fahrenheit and Celsius.
// - Fahrenheit to Celsius: (F - 32) * 5/9
// - Celsius to Fahrenheit: (C * 9/5) + 32
// Implement one function for each conversion direction.

fn main() {
    println!("Temperature Converter");
    println!("Choose 1 -  Fahrenheit to Celsius");
    println!("Choose 2 - Celsius to Fahrenheit");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please type a number!");

    if input == 1 {
        println!("Enter the Fahrenheit degree to be converted");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = input.trim().parse().expect("Please type a number!");

        print!(
            "{input} Fahrenheit equals to {:.2} Celsius",
            fahrenheit_to_celsius(input as f64)
        )
    } else if input == 2 {
        println!("Enter the Celcius degree to be converted");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input: u32 = input.trim().parse().expect("Please type a number!");

        print!(
            "{input} Celsius equals to {:.2} Fahrenheit",
            celsius_to_fahrenheit(input as f64)
        )
    } else {
        println!("You typed {input}, which is different from 1 or 2. Let's start again.");
    }
}

/// Convert Fahrenheit to Celsius
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // Implement: (F - 32) * 5/9
    0.0
}

/// Convert Celsius to Fahrenheit
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    // Implement: (C * 9/5) + 32
    0.0
}
