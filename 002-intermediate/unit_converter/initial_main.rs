// UNIT CONVERTER
// Convert between different length units
// Supports: Meters, Kilometers, Miles, Inches, Feet
// Example: 5 miles → 8046.72 meters

fn main() {
    let conversions = vec![
        (5.0, LengthUnit::Miles, LengthUnit::Kilometers),
        (100.0, LengthUnit::Meters, LengthUnit::Feet),
        (12.0, LengthUnit::Inches, LengthUnit::Meters),
        (1.0, LengthUnit::Kilometers, LengthUnit::Miles),
    ];

    println!("=== Unit Converter ===\n");

    for (value, from, to) in conversions {
        let result = convert(value, from, to);
        println!("{} {:?} = {:.2} {:?}", value, from, result, to);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LengthUnit {
    Meters,
    Kilometers,
    Miles,
    Inches,
    Feet,
}

/// Convert a value from one unit to another
pub fn convert(value: f64, from: LengthUnit, to: LengthUnit) -> f64 {
    todo!("Convert value to base unit (meters), then to target unit using conversion factors")
}
