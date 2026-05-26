// PRIMEIRO CONVERSOR DE TEMPERATURA

// Objetivo: Praticar a declaração de funções com retorno, operações aritméticas com tipos de ponto flutuante (f64) e vinculação de variáveis (let).

// Diretivas para o Editor de código não 'reclamar' do códio incompleto
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(warnings)]

// Converte Celsius para Fahrenheit
// Fórmula: (x * 9/5) + 32
// TODO: Inclua explicitamente nas funções de conversão o tipo de retorno esperado f64 através do operador '->'

fn celsius_para_fahrenheit(c: f64) {
    // TODO: Implemente como uma expressão de linha única, priorizando a precisão de ponto flutuante.
    // Lembre-se: Em Rust, o retorno idiomático não usa 'return' nem ';'

}

// Converte Celsius para Kelvin
// Fórmula: x + 273,15
fn celsius_para_kelvin(c: f64) {
    // TODO: Implemente como uma expressão de linha única, priorizando a precisão de ponto flutuante.

}

fn main() {
    // 1. Definição de dados
    // TODO: Inicie a variável temperatura_c como de ponto flutuante, com o valor 25,5
    // TODO: Depois de completar o código inteiro, você pode testar mudar esse valor


    // 2. Chamada de funções
    // TODO: Inicie as variávels temp_f e temp_k que convertem temperatura_c usando as funções de conversão



    // 3. Lógica de Alerta (Condicional como Expressão)
    // TODO: Use if como expressão (retorna valor) para definir a variável status a partir da temperatura_c
    // Se a temperatura for maior que 30, status é "ALERTA", caso contrário "NORMAL"


    // 4. Saída formatada
    println!("=== RELATÓRIO DO SENSOR ===");

    println!("Temperatura Original: {:.1}°C", temperatura_c);

    // TODO: Imprimir os valores convertidos com 2 casas decimais
    //"Fahrenheit: {??}°F", temp_f);
    //"Kelvin: {??}K", temp_k);


    println!("Status do Sistema: {}", status);
}
