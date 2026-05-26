# Track 001: Basic Foundations

**Learning Focus**: Manual implementation, explicit logic, ownership fundamentals

## 📚 Antes de Começar

**Novo em Rust?** Consulte o guia de referência básica primeiro:

- **[examples/rust_basics.rs](../examples/rust_basics.rs)** - Conceitos fundamentais explicados com exemplos práticos

Execute com: `cargo run --example rust_basics`

Este arquivo contém tudo que você precisa saber sobre variáveis, tipos, funções, formatação e muito mais. **Volte a ele sempre que tiver dúvidas sobre a sintaxe!**

## Philosophy

This track teaches you to be the **owner** of your code, not a passenger. You will:

- Write explicit `for` loops instead of using iterator methods
- Manage variables manually with clear mutability
- Understand what's happening at each step
- Build muscle memory for Rust's ownership model

**No syntactic sugar yet.** You need to understand what abstractions hide before using them.

## What You'll Learn

- Variables, types, and mutability
- Functions and return values
- Control flow: `if`, `else`, `match`
- Loops: `for`, `while`, `loop`
- Basic collections: `Vec`, `HashMap`, `HashSet`
- Ownership basics: borrowing, moving, cloning
- String handling: `String` vs `&str`, UTF-8
- Pattern matching with `match`
- Enums and structs (introduction)

## What You Won't Use (Yet)

❌ `.map()`, `.filter()`, `.fold()` - iterator adapters
❌ `.collect()` - implicit collection building
❌ `.max_by_key()`, `.min_by_key()` - functional selection
❌ Closures with captured variables
❌ Traits and generics
❌ Advanced lifetimes

## Exercise List

**Currently Available: 20 exercises**

Run with: `cargo run -p <folder_name>`

- **1. First Temperature Converter**
  - Pasta: [first_temperature_converter](first_temperature_converter/)
  - Conceitos: Functions, f64, formulas, let
- **2. Temperature Converter**
  - Pasta: [temperature_converter](temperature_converter/)
  - Conceitos: Functions, f64, formulas
- **3. FizzBuzz**
  - Pasta: [fizzbuzz](fizzbuzz/)
  - Conceitos: Loops, modulo, conditionals
- **4. Leap Year Checker**
  - Pasta: [leap_year_checker](leap_year_checker/)
  - Conceitos: Boolean logic, nested conditionals
- **5. Vowel Counter**
  - Pasta: [vowel_counter](vowel_counter/)
  - Conceitos: Char iteration, counting
- **6. Palindrome Checker**
  - Pasta: [palindrome_checker](palindrome_checker/)
  - Conceitos: String reversal with for loop
- **7. String Reverser**
  - Pasta: [string_reverser](string_reverser/)
  - Conceitos: UTF-8 handling, manual collection
- **8. Fibonacci Generator**
  - Pasta: [fibonacci_generator](fibonacci_generator/)
  - Conceitos: Loop with mutable variables
- **9. Prime Number Checker**
  - Pasta: [prime_number_checker](prime_number_checker/)
  - Conceitos: Nested loops, divisibility
- **10. Armstrong Number**
  - Pasta: [armstrong_number_checker](armstrong_number_checker/)
  - Conceitos: Digit extraction, exponentiation
- **11. Sum of Multiples**
  - Pasta: [sum_of_multiples](sum_of_multiples/)
  - Conceitos: Loop + accumulation
- **12. Collatz Conjecture**
  - Pasta: [collatz_conjecture](collatz_conjecture/)
  - Conceitos: Loop with conditional logic
- **13. Acronym Generator**
  - Pasta: [acronym_generator](acronym_generator/)
  - Conceitos: Split + for loop
- **14. Simple Calculator**
  - Pasta: [simple_calculator](simple_calculator/)
  - Conceitos: Enum, pattern matching
- **15. Guessing Game**
  - Pasta: [guessing_game_enhanced](guessing_game_enhanced/)
  - Conceitos: User input loop, random
- **16. Shopping List Manager**
  - Pasta: [shopping_list_manager](shopping_list_manager/)
  - Conceitos: Vec operations, menu
- **17. Word Counter (Basic)**
  - Pasta: [word_counter_basic](word_counter_basic/)
  - Conceitos: String splits, counting
- **18. Caesar Cipher**
  - Pasta: [caesar_cipher](caesar_cipher/)
  - Conceitos: Char manipulation, ASCII
- **19. Binary to Decimal**
  - Pasta: [binary_to_decimal](binary_to_decimal/)
  - Conceitos: Manual base conversion
- **20. ISBN Validator**
  - Pasta: [isbn_validator](isbn_validator/)
  - Conceitos: Validation pattern
- **21. Rock Paper Scissors**
  - Pasta: [rock_paper_scissors](rock_paper_scissors/)
  - Conceitos: Enum, random, game logic

### Coming Soon

More exercises will be added following the [TAXONOMY.md](../TAXONOMY.md) plan. Exercises can be added without renumbering existing ones!

## Estimated Time

**~20-25 hours** for completion (assuming 30-45 minutes per exercise)

## Prerequisites

- Basic programming knowledge (any language)
- Understanding of variables, functions, loops
- Willingness to write verbose code to understand fundamentals

## Next Steps

After completing this track:

1. Review your solutions - are they clear and explicit?
2. Move to **Track 002: Intermediate** for structs and algorithms
3. Eventually reach **Track 003: Advanced** to see the same problems solved functionally

Remember: **Abstractions are only useful when you know what they're hiding.**
