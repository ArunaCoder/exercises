# Track 001: Basic Foundations

**Learning Focus**: Manual implementation, explicit logic, ownership fundamentals

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

| #   | Exercise              | Folder                                                | Key Concepts                       |
| --- | --------------------- | ----------------------------------------------------- | ---------------------------------- |
| 1   | Temperature Converter | [temperature_converter](temperature_converter/)       | Functions, f64, formulas           |
| 2   | FizzBuzz              | [fizzbuzz](fizzbuzz/)                                 | Loops, modulo, conditionals        |
| 3   | Leap Year Checker     | [leap_year_checker](leap_year_checker/)               | Boolean logic, nested conditionals |
| 4   | Vowel Counter         | [vowel_counter](vowel_counter/)                       | Char iteration, counting           |
| 5   | Palindrome Checker    | [palindrome_checker](palindrome_checker/)             | String reversal with for loop      |
| 6   | String Reverser       | [string_reverser](string_reverser/)                   | UTF-8 handling, manual collection  |
| 7   | Fibonacci Generator   | [fibonacci_generator](fibonacci_generator/)           | Loop with mutable variables        |
| 8   | Prime Number Checker  | [prime_number_checker](prime_number_checker/)         | Nested loops, divisibility         |
| 9   | Armstrong Number      | [armstrong_number_checker](armstrong_number_checker/) | Digit extraction, exponentiation   |
| 10  | Sum of Multiples      | [sum_of_multiples](sum_of_multiples/)                 | Loop + accumulation                |
| 11  | Collatz Conjecture    | [collatz_conjecture](collatz_conjecture/)             | Loop with conditional logic        |
| 12  | Acronym Generator     | [acronym_generator](acronym_generator/)               | Split + for loop                   |
| 13  | Simple Calculator     | [simple_calculator](simple_calculator/)               | Enum, pattern matching             |
| 14  | Guessing Game         | [guessing_game_enhanced](guessing_game_enhanced/)     | User input loop, random            |
| 15  | Shopping List Manager | [shopping_list_manager](shopping_list_manager/)       | Vec operations, menu               |
| 16  | Word Counter (Basic)  | [word_counter_basic](word_counter_basic/)             | String splits, counting            |
| 17  | Caesar Cipher         | [caesar_cipher](caesar_cipher/)                       | Char manipulation, ASCII           |
| 18  | Binary to Decimal     | [binary_to_decimal](binary_to_decimal/)               | Manual base conversion             |
| 19  | ISBN Validator        | [isbn_validator](isbn_validator/)                     | Validation pattern                 |
| 20  | Rock Paper Scissors   | [rock_paper_scissors](rock_paper_scissors/)           | Enum, random, game logic           |

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
