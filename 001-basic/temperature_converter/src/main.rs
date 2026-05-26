// CONVERSOR DE TEMPERATURA
// Este programa converte temperaturas entre Fahrenheit e Celsius.
// - Fahrenheit para Celsius: (F - 32) * 5/9
// - Celsius para Fahrenheit: (C * 9/5) + 32

// Instruções para o compilador e formatador ignorarem avisos de código incompleto ou estilo.
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(warnings)]

use std::io;

fn main() {
    loop {
        println!("Conversor de Temperatura");
        println!("Escolha 1 - Fahrenheit para Celsius");
        println!("Escolha 2 - Celsius para Fahrenheit");

        // 'let' cria uma variável; 'mut' permite que o valor dela seja alterado depois.
        // 'String::new()' cria um texto vazio expansível para armazenar o que o usuário digitar.
        let mut input = String::new();

        // 'io::stdin()' acessa a entrada padrão (teclado).
        // '.read_line()' pausa o programa para ler o que foi digitado e guarda na variável informada.
        // '.expect()' define uma mensagem de erro caso algo falhe na leitura.
        io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

        // Reutilizamos o nome 'input' (Shadowing).
        // '.trim()' remove espaços e o "Enter" (\n); '.parse()' tenta converter o texto para o tipo 'u32'.
        let input: u32 = input.trim().parse().expect("Por favor, digite um número!\n\n");

        if input == 1 {
            println!("Digite os graus Fahrenheit a serem convertidos");

            // TODO: Repita a lógica de leitura do teclado (stdin) para capturar o valor da temperatura na variável 'input'.


            // TODO: Use shadowing para converter 'input' em u32: limpe os espaços, faça o parse e trate possíveis erros.


            // TODO Faltou usar 'String::new()' para criar input como texto vazio expansível antes de io::stdin

            // 'input as f64' converte o número inteiro para ponto flutuante (decimal) para permitir o cálculo.
            print!(
                "{input} Fahrenheit equivale a {:.2} Celsius\n",
                fahrenheit_to_celsius(input as f64)
            );
            break;
        } else if input == 2 {
            println!("Digite os graus Celsius a serem convertidos");

            let mut input = String::new();

            io::stdin()
            .read_line(&mut input)
                .expect("Falha ao ler a linha");

            let input: u32 = input.trim().parse().expect("Por favor, digite um número!");

            print!(
                "{input} Celsius equivale a {:.2} Fahrenheit\n",
                celsius_to_fahrenheit(input as f64)
            );
            break;
        } else {
            println!("Você digitou {input}, que é diferente de 1 ou 2. Vamos começar de novo.\n");
        }
    }
}

/// Converter Fahrenheit para Celsius
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // 'todo!' é uma macro que avisa ao compilador que o código ainda será escrito (causa erro se executado).
    todo!(
        "Implementar conversão como uma expressão de linha única, priorizando a precisão de ponto flutuante."
    )
}

/// Convertes Celsius para Fahrenheit
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    todo!(
        "Implementar conversão como uma expressão de linha única, priorizando a precisão de ponto flutuante."
    )
}
