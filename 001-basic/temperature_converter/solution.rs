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

// 1º Lugar: Expressão Direta (Minimalista e Idiomática)
// Quando usar: Código simples, conversões ocasionais. Vantagens: Concisa, clara, idiomática Rust. A expressão final é retornada implicitamente. Sem variáveis intermediárias desnecessárias. Perfeita para a maioria dos casos.

pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

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
