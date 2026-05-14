# Pig Latin Converter

Source: <https://doc.rust-lang.org/book/ch08-03-hash-maps.html>

### Objective

Convert strings to Pig Latin. Move the first consonant to the end and add "ay" (e.g., "first" becomes "irst-fay"). For words starting with a vowel, add "hay" to the end (e.g., "apple" becomes "apple-hay").

### Step-by-Step

1. [ ] Define a list or a string containing vowels (a, e, i, o, u) to check against.
2. [ ] Iterate through each word in the input string.
3. [ ] Check the first character of the word. Handle UTF-8 safely by using `.chars()`.
4. [ ] Apply the transformation rule:
   - If it's a vowel: Append "-hay" to the word.
   - If it's a consonant: Move the first char to the end and append "-fay".
5. [ ] Collect the transformed words back into a single string separated by spaces.

### How to test

Run the following command in your terminal:
`cargo test -p pig_latin`
