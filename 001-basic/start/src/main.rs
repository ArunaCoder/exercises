// RUST BASICS - Guia de Referência Rápida
// Este arquivo contém os fundamentos básicos de Rust que você precisará
// para completar os exercícios. Volte aqui sempre que precisar relembrar a sintaxe!
//
// Para executar: cargo run --example rust_basics

fn tipos() {
    // ==========================================================================
    // TIPOS ESCALARES (Representam um único valor)
    // ==========================================================================

    // Inteiros com sinal (Signed): i8, i16, i32, i64, i128, isize
    // Armazenam valores negativos e positivos usando representação de complemento de dois.
    // 'isize' depende da arquitetura do target (32 ou 64 bits), ideal para indexação.

    // O prefixo "_" suprime avisos de variável não utilizada.
    // Em produção, é usado legitimamente em alguns casos.
    let _inteiro_sinal: i32 = -42;

    // Inteiros sem sinal (Unsigned): u8, u16, u32, u64, u128, usize
    // Úteis para contagens, tamanhos de memória e dados binários brutos.
    // 'usize' é o tipo padrão para tamanhos de coleções e índices de arrays.
    let _inteiro_sem_sinal: u32 = 100;

    // Ponto Flutuante (Floats): f32 (precisão simples), f64 (precisão dupla)
    // Seguem o padrão IEEE 754. f64 é o padrão moderno devido à precisão/performance.
    let _ponto_flutuante: f64 = 3.14159265;

    // Booleano (bool): true ou false
    // Ocupa 1 byte na memória (8 bits), embora precise apenas de 1 bit para a informação.
    let _ativo: bool = true;

    // Caractere (char): Representa um Valor Escalar Unicode (4 bytes).
    // Diferente de C/C++, um char em Rust não é apenas um byte ASCII.
    let _letra: char = 'R';

    // ==========================================================================
    // TIPOS COMPOSTOS (Agrupam múltiplos valores em um tipo)
    // ==========================================================================

    // Tuplas (tuple): Agrupamento de tipos heterogêneos com tamanho fixo.
    // Úteis para retornar múltiplos valores de uma função sem criar uma struct.
    let tupla: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, _y, _z) = tupla; // Destructuring para acesso rápido

    // Arrays: Coleção homogênea de tamanho fixo conhecido em tempo de compilação.
    // Alocados na Stack (pilha). A assinatura inclui o tipo e o tamanho: [T; N].
    let matriz: [i32; 3] = [1, 2, 3];

    // ==========================================================================
    // TIPOS ESPECIAIS E PONTEIROS
    // ==========================================================================

    // Slice (&[T]): Uma visão (view) para uma sequência de elementos num array ou vetor.
    // É um "fat pointer" (contém o endereço de memória e o comprimento).
    let _slice: &[i32] = &matriz[1..3];

    // Unit Type (): Representa a ausência de valor ou de retorno.
    // Equivalente ao 'void' em linguagens como C ou Java, mas é um tipo real de tamanho zero (ZST).
    fn _retorna_nada() -> () {}

    // Never Type (!): Indica que uma função nunca retorna (ex: panic! ou loops infinitos).
    // Estabilizado e refinado na Edition 2024 para melhor interoperabilidade.
    // fn loop_infinito() -> ! { loop {} }
}

// ==================== EXEMPLOS DE FUNÇÕES ====================
// (Definidas aqui para você ver como se escreve funções)

// Função sem retorno (não retorna nada)
fn saudar() {
    println!("Olá, mundo!");
}

// 'fn' é a palavra-chave reservada da linguagem Rust para declarar uma função.
// 'somar_explicito' é o nome da função, definido livremente por quem escreve o código.
// Os valores entre parênteses (a: u32, b: u32) são os parâmetros: dados que a função recebe para processar.
// O operador '->' indica o tipo de dado que a função obrigatoriamente devolverá.
fn somar_explicito(a: u32, b: u32) -> u32 {
    return a + b; // O 'return' encerra a função e envia o resultado de volta para quem a chamou.
}

// Em Rust, esta é a forma idiomática (padrão preferido pela comunidade).
// Note a ausência da palavra-chave 'return' e do ponto-e-vírgula no final.
fn somar_implicito(a: u32, b: u32) -> u32 {
    // Por não ter ponto-e-vírgula, esta linha é uma "expressão de cauda" (tail expression).
    // O Rust entende que o resultado desta expressão deve ser o valor de retorno da função.
    a + b
}

// Função que chama outra função
fn calcular_dobro(n: u32) -> u32 {
    somar_implicito(n, n)
}

// Função com múltiplos parâmetros e tipos diferentes
fn exibir_info(nome: &str, idade: u32, altura: f64) {
    println!("Nome: {}, Idade: {}, Altura: {:.2}m", nome, idade, altura);
}

// ==================== FUNÇÃO PRINCIPAL ====================

fn main() {
    tipos();
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
