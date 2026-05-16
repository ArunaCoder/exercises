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

| #   | Exercise              | Key Concepts                            |
| --- | --------------------- | --------------------------------------- |
| 001 | Variables and Types   | Variables, mutability, type annotations |
| 002 | Temperature Converter | Functions, f64, formulas                |
| 003 | FizzBuzz              | Loops, modulo, conditionals             |
| 004 | Leap Year Checker     | Boolean logic, nested conditionals      |
| 005 | Vowel Counter         | Char iteration, counting                |
| 006 | Palindrome Checker    | String reversal with for loop           |
| 007 | String Reverser       | UTF-8 handling, manual collection       |
| 008 | Fibonacci Generator   | Loop with mutable variables             |
| 009 | Prime Number Checker  | Nested loops, divisibility              |
| 010 | Armstrong Number      | Digit extraction, exponentiation        |
| 011 | Sum of Multiples      | Loop + accumulation                     |
| 012 | Factorial (Iterative) | Loop-based factorial                    |
| 013 | Difference of Squares | Two loops, accumulation                 |
| 014 | Collatz Conjecture    | Loop with conditional logic             |
| 015 | Perfect Number        | Divisor finding with loops              |
| 016 | Raindrops             | Multiple modulo checks                  |
| 017 | Isogram Checker       | HashSet basics                          |
| 018 | Pangram Checker       | HashSet unique tracking                 |
| 019 | Acronym Generator     | Split + for loop                        |
| 020 | Scrabble Score        | HashMap lookup in loop                  |
| 021 | Simple Calculator     | Enum, pattern matching                  |
| 022 | Triangle Classifier   | Enum return type                        |
| 023 | Resistor Color Code   | Enum with values                        |
| 024 | Guessing Game         | User input loop, random                 |
| 025 | Shopping List Manager | Vec operations, menu                    |
| 026 | Word Counter          | String splits, counting                 |
| 027 | Caesar Cipher         | Char manipulation, ASCII                |
| 028 | Atbash Cipher         | Character transformation                |
| 029 | Binary to Decimal     | Manual base conversion                  |
| 030 | Luhn Algorithm        | Digit iteration, validation             |
| 031 | ISBN Validator        | Validation pattern                      |
| 032 | Nucleotide Count      | HashMap counting                        |
| 033 | Run-Length Encoding   | String grouping logic                   |

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
