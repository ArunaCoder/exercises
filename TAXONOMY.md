# Exercise Taxonomy - Track Classification

This document classifies all 100 exercises into learning tracks based on pedagogical progression.

## Track Philosophy

- **001-basic**: Foundation (33 exercises) - Loops, variables, basic control flow, ownership fundamentals
- **002-intermediate**: Structures (33 exercises) - Structs, enums, collections with loops, algorithms
- **003-advanced**: Functional (34 exercises) - Iterators, traits, generics, advanced patterns

## 001-basic/ (Exercises 1-33)

Focus: Manual loops, if/else, basic variables, simple functions. NO iterator methods yet.

| #   | Name                     | Rationale                                            |
| --- | ------------------------ | ---------------------------------------------------- |
| 001 | variables_and_types      | Start: basic variables, mutability, type annotations |
| 002 | temperature_converter    | Functions, f64, simple formulas                      |
| 003 | fizzbuzz                 | Loops, modulo, conditionals - classic beginner       |
| 004 | leap_year_checker        | Boolean logic, nested conditionals                   |
| 005 | vowel_counter            | Char iteration with for loop, counting               |
| 006 | palindrome_checker       | String reversal with for loop                        |
| 007 | string_reverser          | UTF-8 handling, manual char collection               |
| 008 | fibonacci_generator      | Loop with mutable variables, Vec::push               |
| 009 | prime_number_checker     | Nested loops, divisibility testing                   |
| 010 | armstrong_number_checker | Digit extraction, exponentiation                     |
| 011 | sum_of_multiples         | Loop + accumulation pattern                          |
| 012 | factorial_iterative      | Loop-based factorial (save recursive for advanced)   |
| 013 | difference_of_squares    | Two separate loops, accumulation                     |
| 014 | collatz_conjecture       | Loop with conditional logic                          |
| 015 | perfect_number_checker   | Divisor finding with loops                           |
| 016 | raindrops                | Multiple modulo checks, string building              |
| 017 | isogram_checker          | HashSet basics, simple insertion check               |
| 018 | pangram_checker          | HashSet to track unique chars                        |
| 019 | acronym_generator        | Split + for loop to extract first chars              |
| 020 | scrabble_score           | HashMap lookup in loop                               |
| 021 | simple_calculator        | Enum introduction, pattern matching basics           |
| 022 | triangle_classifier      | Enum return type, multiple conditions                |
| 023 | resistor_color_code      | Enum with pattern matching                           |
| 024 | guessing_game_enhanced   | User input loop, random numbers                      |
| 025 | shopping_list_manager    | Vec operations (push, remove), menu system           |
| 026 | word_counter_basic       | String splits, manual counting                       |
| 027 | caesar_cipher            | Char manipulation, ASCII math                        |
| 028 | atbash_cipher            | Character transformation formula                     |
| 029 | binary_to_decimal        | Manual base conversion with loops                    |
| 030 | luhn_algorithm           | Digit iteration, validation logic                    |
| 031 | isbn_validator           | Similar validation pattern                           |
| 032 | nucleotide_count         | HashMap counting with validation                     |
| 033 | run_length_encoding      | String building with grouping logic                  |

## 002-intermediate/ (Exercises 34-66)

Focus: Structs, methods, HashMap/Vec with loops, algorithms, no functional style yet.

| #   | Name                    | Rationale                                    |
| --- | ----------------------- | -------------------------------------------- |
| 034 | grade_calculator        | Struct with fields, weighted average         |
| 035 | simple_inventory_system | Struct methods, CRUD operations              |
| 036 | clock_implementation    | Struct, modulo arithmetic, Display trait     |
| 037 | queen_attack            | Struct Position, coordinate logic            |
| 038 | phone_number_formatter  | String processing, validation                |
| 039 | roman_numeral_converter | HashMap bidirectional conversion             |
| 040 | unit_converter          | Enum + pattern matching for conversions      |
| 041 | space_age_calculator    | Enum Planet, astronomical calculations       |
| 042 | median_loops            | Vec sorting + manual median (for loops only) |
| 043 | mode_loops              | HashMap + manual max finding (for loops)     |
| 044 | pig_latin               | String manipulation, UTF-8 chars             |
| 045 | company_departments     | HashMap<String, Vec>, sorting                |
| 046 | array_statistics        | Multiple statistical calculations            |
| 047 | word_counter_advanced   | HashMap word frequency                       |
| 048 | duplicate_remover       | HashSet + Vec, order preservation            |
| 049 | anagram_checker         | Sort + compare, no functional methods        |
| 050 | hamming_distance        | Dual iteration, error handling               |
| 051 | matching_brackets       | Vec as stack, manual pushing/popping         |
| 052 | secret_handshake        | Binary operations, bit testing               |
| 053 | allergies               | Binary flags, bitwise AND                    |
| 054 | sieve_of_eratosthenes   | Vec<bool> algorithm, prime generation        |
| 055 | bob_chatbot             | Text pattern analysis                        |
| 056 | protein_translation     | HashMap codon mapping, chunking              |
| 057 | saddle_points           | Nested loops, matrix operations              |
| 058 | etl_transform           | HashMap transformation                       |
| 059 | grains_on_chessboard    | Exponentiation, overflow handling            |
| 060 | largest_series_product  | Windows-like logic with manual loops         |
| 061 | rail_fence_cipher       | 2D vectors, encryption pattern               |
| 062 | diffie_hellman          | Modular arithmetic, cryptography             |
| 063 | dot_product             | Manual zip-like iteration                    |
| 064 | matrix_operations       | Nested loops, matrix math                    |
| 065 | spiral_matrix           | 2D generation, direction tracking            |
| 066 | pascals_triangle        | Nested Vec generation                        |

## 003-advanced/ (Exercises 67-100)

Focus: Iterators, closures, traits, generics, recursion, functional programming.

| #   | Name                         | Rationale                                        |
| --- | ---------------------------- | ------------------------------------------------ |
| 067 | median_functional            | **Same as 042** but with .sort(), .get(), .len() |
| 068 | mode_functional              | **Same as 043** but with .max_by_key()           |
| 069 | company_functional           | **Same as 045** but with entry API, .collect()   |
| 070 | word_freq_functional         | **Same as 047** but with iterator chains         |
| 071 | array_stats_functional       | **Same as 046** but with .fold(), .map()         |
| 072 | anagram_functional           | Sort with .chars().collect(), functional style   |
| 073 | duplicate_remove_functional  | .collect::<HashSet>(), dedupe methods            |
| 074 | tournament_results           | HashMap, sorting with custom comparator          |
| 075 | beer_song_generator          | String generation with ranges                    |
| 076 | food_chain_song              | Accumulation with iterator methods               |
| 077 | bottle_song                  | Range iteration, pluralization                   |
| 078 | kindergarten_garden          | HashMap, parsing, indexing                       |
| 079 | poker_hand_evaluator         | Complex struct, Ord trait, pattern recognition   |
| 080 | twelve_days_song             | Accumulation with ranges                         |
| 081 | word_search_puzzle           | 2D iteration, direction search                   |
| 082 | change_calculator            | Greedy algorithm or dynamic programming          |
| 083 | knapsack_problem             | Dynamic programming, optimization                |
| 084 | pythagorean_triplet_finder   | Iterator combinations                            |
| 085 | largest_palindrome_product   | Iterator product finding                         |
| 086 | simple_substitution_cipher   | HashMap mapping                                  |
| 087 | gigasecond_calculator        | Date/time operations                             |
| 088 | reverse_string_recursive     | Recursion demonstration                          |
| 089 | factorial_recursive          | Recursion with memoization                       |
| 090 | binary_search                | Classic algorithm, O(log n)                      |
| 091 | simple_grep                  | String filtering with iterators                  |
| 092 | word_wrap                    | Text processing with accumulation                |
| 093 | minesweeper_board            | 2D iteration, neighbor counting                  |
| 094 | connect_game_winner          | Graph traversal, DFS/BFS                         |
| 095 | dominoes_chain               | Backtracking, recursion                          |
| 096 | simple_state_machine         | Advanced enum patterns                           |
| 097 | rotational_cipher_generic    | Generic ROT-N with trait bounds                  |
| 098 | two_bucket_problem           | BFS, state exploration                           |
| 099 | yacht_scoring                | Complex pattern matching                         |
| 100 | linked_list_length_recursive | Recursive data structure traversal               |
| 101 | flatten_nested_array         | Recursive enum processing                        |
| 102 | sublist_checker              | .windows() iterator, comparison                  |
| 103 | all_your_base                | Mathematical base conversion                     |
| 104 | custom_set_generic           | Generic implementation, trait bounds             |
| 105 | circular_buffer_generic      | Generic data structure                           |
| 106 | simple_linked_list_generic   | Generic linked list, Box, Option                 |

## Duplicate Exercises (Different Implementations)

These exercises appear in multiple tracks with different solution approaches:

- **Median & Mode**: 042 (loops) → 067 (functional)
- **Company Departments**: 045 (loops) → 069 (entry API + functional)
- **Word Frequency**: 047 (loops) → 070 (iterator chains)
- **Array Statistics**: 046 (loops) → 071 (fold/map)
- **Anagram Checker**: 049 (manual sort) → 072 (chars().collect())
- **Duplicate Remover**: 048 (manual) → 073 (functional deduplication)

## Projects/ (Optional Integrative Projects)

Complex projects combining multiple concepts:

1. **CLI Todo List Manager** - File I/O, structs, serialization, command parsing
2. **Text Adventure Game** - State machine, enums, user input, game loop
3. **HTTP Log Analyzer** - File reading, regex, HashMap statistics, formatting
4. **Simple Web Scraper** - HTTP requests, HTML parsing, data extraction
5. **Markdown to HTML Converter** - Parsing, state machine, string building

## Implementation Notes

- Each track README should explain the pedagogical focus
- Advanced exercises should reference their basic equivalents
- Projects are optional capstone challenges
- Track 001 = ~20 hours, Track 002 = ~30 hours, Track 003 = ~40 hours
