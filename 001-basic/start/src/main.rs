// RUST BASICS - Guia de Referência Rápida
// Este arquivo contém os fundamentos básicos de Rust que você precisará
// para completar os exercícios. Volte aqui sempre que precisar relembrar a sintaxe!
//
// Para executar: cargo run --example rust_basics

// ==================== EXEMPLOS DE FUNÇÕES ====================
// (Definidas aqui para você ver como se escreve funções)

/// Função sem retorno (não retorna nada)
fn saudar() {
    println!("Olá, mundo!");
}

/// Função com return explícito
/// Retorna a soma de dois números
fn somar_explicito(a: u32, b: u32) -> u32 {
    return a + b; // return explícito com ponto-e-vírgula
}

/// Função com return implícito (idiomático em Rust)
/// Retorna a soma de dois números
fn somar_implicito(a: u32, b: u32) -> u32 {
    a + b // ⚠️ SEM ponto-e-vírgula! Expressão retorna automaticamente
}

/// Função que chama outra função
fn calcular_dobro(n: u32) -> u32 {
    somar_implicito(n, n)
}

/// Função com múltiplos parâmetros e tipos diferentes
fn exibir_info(nome: &str, idade: u32, altura: f64) {
    println!("Nome: {}, Idade: {}, Altura: {:.2}m", nome, idade, altura);
}

// ==================== FUNÇÃO PRINCIPAL ====================

fn main() {
    println!("=== RUST BASICS - Guia de Referência ===\n");

    // ==================== VARIÁVEIS ====================
    println!("--- 1. VARIÁVEIS ---");

    // Variável imutável (padrão)
    let x = 5;
    println!("x (imutável) = {}", x);
    // x = 6; // ❌ ERRO! Não pode mudar variável imutável

    // Variável mutável
    let mut y = 10;
    println!("y (mutável) = {}", y);
    y = 15; // ✅ OK! Pode mudar variável mutável
    println!("y depois de mudar = {}", y);

    // Shadowing (criar nova variável com mesmo nome)
    let z = 5;
    let z = z + 1; // ✅ OK! Cria nova variável z
    println!("z (shadowing) = {}", z);

    // ==================== TIPOS ====================
    println!("\n--- 2. TIPOS BÁSICOS ---");

    let inteiro: i32 = -42; // inteiro com sinal (32 bits)
    let inteiro_positivo: u32 = 42; // inteiro sem sinal (32 bits)
    let inteiro_grande: u64 = 1000000; // inteiro sem sinal (64 bits)
    let decimal: f64 = 3.14159; // número decimal
    let booleano: bool = true; // verdadeiro ou falso
    let caractere: char = '🦀'; // caractere único
    let texto: &str = "Hello, Rust!"; // string literal

    println!("i32: {}", inteiro);
    println!("u32: {}", inteiro_positivo);
    println!("u64: {}", inteiro_grande);
    println!("f64: {}", decimal);
    println!("bool: {}", booleano);
    println!("char: {}", caractere);
    println!("&str: {}", texto);

    // ==================== OPERAÇÕES ARITMÉTICAS ====================
    println!("\n--- 3. OPERAÇÕES ARITMÉTICAS ---");

    let a = 10;
    let b = 3;

    println!("{} + {} = {}", a, b, a + b); // adição
    println!("{} - {} = {}", a, b, a - b); // subtração
    println!("{} * {} = {}", a, b, a * b); // multiplicação
    println!("{} / {} = {}", a, b, a / b); // divisão (inteira)
    println!("{} % {} = {}", a, b, a % b); // resto (módulo)

    // ==================== CONVERSÃO DE TIPOS ====================
    println!("\n--- 4. CONVERSÃO DE TIPOS ---");

    let numero_inteiro: u32 = 42;
    let numero_decimal: f64 = numero_inteiro as f64; // converte u32 para f64
    println!("u32: {} → f64: {}", numero_inteiro, numero_decimal);

    let grande: i64 = 1000;
    let pequeno: i32 = grande as i32; // converte i64 para i32
    println!("i64: {} → i32: {}", grande, pequeno);

    // ==================== FUNÇÕES ====================
    println!("\n--- 5. FUNÇÕES ---");

    // Função sem retorno
    saudar();

    // Função com return explícito
    let soma1 = somar_explicito(5, 3);
    println!("somar_explicito(5, 3) = {}", soma1);

    // Função com return implícito (idiomático em Rust)
    let soma2 = somar_implicito(10, 7);
    println!("somar_implicito(10, 7) = {}", soma2);

    // Função que chama outra função
    let resultado = calcular_dobro(5);
    println!("calcular_dobro(5) = {}", resultado);

    // Função com múltiplos parâmetros e tipos diferentes
    exibir_info("Jonas", 45, 1.88);

    // ==================== FORMATAÇÃO COM println! ====================
    println!("\n--- 6. FORMATAÇÃO COM println! ---");

    let nome = "Ferris";
    let idade = 13;
    let altura = 1.75;

    // Texto simples
    println!("Texto simples");

    // Com uma variável
    println!("Nome: {}", nome);

    // Com múltiplas variáveis
    println!("{} tem {} anos", nome, idade);

    // Com formatação de decimais
    println!("Altura: {:.2} metros", altura);

    // Com nomes explícitos
    println!("Dados: nome={nome}, idade={idade}");

    // Debug print (mostra estrutura completa)
    println!("Debug: {:?}", (nome, idade, altura));

    // ==================== COMPARAÇÕES ====================
    println!("\n--- 7. COMPARAÇÕES ---");

    let x = 5;
    let y = 10;

    println!("{} == {} ? {}", x, y, x == y); // igual
    println!("{} != {} ? {}", x, y, x != y); // diferente
    println!("{} < {} ? {}", x, y, x < y); // menor que
    println!("{} > {} ? {}", x, y, x > y); // maior que
    println!("{} <= {} ? {}", x, y, x <= y); // menor ou igual
    println!("{} >= {} ? {}", x, y, x >= y); // maior ou igual

    // ==================== CONDICIONAIS ====================
    println!("\n--- 8. CONDICIONAIS (if/else) ---");

    let numero = 7;

    if numero > 10 {
        println!("{} é maior que 10", numero);
    } else if numero > 5 {
        println!("{} está entre 6 e 10", numero);
    } else {
        println!("{} é 5 ou menor", numero);
    }

    // if como expressão (retorna valor)
    let mensagem = if numero % 2 == 0 { "par" } else { "ímpar" };
    println!("{} é {}", numero, mensagem);

    // =================se este arquivo como referência sempre que precisar!");
}

// ==================== DICAS IMPORTANTES ====================

/*
PONTOS DE ATENÇÃO:

1. PONTO-E-VÍRGULA:
   - Com ';' → statement (não retorna valor)
   - Sem ';' → expressão (retorna valor)

   fn exemplo() -> u32 {
       5 + 3  // retorna 8
   }

   fn exemplo2() -> u32 {
       5 + 3;  // ❌ ERRO! Não retorna nada
   }

2. MUTABILIDADE:
   - Variáveis são imutáveis por padrão
   - Use 'mut' para torná-las mutáveis
   - Shadowing permite "recriar" variável

3. TIPOS:
   - Rust é estaticamente tipado
   - Inferência automática na maioria dos casos
   - Use anotações quando necessário: let x: u32 = 5;

4. NAMING CONVENTIONS:
   - Variáveis e funções: snake_case (minúsculas_com_underline)
   - Constantes: UPPER_CASE
   - Tipos (structs, enums): PascalCase

5. CONVERSÕES:
   - Use 'as' para conversões numéricas
   - Para strings, use métodos como .to_string() ou .parse()
*/
