# Trilha 001: Fundamentos Básicos

**Foco de Aprendizado**: Implementação manual, lógica explícita, fundamentos de ownership

## 📚 Antes de Começar

**Novo em Rust?** Consulte o guia de referência básica primeiro:

- **[start/src/main.rs](start/src/main.rs)** - Conceitos fundamentais explicados com exemplos práticos

Este arquivo contém tudo que você precisa saber sobre variáveis, tipos, funções, formatação e muito mais. **Volte a ele sempre que tiver dúvidas sobre a sintaxe!**

## Filosofia

Esta trilha ensina você a ser o **dono** do seu código, não um passageiro. Você irá:

- Escrever loops `for` explícitos em vez de usar métodos de iteradores
- Gerenciar variáveis manualmente com mutabilidade clara
- Entender o que está acontecendo em cada etapa
- Criar memória muscular para o modelo de ownership do Rust

**Nada de "syntactic sugar" por enquanto.** Você precisa entender o que as abstrações escondem antes de usá-las.

## O Que Você Irá Aprender

- Variáveis, tipos e mutabilidade
- Funções e valores de retorno
- Fluxo de controle: `if`, `else`, `match`
- Loops: `for`, `while`, `loop`
- Coleções básicas: `Vec`, `HashMap`, `HashSet`
- Fundamentos de ownership: borrowing, moving, cloning
- Manipulação de Strings: `String` vs `&str`, UTF-8
- Pattern matching com `match`
- Enums e structs (introdução)

## O Que Você Não Irá Usar (Ainda)

❌ `.map()`, `.filter()`, `.fold()` - adaptadores de iteradores
❌ `.collect()` - construção implícita de coleções
❌ `.max_by_key()`, `.min_by_key()` - seleção funcional
❌ Closures com variáveis capturadas
❌ Traits e generics
❌ Lifetimes avançados

## Lista de Exercícios

**Atualmente Disponível: 22 exercícios**

Execute com: `cargo run -p <nome_da_pasta>`

- **1. Primeiro Conversor de Temperatura**
  - Pasta: [first_temperature_converter](first_temperature_converter/)
  - Conceitos: Functions, f64, formulas, let
- **2. Conversor de Temperatura**
  - Pasta: [temperature_converter](temperature_converter/)
  - Conceitos: Functions, f64, formulas
- **3. Sair do loop**
  - Pasta: [exit_loop](exit_loop/)
  - Conceitos: loop, break, if
- **4. FizzBuzz**
  - Pasta: [fizzbuzz](fizzbuzz/)
  - Conceitos: Loops, modulo, conditionals
- **5. Verificador de Ano Bissexto**
  - Pasta: [leap_year_checker](leap_year_checker/)
  - Conceitos: Boolean logic, nested conditionals
- **6. Contador de Vogais**
  - Pasta: [vowel_counter](vowel_counter/)
  - Conceitos: Char iteration, counting
- **7. Verificador de Palíndromo**
  - Pasta: [palindrome_checker](palindrome_checker/)
  - Conceitos: String reversal com loop for
- **8. Inversor de String**
  - Pasta: [string_reverser](string_reverser/)
  - Conceitos: UTF-8 handling, manual collection
- **9. Gerador de Fibonacci**
  - Pasta: [fibonacci_generator](fibonacci_generator/)
  - Conceitos: Loop com variáveis mutáveis
- **10. Verificador de Números Primos**
  - Pasta: [prime_number_checker](prime_number_checker/)
  - Conceitos: Nested loops, divisibility
- **11. Número de Armstrong**
  - Pasta: [armstrong_number_checker](armstrong_number_checker/)
  - Conceitos: Digit extraction, exponentiation
- **12. Soma de Múltiplos**
  - Pasta: [sum_of_multiples](sum_of_multiples/)
  - Conceitos: Loop + accumulation
- **13. Conjectura de Collatz**
  - Pasta: [collatz_conjecture](collatz_conjecture/)
  - Conceitos: Loop com lógica condicional
- **14. Gerador de Acrônimos**
  - Pasta: [acronym_generator](acronym_generator/)
  - Conceitos: Split + loop for
- **15. Calculadora Simples**
  - Pasta: [simple_calculator](simple_calculator/)
  - Conceitos: Enum, pattern matching
- **16. Jogo de Adivinhação**
  - Pasta: [guessing_game_enhanced](guessing_game_enhanced/)
  - Conceitos: User input loop, random
- **17. Gerenciador de Lista de Compras**
  - Pasta: [shopping_list_manager](shopping_list_manager/)
  - Conceitos: Vec operations, menu
- **18. Contador de Palavras (Básico)**
  - Pasta: [word_counter_basic](word_counter_basic/)
  - Conceitos: String splits, counting
- **19. Cifra de César**
  - Pasta: [caesar_cipher](caesar_cipher/)
  - Conceitos: Char manipulation, ASCII
- **20. Binário para Decimal**
  - Pasta: [binary_to_decimal](binary_to_decimal/)
  - Conceitos: Manual base conversion
- **21. Validador de ISBN**
  - Pasta: [isbn_validator](isbn_validator/)
  - Conceitos: Validation pattern
- **22. Pedra Papel Tesoura**
  - Pasta: [rock_paper_scissors](rock_paper_scissors/)
  - Conceitos: Enum, random, game logic

### Em Breve

Mais exercícios serão adicionados seguindo o plano [TAXONOMY.md](../TAXONOMY.md). Exercícios podem ser adicionados sem a necessidade de renumerar os existentes!

## Tempo Estimado

**~20-25 horas** para conclusão (assumindo 30-45 minutos por exercício)

## Pré-requisitos

- Conhecimento básico de programação (qualquer linguagem)
- Entendimento de variáveis, funções e loops
- Disposição para escrever código verboso para entender os fundamentos

## Próximos Passos

Após completar esta trilha:

1. Revise suas soluções - elas estão claras e explícitas?
2. Vá para a **Trilha 002: Intermediário** para structs e algoritmos
3. Eventualmente chegue à **Trilha 003: Avançado** para ver os mesmos problemas resolvidos de forma funcional

Lembre-se: **Abstrações só são úteis quando você sabe o que elas estão escondendo.**
