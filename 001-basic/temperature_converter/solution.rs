// First solution. Not optimized.

use std::io;
fn main() {
    // Your code here: prompt user for temperature and conversion direction
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
            "{input} Fahrenheit equals to {} Celsius",
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
            "{input} Celsius equals to {} Fahrenheit",
            celsius_to_fahrenheit(input as f64)
        )
    } else {
        println!("You typed {input}, which is different from 1 or 2. Let's start again.");
    }
}

// ============================================================
// VERSÕES DE CONVERSÃO DE TEMPERATURA
// ============================================================

// 1º Lugar: Expressão Direta (Minimalista e Idiomática)
// Quando usar: Código simples, conversões ocasionais. Vantagens: Concisa, clara, idiomática Rust. A expressão final é retornada implicitamente. Sem variáveis intermediárias desnecessárias. Perfeita para a maioria dos casos.
/// Convert Fahrenheit to Celsius
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Convert Celsius to Fahrenheit
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

// 2º Lugar: Com Constantes (Otimização para Hot Loops)
// Quando usar: Milhões de conversões em loop. Vantagens: Ratios pré-calculados (5.0/9.0 e 9.0/5.0 computados em compile-time). Evita divisões repetidas. Nomes descritivos melhoram legibilidade. Só vale a pena se performance for crítica.
const FAHRENHEIT_OFFSET: f64 = 32.0;
const F_TO_C_RATIO: f64 = 5.0 / 9.0;
const C_TO_F_RATIO: f64 = 9.0 / 5.0;

pub fn fahrenheit_to_celsius_opt(f: f64) -> f64 {
    (f - FAHRENHEIT_OFFSET) * F_TO_C_RATIO
}

pub fn celsius_to_fahrenheit_opt(c: f64) -> f64 {
    c * C_TO_F_RATIO + FAHRENHEIT_OFFSET
}

// 3º Lugar: Usando 1.8 (Direto e Matemático)
// Quando usar: Preferência por divisão vs multiplicação, ou quando 1.8 é mais familiar (9/5 = 1.8). Vantagens: Divisão é mais intuitiva que multiplicação para alguns. Ligeiramente mais eficiente (1 operação a menos). Desvantagens: Menos explícito que a fração 9/5.
pub fn fahrenheit_to_celsius_alt(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

pub fn celsius_to_fahrenheit_alt(c: f64) -> f64 {
    c * 1.8 + 32.0
}
