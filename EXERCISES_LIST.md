# Rust Basic Exercises - Complete List

This list contains 100 basic Rust exercises to train fundamental language concepts.

## Implemented Exercises

### 01. Median and Mode

Given a vector of integers, calculate and return both the median (the middle value when sorted) and the mode (the value that appears most frequently). Use a Vec<i32> to store the numbers and a HashMap to count occurrences. Sort the vector to find the median, then iterate through the HashMap to find the key with the highest count.
**Concepts:** Vectors, HashMap, sorting, iteration, mathematical operations

### 02. Pig Latin Converter

Transform English words into Pig Latin by applying two rules: for words starting with a consonant, move the first consonant to the end and add "-ay"; for words starting with a vowel, simply add "-hay" to the end. Accept a String as input and return the transformed String. Handle UTF-8 characters properly using the chars() method.
**Concepts:** Strings, chars, pattern matching, string manipulation, UTF-8

### 03. Company Department Manager

Build a text-based system to manage employees across different departments. Use a HashMap<String, Vec<String>> where keys are department names and values are vectors of employee names. Implement functionality to parse commands like "Add Sally to Engineering" and retrieve sorted lists of employees by department or for the entire company.
**Concepts:** HashMap, Vectors, sorting, string parsing, text commands

---

## Proposed Exercises

### 04. Temperature Converter

Create a command-line program that converts temperatures between Fahrenheit and Celsius. Accept user input specifying the temperature value and the direction of conversion (F to C or C to F). Use functions for each conversion formula and display the result with appropriate formatting.
**Concepts:** Functions, variables, control flow, user input, f64 data type

### 05. Fibonacci Generator

Generate and return the first N numbers of the Fibonacci sequence. Take an integer N as input and produce a Vec<u64> containing the sequence. Start with 0 and 1, then each subsequent number is the sum of the previous two. Use a loop with mutable variables to build the sequence.
**Concepts:** Loops, variables, mutability, vectors, functions

### 06. Prime Number Checker

Create a function that determines whether a given number is prime. Additionally, implement a function that returns a Vec<u32> containing all prime numbers up to a given limit N. Use nested loops to check divisibility and boolean logic to validate primality.
**Concepts:** Loops, functions, boolean logic, control flow, conditionals

### 07. String Reverser

Reverse a String while correctly handling UTF-8 multi-byte characters (like emojis and accented letters). Take a String or &str as input and return a new String with characters in reverse order. Use the chars() method to iterate over Unicode scalar values rather than bytes.
**Concepts:** Strings, chars, iteration, UTF-8, Unicode handling

### 08. Word Counter

Analyze a text string and count the number of words, lines, and characters. Return these three counts as a tuple (usize, usize, usize). Split the string by whitespace for word counting, by newlines for line counting, and iterate over chars for character counting. Optionally use a HashMap to count word frequency.
**Concepts:** Strings, iteration, HashMap, counting, tuples

### 09. Palindrome Checker

Determine whether a given word or phrase is a palindrome (reads the same forwards and backwards). Normalize the input by converting to lowercase and removing non-alphanumeric characters. Compare the cleaned string with its reverse to check for palindrome property.
**Concepts:** Strings, comparison, iteration, normalization, filtering

### 10. Array Statistics

Calculate comprehensive statistics for a vector of floating-point numbers: mean (average), median, mode, and variance. Store numbers in a Vec<f64>. Sort for median calculation, use HashMap for mode counting, and apply statistical formulas for mean and variance.
**Concepts:** Vectors, functions, mathematical operations, sorting, HashMap

### 11. Simple Calculator

Build a basic calculator that can perform addition, subtraction, multiplication, and division. Parse a string expression like "5 + 3" or "10 / 2" and return the result. Use an enum to represent operations and pattern matching to execute the correct operation. Handle division by zero appropriately.
**Concepts:** Enums, pattern matching, functions, error handling, string parsing

### 12. Guessing Game Enhanced

Create an interactive number guessing game where the program randomly selects a number between 1 and 100, and the player tries to guess it. Provide feedback ("too high" or "too low") after each guess and count the number of attempts. Use loops for repeated guessing and comparison operators for feedback.
**Concepts:** Loops, random numbers, user input, comparison, control flow

### 13. Shopping List Manager

Implement a shopping list application where users can add items, remove items, and display all items. Store items in a Vec<String>. Present a text menu with options (1: Add, 2: Remove, 3: List, 4: Quit) and loop until the user chooses to exit.
**Concepts:** Vectors, user input, control flow, string manipulation, menu systems

### 14. Grade Calculator

Calculate a student's final grade based on multiple assignments with different weights. Create a struct to hold assignment names, scores, and weights. Store multiple assignments in a Vec and compute the weighted average. Return the final grade as a percentage or letter grade.
**Concepts:** Vectors, structs, mathematical operations, functions, weighted averages

### 15. Vowel Counter

Count the number of vowels and consonants in a given text string. Iterate through each character using chars(), check if it's a vowel (a, e, i, o, u), and maintain separate counters. Return both counts as a tuple (usize, usize).
**Concepts:** Strings, chars, pattern matching, counting, conditionals

### 16. Acronym Generator

Generate acronyms from phrases by taking the first letter of each word and capitalizing it. For example, "Portable Network Graphics" becomes "PNG". Split the input string by whitespace, extract the first character of each word, convert to uppercase, and collect into a new String.
**Concepts:** Strings, iteration, uppercase conversion, collection, character extraction

### 17. Anagram Checker

Determine if two words are anagrams (contain the same letters in different order). Normalize both strings to lowercase, convert to char vectors, sort both vectors, and compare for equality. Return a boolean indicating whether they are anagrams.
**Concepts:** Strings, sorting, comparison, character manipulation, vectors

### 18. Caesar Cipher

Implement the Caesar cipher encryption technique where each letter is shifted by N positions in the alphabet. Accept a String and a shift amount (i32), then return the encrypted String. Handle wrapping (e.g., 'z' shifted by 1 becomes 'a') using modulo arithmetic. Preserve non-alphabetic characters.
**Concepts:** Strings, chars, ASCII manipulation, modulo arithmetic, character mapping

### 19. Duplicate Remover

Remove duplicate elements from a vector while preserving the original order of first occurrences. Take a Vec<i32> as input and return a new Vec<i32> without duplicates. Use a HashSet to track seen elements while iterating through the original vector.
**Concepts:** Vectors, HashSet, iteration, deduplication, order preservation

### 20. FizzBuzz

Implement the classic FizzBuzz problem: print numbers from 1 to 100, but for multiples of 3 print "Fizz", for multiples of 5 print "Buzz", and for multiples of both print "FizzBuzz". Use a loop with conditionals and modulo operations to check divisibility.
**Concepts:** Loops, conditionals, modulo operations, printing, control flow

### 21. Leap Year Checker

Determine whether a given year is a leap year according to the Gregorian calendar rules. A year is a leap year if it's divisible by 4, except for years divisible by 100 unless they are also divisible by 400. Take a u32 representing the year and return a bool. Use nested conditionals or boolean logic to implement the rules.
**Concepts:** Functions, boolean logic, conditionals, modulo operations

### 22. Rock Paper Scissors

Create a command-line game of Rock, Paper, Scissors against the computer. Define an enum with variants Rock, Paper, and Scissors. Generate a random computer choice, accept user input, and determine the winner using pattern matching. Keep playing until the user chooses to quit.
**Concepts:** Enums, pattern matching, random generation, user input, game logic

### 23. Unit Converter

Build a unit converter that can convert between different units of measurement such as meters, kilometers, miles, inches, feet, etc. Define an enum for unit types and use pattern matching to determine which conversion formula to apply. Accept a value (f64), source unit, and target unit, then return the converted value.
**Concepts:** Functions, pattern matching, enums, f64 operations, conversion formulas

### 24. Simple Inventory System

Create an inventory management system for a store. Define a struct Product with fields for name (String), quantity (u32), and price (f64). Store multiple products in a Vec<Product>. Implement functionality to add new products, update quantities, remove products, and display all items with total inventory value.
**Concepts:** Structs, vectors, methods, CRUD operations, struct field access

### 25. ISBN Validator

Validate ISBN-10 and ISBN-13 book identification numbers using their respective checksum algorithms. For ISBN-10, multiply each of the first 9 digits by their position (1-9) and sum them, then check if the sum modulo 11 equals the check digit. For ISBN-13, alternate multiplying digits by 1 and 3. Accept a String and return a bool indicating validity.
**Concepts:** Strings, iteration, validation, checksum algorithms, digit manipulation

### 26. Binary to Decimal Converter

Convert binary number strings to decimal integers and vice versa. Accept a String containing only '0' and '1' characters, iterate through it, and calculate the decimal value using powers of 2. For the reverse operation, repeatedly divide by 2 and collect remainders. Return the converted value as a String or u32.
**Concepts:** Strings, parsing, mathematical operations, base conversion, powers

### 27. Armstrong Number Checker

Determine if a number is an Armstrong number (also called narcissistic number), where the sum of its digits raised to the power of the number of digits equals the number itself. For example, 153 is an Armstrong number because 1³ + 5³ + 3³ = 153. Extract individual digits from a u32 and perform exponentiation.
**Concepts:** Mathematical operations, loops, digit extraction, exponentiation

### 28. Sum of Multiples

Calculate the sum of all positive integers below a given number N that are multiples of 3 or 5. For example, if N is 10, the multiples are 3, 5, 6, and 9, which sum to 23. Use a loop to iterate from 1 to N-1, check divisibility with modulo, and accumulate the sum in a mutable variable.
**Concepts:** Loops, conditionals, accumulation, modulo operations

### 29. Largest Product in Series

Given a string of digits, find the largest product of N consecutive digits. For example, in the string "1234", the largest product of 2 consecutive digits is 3×4 = 12. Use the windows() method on a Vec<u32> of digits to examine consecutive subslices, calculate their product, and track the maximum.
**Concepts:** Strings, windows iteration, multiplication, maximum tracking

### 30. Collatz Conjecture

Generate the Collatz sequence (also called the 3n+1 problem) for a starting number. If the number is even, divide it by 2; if odd, multiply by 3 and add 1. Continue until reaching 1. Return a Vec<u64> containing the complete sequence. Use a loop with conditional logic to apply the rules.
**Concepts:** Loops, mathematical operations, sequence generation, conditionals

### 31. Pangram Checker

Determine if a sentence is a pangram (contains all 26 letters of the alphabet at least once). Convert the input string to lowercase, iterate through each character, and add alphabetic characters to a HashSet<char>. After processing, check if the set contains 26 unique letters.
**Concepts:** Strings, HashSet, character validation, set operations

### 32. Roman Numeral Converter

Convert between Roman numerals and decimal integers in both directions. Create a HashMap mapping Roman symbols (I, V, X, L, C, D, M) to their values. For Roman to decimal, iterate through the string and add or subtract values based on whether each symbol is less than the next. For decimal to Roman, use a greedy approach with descending value pairs.
**Concepts:** Strings, HashMap, pattern matching, conversion algorithms

### 33. Luhn Algorithm

Validate credit card numbers using the Luhn algorithm (checksum formula). Starting from the rightmost digit, double every second digit, subtract 9 if the result exceeds 9, sum all digits, and check if the total is divisible by 10. Accept a String of digits and return a bool indicating validity.
**Concepts:** Strings, digit manipulation, validation algorithms, modulo

### 34. Scrabble Score Calculator

Calculate the point value of words according to Scrabble letter values. Create a HashMap mapping each letter to its point value (A=1, E=1, K=5, Q=10, etc.). Accept a String word, convert to uppercase, lookup each letter's value, and sum the total score.
**Concepts:** HashMap, strings, iteration, scoring systems, lookups

### 35. Triangle Type Classifier

Given three side lengths, determine the type of triangle: equilateral (all sides equal), isosceles (two sides equal), or scalene (no sides equal). Also validate that the sides can form a valid triangle using the triangle inequality theorem (sum of any two sides must exceed the third). Define an enum TriangleType and return it along with a validity check.
**Concepts:** Functions, conditionals, enums, validation, geometric logic

### 36. Resistor Color Code

Decode resistor color bands into their numeric resistance values. Define an enum ResistorColor with variants for each color (Black, Brown, Red, etc.). Given a Vec<ResistorColor> representing the color bands, convert each to its numeric value using pattern matching and calculate the total resistance using the formula: first_digit \* 10 + second_digit, multiplied by 10^third_band.
**Concepts:** Enums, pattern matching, vectors, lookup logic, calculations

### 37. Phone Number Formatter

Clean and format phone numbers into a standard format. Accept various input formats like "(555) 123-4567", "555.123.4567", or "5551234567" and normalize them to a consistent format. Remove non-numeric characters except extensions, validate that exactly 10 digits remain (or 11 if country code 1 is present), and format as (XXX) XXX-XXXX.
**Concepts:** Strings, validation, formatting, character filtering

### 38. Clock Implementation

Create a Clock struct that stores hours and minutes and can add or subtract minutes while properly handling overflow (wrapping at 24 hours). Implement methods like `add_minutes()` and `subtract_minutes()` using modulo arithmetic. Also implement the Display trait to format output as "HH:MM".
**Concepts:** Structs, methods, modulo arithmetic, Display trait, time calculations

### 39. Perfect Number Checker

Determine if a number is perfect (equals the sum of its proper divisors excluding itself). For example, 6 is perfect because 1 + 2 + 3 = 6. Find all divisors of a u32 by testing each number from 1 to n/2, checking divisibility with modulo, accumulating the sum, and comparing to the original number.
**Concepts:** Loops, mathematical operations, divisor finding, accumulation

### 40. Hamming Distance

Calculate the Hamming distance between two strings of equal length (the number of positions where corresponding characters differ). For example, "karolin" and "kathrin" have a Hamming distance of 3. Iterate through both strings simultaneously using zip(), compare characters, and count differences. Return an error if lengths don't match.
**Concepts:** Strings, iteration, comparison, error handling, zip

### 41. Run-Length Encoding

Implement run-length encoding compression: convert "AAABBBCCCC" to "3A3B4C". Iterate through the string, count consecutive identical characters, and build a new String with count+character pairs. For decoding, parse the encoded string and expand each count+character back to repeated characters.
**Concepts:** Strings, iteration, compression, grouping, parsing

### 42. Difference of Squares

Calculate the difference between the square of the sum and the sum of squares for numbers 1 to N. For example, for N=10: (1+2+...+10)² - (1²+2²+...+10²) = 55² - 385 = 2640. Use loops to calculate both values separately, then compute their difference.
**Concepts:** Mathematical operations, loops, functions, squaring

### 43. Matching Brackets

Verify that brackets, parentheses, and braces are properly balanced in a string. Every opening bracket must have a corresponding closing bracket in the correct order. Use a Vec as a stack: push opening brackets onto it and pop when encountering closing brackets, verifying they match. The stack should be empty at the end for valid input.
**Concepts:** Vectors (as stack), iteration, pattern matching, validation

### 44. Secret Handshake

Generate a sequence of actions for a secret handshake based on a binary number. Each bit position represents an action: 1=wink, 2=double blink, 4=close eyes, 8=jump. If bit 16 is set, reverse the sequence. Use bitwise AND operations to test each bit and build a Vec<String> of actions.
**Concepts:** Binary operations, vectors, pattern matching, bit manipulation

### 45. Allergies

Determine a person's allergies based on an allergy score using binary flags. Each allergen corresponds to a power of 2: eggs=1, peanuts=2, shellfish=4, strawberries=8, etc. Given a u32 score, use bitwise AND to test which allergens are present and return a Vec containing the allergen names.
**Concepts:** Binary operations, enums, vectors, bit manipulation, flags

### 46. Sieve of Eratosthenes

Implement the Sieve of Eratosthenes algorithm to efficiently find all prime numbers up to a given limit N. Create a Vec<bool> of size N+1 initialized to true. Starting from 2, mark all multiples of each prime as false. After processing, collect all indices still marked true into a Vec<u32> of primes.
**Concepts:** Vectors, boolean arrays, loops, algorithms, prime generation

### 47. Bob Chatbot

Create a chatbot that responds like Bob, a lackadaisical teenager. Bob responds differently based on input: "Whatever." for statements, "Whoa, chill out!" for yelling, "Calm down, I know what I'm doing!" for yelling questions, "Sure." for questions, and "Fine. Be that way!" for silence. Analyze the input String to detect these patterns.
**Concepts:** Strings, pattern matching, text analysis, conditionals

### 48. Protein Translation

Translate RNA sequences into protein names using codon mappings. Each 3-letter codon maps to a protein: "AUG"→Methionine, "UUU"→Phenylalanine, etc. Create a HashMap of codon to protein mappings. Split the RNA String into 3-character chunks, translate each codon, and stop at stop codons (UAA, UAG, UGA). Return a Vec<String> of protein names.
**Concepts:** Strings, HashMap, chunking, pattern matching, translation

### 49. Queen Attack

Determine if two chess queens on a board can attack each other. Queens can attack along rows, columns, and diagonals. Create a struct Position with row and column (u8). Given two positions, check if they share the same row, column, or diagonal (where the difference in rows equals the difference in columns).
**Concepts:** Structs, methods, coordinate logic, validation, comparison

### 50. Saddle Points

Find saddle points in a matrix (elements that are the largest in their row and smallest in their column, or vice versa). Store the matrix as Vec<Vec<u32>>. For each element, check if it's the max in its row and min in its column. Return a Vec of (row, column) tuples representing saddle point positions.
**Concepts:** Vectors, nested loops, matrix operations, comparison, coordinates

### 51. ETL Transform

Transform data from a legacy format to a modern format. The old format uses a HashMap<u32, Vec<char>> where keys are scores and values are letters with that score (e.g., 1→[A,E,I]). Convert to HashMap<char, u32> where each letter maps to its score. Iterate through the old map, and for each letter in each vector, insert it with its corresponding score into the new map.
**Concepts:** HashMap, iteration, transformation, key-value manipulation

### 52. Space Age Calculator

Calculate a person's age on different planets of the solar system. Each planet has a different orbital period relative to Earth years: Mercury=0.2408467, Venus=0.61519726, Mars=1.8808158, etc. Create an enum Planet and given a person's age in seconds (u64), divide by Earth year seconds then by the planet's orbital period to get their age on that planet.
**Concepts:** Structs, methods, enums, astronomical calculations, f64

### 53. Nucleotide Count

Count the occurrences of each nucleotide (A, C, G, T) in a DNA sequence string. Validate that the string contains only valid nucleotides. Create a HashMap<char, usize> to store counts, iterate through the String, and increment the appropriate counter. Return an error if invalid characters are encountered.
**Concepts:** HashMap, strings, iteration, validation, counting

### 54. Grains on Chessboard

Calculate the number of grains of wheat on a chessboard where each square has double the grains of the previous square, starting with 1. For square N, the number of grains is 2^(N-1). Calculate for a specific square or sum all 64 squares. Use u64 to handle large numbers and be careful of overflow.
**Concepts:** u64, exponential growth, overflow handling, exponentiation

### 55. Circular Buffer

Implement a fixed-size circular buffer (ring buffer) that overwrites old data when full. Create a struct with a Vec<T> and read/write pointers. Implement methods to write (which wraps around when reaching capacity), read, and clear. Use modulo arithmetic to handle wraparound.
**Concepts:** Vectors, modulo, methods, data structures, generics

### 56. Simple Linked List

Implement a singly-linked list from scratch using structs and pointers. Define a Node struct with a value and an Option<Box<Node>> for the next node. Implement push (add to front), pop (remove from front), and length methods. Handle ownership carefully with Box for heap allocation.
**Concepts:** Structs, Option, Box, ownership, pointers, recursion

### 57. Tournament Results

Calculate tournament standings from match results. Parse input strings like "Allegiance;Blithery;win" where format is: team1;team2;result. Track each team's wins, draws, and losses in a HashMap<String, TeamStats>. Calculate points (3 for win, 1 for draw, 0 for loss) and sort teams by points, then alphabetically.
**Concepts:** HashMap, structs, sorting, parsing, statistics

### 58. Largest Series Product

Find the largest product of N adjacent digits in a string of digits. For example, in "123456", the largest product of 3 adjacent digits is 4×5×6=120. Convert the String to Vec<u32>, use windows(n) to get slices of adjacent digits, calculate each product, and track the maximum.
**Concepts:** Strings, windows, iteration, multiplication, maximum tracking

### 59. Atbash Cipher

Implement the Atbash cipher where the alphabet is reversed (A↔Z, B↔Y, etc.). For each letter, calculate its reverse: for 'a', its position is 0, reversed position is 25, giving 'z'. Convert the String to lowercase, transform each alphabetic character, and preserve non-alphabetic characters.
**Concepts:** Strings, character mapping, transformation, alphabet manipulation

### 60. Rail Fence Cipher

Implement the rail fence cipher encryption technique. Text is written in a zigzag pattern across N "rails" and then read row by row. For encoding, determine which rail each character belongs to based on its position and the zigzag pattern. Store characters in Vec<Vec<char>> representing each rail, then concatenate. For decoding, reverse the process.
**Concepts:** Strings, 2D vectors, pattern logic, encryption algorithms

### 61. Diffie-Hellman Key Exchange

Implement a simplified Diffie-Hellman key exchange for cryptography. Given a prime number p and a base g, each party generates a private key (random u64), calculates their public key as g^private mod p, and can compute a shared secret using the other's public key. Implement modular exponentiation carefully to avoid overflow.
**Concepts:** Mathematical operations, modular arithmetic, cryptography basics, u64

### 62. Dot Product Calculator

Calculate the dot product of two vectors (sum of products of corresponding elements). Accept two Vec<f64> of equal length, zip them together, multiply corresponding pairs, and sum the results. Return an error if vectors have different lengths.
**Concepts:** Vectors, iteration, mathematical operations, zip, validation

### 63. Matrix Operations

Implement basic matrix operations: addition, subtraction, and multiplication. Represent matrices as Vec<Vec<f64>>. For addition/subtraction, ensure matrices have the same dimensions and add/subtract corresponding elements. For multiplication, ensure inner dimensions match and compute dot products of rows and columns.
**Concepts:** Nested vectors, loops, matrix mathematics, validation

### 64. Spiral Matrix Generator

Generate an N×N matrix filled with numbers 1 to N² in a spiral pattern starting from the top-left and moving clockwise. Create a Vec<Vec<u32>> initialized with zeros. Track current position and direction (right, down, left, up), fill cells sequentially, and turn when hitting boundaries or filled cells.
**Concepts:** 2D vectors, algorithms, pattern generation, direction tracking

### 65. Pascal's Triangle

Generate the first N rows of Pascal's Triangle where each number is the sum of the two numbers above it. Return Vec<Vec<u32>>. The first row is [1], second is [1,1], third is [1,2,1], etc. Each row starts and ends with 1, and interior elements are calculated from the previous row.
**Concepts:** Vectors, nested loops, mathematical patterns, generation

### 66. Beer Song Generator

Generate the lyrics to "99 Bottles of Beer on the Wall" for any range of verses. Each verse follows the pattern: "N bottles of beer on the wall, N bottles of beer. Take one down and pass it around, N-1 bottles of beer on the wall." Handle pluralization ("bottle" vs "bottles") and special cases for 0 and 1 bottle. Return Vec<String> of verses.
**Concepts:** Loops, strings, conditionals, text generation, pluralization

### 67. Food Chain Song

Generate lyrics to "There Was an Old Lady Who Swallowed a Fly" with accumulating verses. Each new verse repeats all previous animals in reverse order. Store animals and their rhymes in Vecs, build each verse by combining current and previous content. Return a complete song String with proper formatting.
**Concepts:** Strings, iteration, accumulation, pattern generation

### 68. Bottle Song

Generate verses for a countdown song: "Ten green bottles hanging on the wall... And if one green bottle should accidentally fall, there'll be nine green bottles hanging on the wall." Handle number words (one through ten) and pluralization. Return Vec<String> containing all verses.
**Concepts:** Loops, string formatting, pluralization, number words

### 69. Kindergarten Garden

Determine which plants each student has in a kindergarten garden based on a diagram. The garden is represented as a String with two rows, where each pair of characters represents one student's plants (G=grass, C=clover, R=radishes, V=violets). Map student names to positions alphabetically and return a Vec<String> of their plants.
**Concepts:** Strings, HashMap, parsing, lookup logic, indexing

### 70. Poker Hand Evaluator

Evaluate and compare poker hands. Define structs for Card and Hand. Determine hand rankings (high card, pair, two pair, three of a kind, straight, flush, full house, four of a kind, straight flush). Parse card strings like "4S" (4 of Spades), sort cards, check for patterns, and return an enum HandRank. Also implement comparison to determine which hand wins.
**Concepts:** Structs, enums, sorting, complex logic, pattern recognition

### 71. Twelve Days Song

Generate lyrics to "The Twelve Days of Christmas." Each verse adds a new gift while repeating all previous gifts in reverse order. Store gifts in a Vec with proper ordinal numbers (first, second, etc.). Build each verse by accumulating from day 1 to current day. Return complete song as Vec<String>.
**Concepts:** Vectors, loops, accumulation, string building, ordinals

### 72. Word Search Puzzle

Find words hidden in a letter grid (horizontal, vertical, or diagonal in any direction). Accept a Vec<Vec<char>> grid and a Vec<String> of words to find. For each word, search in all 8 directions from each starting position. Return a HashMap<String, Option<(start_row, start_col, end_row, end_col)>> indicating where each word was found.
**Concepts:** 2D vectors, iteration, direction search, coordinates, pattern matching

### 73. Change Calculator

Calculate the minimum number of coins needed to make change for a given amount. Accept a target value (u32) and a Vec<u32> of available coin denominations. Use a greedy algorithm (starting with largest coin) or dynamic programming for optimal solution. Return a Vec<u32> showing count of each coin type used.
**Concepts:** Vectors, greedy algorithms, optimization, dynamic programming

### 74. Knapsack Problem

Solve the 0/1 knapsack problem: given items with weights and values, determine the maximum value achievable within a weight limit. Define a struct Item with weight and value fields. Use dynamic programming with a Vec<Vec<u32>> table to track maximum values for each weight capacity. Return the maximum value and optionally which items to include.
**Concepts:** Vectors, optimization, dynamic programming, structs

### 75. ISBN-13 Check Digit Generator

Generate the check digit for an ISBN-13 number. Given the first 12 digits as a String, calculate the check digit using the ISBN-13 algorithm: multiply odd-position digits by 1 and even-position digits by 3, sum them, and the check digit is 10 minus (sum mod 10), mod 10. Append and return the complete ISBN-13.
**Concepts:** Strings, checksum algorithms, mathematical operations, digit manipulation

### 76. Pythagorean Triplet Finder

Find all Pythagorean triplets (a, b, c where a² + b² = c²) that sum to a given number N. Use nested loops to test combinations where a < b < c. Check both the Pythagorean condition and sum condition. Return Vec<(u32, u32, u32)> of all valid triplets found.
**Concepts:** Nested loops, mathematical operations, conditions, tuples

### 77. Sum of Squares

Calculate the sum of squares from 1 to N: 1² + 2² + 3² + ... + N². Use a loop to iterate from 1 to N, square each number, and accumulate the sum. Alternatively, use the mathematical formula: N(N+1)(2N+1)/6 for efficiency. Return u64 to handle large results.
**Concepts:** Loops, mathematical operations, accumulation, formulas

### 78. Largest Palindrome Product

Find the largest palindrome that is the product of two N-digit numbers. For example, for 2-digit numbers, the largest palindrome product is 9009 = 91 × 99. Use nested loops to generate products, check if each is a palindrome by converting to String and comparing with its reverse, and track the maximum.
**Concepts:** Loops, string operations, palindrome checking, optimization

### 79. Special Pythagorean Triplet

Find the unique Pythagorean triplet (a, b, c) where a + b + c = 1000 and a < b < c. Use nested loops to test combinations, checking both the sum condition and the Pythagorean theorem (a² + b² = c²). Return the triplet as (u32, u32, u32) and optionally their product.
**Concepts:** Nested loops, mathematical validation, conditionals

### 80. Simple Substitution Cipher

Implement a customizable substitution cipher where each letter maps to a different letter based on a provided key. The key is a 26-character String representing the substitution alphabet. For encryption, map each letter in the plaintext to the corresponding position in the key. For decryption, reverse the mapping. Preserve case and non-alphabetic characters.
**Concepts:** Strings, character mapping, encryption, HashMap

### 81. Gigasecond Calculator

Calculate the date and time exactly 1 gigasecond (10^9 seconds) after a given date and time. A gigasecond is approximately 31.69 years. Use Rust's datetime libraries or manual calculation: add 1,000,000,000 seconds to the input timestamp, handling day, month, and year overflow correctly.
**Concepts:** Date/time operations, mathematical calculations, time arithmetic

### 82. Reverse String Recursively

Reverse a String using recursion instead of iteration or built-in methods. Base case: empty string or single character returns itself. Recursive case: take the first character, recursively reverse the rest, and append the first character to the end. Demonstrate understanding of recursive thinking and base cases.
**Concepts:** Recursion, strings, base cases, recursive thinking

### 83. Factorial Calculator

Calculate the factorial of a number using both iterative and recursive approaches. Iterative: use a loop with accumulation. Recursive: base case is 0! = 1 or 1! = 1, recursive case is n! = n × (n-1)!. Return u64 or u128 to handle large factorials. Consider adding memoization to the recursive version for efficiency.
**Concepts:** Recursion, loops, mathematical operations, memoization

### 84. Binary Search Implementation

Implement the binary search algorithm to find an element in a sorted Vec<i32>. Return Option<usize> containing the index if found, None otherwise. Maintain left and right pointers, calculate middle index, compare target with middle element, and adjust search range accordingly. Demonstrate O(log n) time complexity understanding.
**Concepts:** Vectors, algorithms, search, indexing, Option

### 85. Simple Grep

Implement a simplified version of the grep command-line utility. Search for a pattern String within text lines (Vec<String>). Return a Vec<String> of matching lines. Support basic options like case-insensitive search, line numbers, and inverted matching (lines that don't match). Optionally support simple regex patterns.
**Concepts:** Strings, pattern matching, iteration, filtering

### 86. Word Wrap

Break text into lines with a maximum line length while preserving word boundaries. Accept a String and max line width (usize). Split text into words, accumulate words into lines until adding the next word would exceed the width, then start a new line. Return Vec<String> where each element is a line.
**Concepts:** Strings, iteration, text formatting, accumulation

### 87. Minesweeper Board

Generate a minesweeper game board with mine counts. Accept a Vec<String> where '\*' represents mines and spaces represent empty cells. For each empty cell, count adjacent mines (including diagonals - all 8 directions). Replace empty cells with their mine counts. Return the completed board as Vec<String>.
**Concepts:** 2D vectors, nested iteration, neighbor counting, directions

### 88. Connect Game Winner

Determine the winner of a Connect/Hex game. The board is represented as Vec<Vec<char>> where 'X' and 'O' are player markers. Player X wins by connecting top to bottom, player O wins by connecting left to right. Use depth-first search or breadth-first search to find if a connected path exists. Return Option<char> for the winner.
**Concepts:** 2D vectors, graph traversal basics, game logic, pathfinding

### 89. Dominoes Chain

Determine if a set of dominoes can form a valid chain where adjacent dominoes have matching numbers. Each domino is represented as a tuple (u8, u8). For the chain to be valid, the second number of each domino must match the first number of the next domino. Use backtracking or recursion to try different arrangements.
**Concepts:** Vectors, tuples, matching logic, backtracking

### 90. Simple State Machine

Implement a finite state machine with defined states and transitions. Define an enum State with variants and an enum Event for triggers. Create a struct StateMachine that holds the current state. Implement a transition() method that accepts an Event and updates the state based on predefined rules. Track and return state history in a Vec<State>.
**Concepts:** Enums, pattern matching, state transitions, state management

### 91. Raindrops

Convert a number to a string based on its factors: if divisible by 3 add "Pling", by 5 add "Plang", by 7 add "Plong". Combine sounds for multiple factors (e.g., 15 → "PlingPlang"). If no factors apply, return the number as a String. Use modulo operations to test divisibility and concatenate results.
**Concepts:** Modulo operations, strings, conditionals, concatenation

### 92. Isogram Checker

Determine if a word or phrase is an isogram (contains no repeating letters). Convert to lowercase, filter to only alphabetic characters, add each to a HashSet<char>, and check if any insertions fail (indicating a duplicate). Return a bool. Spaces and hyphens don't count as repetitions.
**Concepts:** Strings, HashSet, validation, character filtering

### 93. Rotational Cipher (ROT-N)

Implement a generalized version of the Caesar cipher where the rotation amount is variable (ROT-N). Accept a String and rotation amount (i32), then shift each alphabetic character by that amount, wrapping around the alphabet. Preserve case and non-alphabetic characters. ROT-13 is a special case where N=13.
**Concepts:** Strings, character rotation, modulo, wrapping

### 94. Two Bucket Problem

Solve the classic two bucket puzzle: given two buckets with capacities A and B liters, determine the minimum steps to measure exactly G liters. Valid operations: fill a bucket, empty a bucket, pour from one to another. Use breadth-first search to explore states (bucket1_amount, bucket2_amount) and track steps. Return the sequence of operations.
**Concepts:** Algorithms, state management, BFS, problem solving

### 95. Yacht Scoring

Calculate scores for the dice game Yacht (similar to Yahtzee). Given five dice values as Vec<u8> and a category enum (Ones, Twos, FullHouse, FourOfAKind, LittleStraight, BigStraight, Choice, Yacht), calculate the score according to game rules. Use pattern matching to handle each category's scoring logic.
**Concepts:** Vectors, pattern matching, scoring logic, enums

### 96. Linked List Length

Calculate the length of a linked list using recursion. Given a reference to the head Node (Option<Box<Node>>), recursively count nodes: base case is None returns 0, recursive case is 1 + length of the rest. Demonstrate understanding of recursive data structure traversal.
**Concepts:** Recursion, linked lists, counting, Option, Box

### 97. Flatten Nested Array

Flatten a nested array/vector structure into a single-level vector. For example, [1, [2, [3, 4]], 5] becomes [1, 2, 3, 4, 5]. Use recursion to process nested structures: base case handles individual values, recursive case processes Vec contents. Note: This requires enum or generic type to represent mixed data.
**Concepts:** Recursion, vectors, nested structures, enums

### 98. Sublist Checker

Determine the relationship between two lists: equal, sublist, superlist, or unequal. A sublist is a contiguous sequence appearing in another list. Use windows() to check if list A appears anywhere in list B (sublist), if B appears in A (superlist), if they're identical (equal), or neither (unequal). Return an enum Relation.
**Concepts:** Vectors, comparison, enums, iteration, windows

### 99. All Your Base

Convert numbers between arbitrary bases (from base 2 to base 36). Accept input as Vec<u32> of digits in the source base and the target base. First convert to base 10 by multiplying each digit by its positional value, then convert from base 10 to target base by repeated division. Validate that all digits are valid for the input base.
**Concepts:** Mathematical operations, base conversion, validation, vectors

### 100. Custom Set Implementation

Implement a custom Set data structure with basic operations. Store elements in a Vec<T> and implement methods: add(), remove(), contains(), union(), intersection(), difference(), and is_subset(). Ensure uniqueness by checking before adding. Use generic type T that implements PartialEq. Optionally sort the internal vector for efficient operations.
**Concepts:** Vectors, set operations, methods, data structures, generics

---

## Concepts Index

Exercises organized by Rust concepts covered:

### Variables & Mutability

04, 05, 06, 12, 21, 27, 28, 77, 83

### Data Types & Basic Operations

04, 05, 06, 10, 14, 21, 23, 26, 27, 28, 61, 62, 77, 81

### Functions

04, 05, 06, 10, 11, 14, 21, 23, 35, 42, 82, 83, 84

### Control Flow & Conditionals

04, 06, 09, 12, 13, 15, 20, 21, 22, 28, 30, 35, 43, 47, 66, 68, 91

### Loops & Iteration

05, 06, 08, 12, 15, 20, 28, 29, 30, 39, 42, 46, 63, 65, 66, 77, 78, 83

### Ownership & Borrowing

56, 57, 84, 96

### Structs & Methods

14, 24, 38, 49, 52, 55, 56, 57, 70, 74, 90

### Enums & Pattern Matching

11, 22, 23, 35, 36, 43, 44, 45, 52, 70, 90, 95, 98

### Strings & String Manipulation

02, 07, 08, 09, 15, 16, 17, 18, 25, 31, 32, 34, 37, 40, 41, 43, 47, 48, 53, 59, 60, 66, 67, 68, 71, 80, 82, 85, 86, 91, 92, 93

### Vectors & Collections

01, 03, 05, 10, 13, 19, 24, 29, 36, 43, 44, 45, 46, 50, 55, 56, 57, 62, 63, 64, 65, 69, 71, 72, 73, 74, 84, 87, 89, 95, 97, 98, 99, 100

### HashMap & HashSet

01, 03, 08, 19, 31, 32, 34, 48, 51, 53, 57, 69, 72, 80, 92

### Mathematical Operations

04, 06, 10, 14, 21, 23, 26, 27, 28, 38, 39, 42, 54, 61, 62, 63, 75, 76, 77, 78, 79, 81, 83, 99

### Algorithms & Problem Solving

06, 20, 25, 28, 29, 30, 33, 39, 42, 46, 54, 61, 64, 73, 74, 76, 78, 79, 84, 88, 94

### Recursion

56, 82, 83, 96, 97

### Pattern Recognition & Validation

09, 17, 18, 25, 31, 33, 35, 37, 40, 48, 53, 62, 92, 98, 99

### Game Logic & Simulation

12, 22, 49, 70, 88, 90, 95

### Cryptography & Encoding

18, 33, 41, 59, 60, 61, 80, 93

### Data Structures

24, 43, 55, 56, 100

### Text Processing & Formatting

08, 16, 37, 41, 66, 67, 68, 71, 85, 86, 91

### Binary Operations & Bit Manipulation

26, 44, 45, 54

### Sorting & Comparison

01, 10, 17, 35, 57, 70, 98

### Error Handling

11, 40, 53, 62, 84

### Character & UTF-8 Operations

02, 07, 15, 18, 31, 59, 92, 93

### Matrix & 2D Operations

50, 60, 63, 64, 87, 88

### Iterators & Advanced Iteration

08, 19, 29, 40, 41, 51, 58, 72, 98

### Scoring & Counting Systems

08, 15, 34, 53, 95

### Time & Date Operations

38, 81

### Optimization & Dynamic Programming

73, 74

### Graph Traversal

88, 94

---

## Difficulty Progression

### Beginner (Exercises 1-30)

Focus on basic syntax, control flow, simple data types, and fundamental operations. Good for those just starting with Rust.

### Intermediate (Exercises 31-70)

Introduce more complex data structures, algorithms, and Rust-specific concepts like ownership and trait implementation.

### Advanced (Exercises 71-100)

Complex algorithms, advanced data structures, recursion, optimization problems, and multi-concept integration.
