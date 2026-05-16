# Unit Converter

Source: Exercise 23 from Rust Basic Exercises

### Objective

Build a unit converter that can convert between different units of measurement such as meters, kilometers, miles, inches, feet, etc. Define an enum for unit types and use pattern matching to determine which conversion formula to apply. Accept a value (f64), source unit, and target unit, then return the converted value.

### Step-by-Step

1. [ ] Define an enum `LengthUnit` with variants: `Meters`, `Kilometers`, `Miles`, `Inches`, `Feet`
2. [ ] Create a function `convert(value: f64, from: LengthUnit, to: LengthUnit) -> f64`
3. [ ] First, convert the input value to a common base unit (e.g., meters)
4. [ ] Then convert from the base unit to the target unit
5. [ ] Use pattern matching to select the appropriate conversion factors
6. [ ] Conversion factors: 1 km = 1000 m, 1 mile = 1609.34 m, 1 foot = 0.3048 m, 1 inch = 0.0254 m
7. [ ] **Bonus**: Extend to support temperature (Celsius, Fahrenheit, Kelvin) or weight units

### How to test

Run the following command in your terminal:
`cargo test -p unit_converter`

Or run the program:
`cargo run -p unit_converter`
