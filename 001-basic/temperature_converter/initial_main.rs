// CONVERSOR DE TEMPERATURA
// Este programa converte temperaturas entre Fahrenheit e Celsius.
// - Fahrenheit para Celsius: (F - 32) * 5/9
// - Celsius para Fahrenheit: (C * 9/5) + 32
// Implemente uma função para cada direção de conversão.

use std::io;

fn main() {
    // Esta função não exige nenhuma edição
    println!("Conversor de Temperatura");
    println!("Escolha 1 - Fahrenheit para Celsius");
    println!("Escolha 2 - Celsius para Fahrenheit");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    let input: u32 = input.trim().parse().expect("Por favor, digite um número!");

    if input == 1 {
        println!("Digite os graus Fahrenheit a serem convertidos");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        let input: u32 = input.trim().parse().expect("Por favor, digite um número!");

        print!(
            "{input} Fahrenheit equivale a {:.2} Celsius",
            fahrenheit_to_celsius(input as f64)
        )
    } else if input == 2 {
        println!("Digite os graus Celsius a serem convertidos");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        let input: u32 = input.trim().parse().expect("Por favor, digite um número!");

        print!(
            "{input} Celsius equivale a {:.2} Fahrenheit",
            celsius_to_fahrenheit(input as f64)
        )
    } else {
        println!("Você digitou {input}, que é diferente de 1 ou 2. Vamos começar de novo.");
    }
}

/// Converter Fahrenheit para Celsius
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    todo!(
        "Implemente como uma expressão de linha única, priorizando a precisão de ponto flutuante."
    )
}

/// Convertes Celsius para Fahrenheit
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!(
        "Implemente como uma expressão de linha única, priorizando a precisão de ponto flutuante."
    )
}
