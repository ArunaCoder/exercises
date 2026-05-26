// EXIT LOOP
// Crie uma terceira saída com a escolha de 0 para sair do programa
// Remova o break das conversões par permitir ao usuário fazer diversas conversões e sair apenas quando quiser

use std::io;

fn main() {
    loop {
        println!("Conversor de Temperatura");
        //TODO inclua uma mensagem aqui com a nova opção
        println!("Escolha 0 - Para sair do programa\n");
        println!("Escolha 1 - Fahrenheit para Celsius");
        println!("Escolha 2 - Celsius para Fahrenheit");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        let input: u32 = input
            .trim()
            .parse()
            .expect("Por favor, digite um número!\n\n");

        //TODO inclua um braço if para a entrada 0 com break no final e remova os demais breaks
        if input == 0 {
            break;
        } else if input == 1 {
            println!("Digite os graus Fahrenheit a serem convertidos");

            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Falha ao ler a linha");

            let input: u32 = input
                .trim()
                .parse()
                .expect("Por favor, digite um número!!\n\n");

            print!(
                "{input} Fahrenheit equivale a {:.2} Celsius\n",
                fahrenheit_to_celsius(input as f64)
            );
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
        } else {
            println!("Você digitou {input}, que é diferente de 1 ou 2. Vamos começar de novo.\n");
        }
    }
}

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}
