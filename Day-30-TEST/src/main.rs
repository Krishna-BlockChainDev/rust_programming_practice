// ============================================================
// DAY 30 - PHASE 3 TEST: Error Handling & Collections
// Self-Assessment: Days 21-30
// Time: 45-60 minutes
// ============================================================
//
// INSTRUCTIONS:
//   - Try to solve each question WITHOUT looking at previous days
//   - Only check Day-21 to Day-30 files if you are truly stuck
//   - Write ALL solutions inside fn main() or as separate functions
//   - Run with: cargo run
//
// SCORING:
//   Q1  (10 pts) — Result<T,E>
//   Q2  (10 pts) — Custom Error type
//   Q3  (10 pts) — ? operator
//   Q4  (10 pts) — Iterators
//   Q5  (10 pts) — HashMap
//   Q6  (10 pts) — HashSet
//   Q7  (10 pts) — Closures
//   Q8  (10 pts) — File I/O
//   Q9  (10 pts) — String processing
//   Q10 (10 pts) — Combined challenge
//   TOTAL: 100 pts
//
// PASS: 70+ pts | GREAT: 85+ pts | EXCELLENT: 95+ pts
// ============================================================

use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    println!("=== Phase 3 Self-Test: Error Handling & Collections ===\n");

    // ─────────────────────────────────────────────────────────
    // Q1 (10 pts): RESULT<T, E>
    // Write a function: fn safe_divide(a: f64, b: f64) -> Result<f64, String>
    // It should return Err("Cannot divide by zero".to_string()) if b == 0.0
    // Otherwise return Ok(a / b)
    // Test it with: safe_divide(10.0, 2.0) and safe_divide(5.0, 0.0)
    // Print the result in each case using match
    // ─────────────────────────────────────────────────────────
    println!("--- Q1: Result<T, E> ---");
    // TODO: Write your solution here




    // ─────────────────────────────────────────────────────────
    // Q2 (10 pts): CUSTOM ERROR TYPE
    // Create a custom error enum: MathError with variants:
    //   DivisionByZero
    //   NegativeSquareRoot
    //   Overflow
    // Implement fmt::Display for MathError showing a message for each variant
    // Write fn sqrt_positive(n: f64) -> Result<f64, MathError>
    // that returns NegativeSquareRoot error if n < 0.0 else Ok(n.sqrt())
    // ─────────────────────────────────────────────────────────
    println!("\n--- Q2: Custom Error Type ---");
    // TODO: Write your solution here (define enum above main, implement here)




    // ─────────────────────────────────────────────────────────
    // Q3 (10 pts): ? OPERATOR
    // Write a function: fn read_and_parse(filename: &str) -> Result<i32, Box<dyn std::error::Error>>
    // It should:
    //   1. Read a file using fs::read_to_string(filename)?
    //   2. Parse the contents as i32 using .trim().parse::<i32>()?
    //   3. Return the parsed number
    // Create a file "number.txt" with content "42" and test your function
    // Handle the case where the file doesn't exist gracefully
    // ─────────────────────────────────────────────────────────
    println!("\n--- Q3: ? Operator ---");
    // TODO: Write your solution here




    // ─────────────────────────────────────────────────────────
    // Q4 (10 pts): ITERATORS
    // Given: let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // Use iterator methods (NO loops allowed) to:
    //   a) Collect all even numbers into a Vec<i32>
    //   b) Compute the sum of squares of odd numbers (1²+3²+5²+7²+9² = 165)
    //   c) Create a Vec of strings: ["1 is odd", "2 is even", ...]
    //   d) Find the first number greater than 7
    // ─────────────────────────────────────────────────────────
    println!("\n--- Q4: Iterators ---");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // TODO: Write your solution here




    // ─────────────────────────────────────────────────────────
    // Q5 (10 pts): HASHMAP
    // Write fn count_chars(s: &str) -> HashMap<char, usize>
    // that counts the frequency of each character (ignore spaces)
    // Test with: "hello world"
    // Then: print the characters sorted by frequency (highest first)
    // ─────────────────────────────────────────────────────────
    println!("\n--- Q5: HashMap ---");
    // TODO: Write your solution here




    // ─────────────────────────────────────────────────────────
    // Q6 (10 pts): HASHSET
    // Given two Vec<i32>:
    //   let a = vec![1, 2, 3, 4, 5, 6];
    //   let b = vec![4, 5, 6, 7, 8, 9];
    // Using HashSet operations, find and print:
    //   a) Numbers in both (intersection)
    //   b) Numbers only in 'a' (difference a - b)
    //   c) Numbers only in 'b' (difference b - a)
    //   d) All unique numbers from both (union)
    // ─────────────────────────────────────────────────────────
    println!("\n--- Q6: HashSet ---");
    let a = vec![1, 2, 3, 4, 5, 6];
    let b = vec![4, 5, 6, 7, 8, 9];
    // TODO: Write your solution here




    // ─────────────────────────────────────────────────────────
    // Q7 (10 pts): CLOSURES
    // a) Write a closure 'multiplier' that takes a factor: i32
    //    and returns a closure that multiplies its input by that factor
    //    (i.e., a closure that returns a closure)
    //    Usage: let triple = multiplier(3); triple(5) == 15
    //
    // b) Given: let words = vec!["apple", "banana", "cherry", "date", "elderberry"];
    //    Use a closure to: filter words longer than 5 chars, uppercase them,
    //    then collect into Vec<String>
    // ─────────────────────────────────────────────────────────
    println!("\n--- Q7: Closures ---");
    // TODO: Write your solution here




    // ─────────────────────────────────────────────────────────
    // Q8 (10 pts): FILE I/O
    // a) Create a file "scores.txt" and write 5 lines, each with:
    //    "Name Score" e.g., "Alice 95"
    // b) Read the file back
    // c) Parse each line and store as (String, i32) tuples in a Vec
    // d) Find and print the student with the highest score
    // e) Clean up: delete the file
    // ─────────────────────────────────────────────────────────
    println!("\n--- Q8: File I/O ---");
    // TODO: Write your solution here




    // ─────────────────────────────────────────────────────────
    // Q9 (10 pts): STRING PROCESSING
    // Given: let sentence = "  The quick brown fox jumps over the lazy dog  ";
    // a) Trim and print the sentence
    // b) Count the number of words
    // c) Reverse the entire string
    // d) Check if it contains the word "fox"
    // e) Replace "fox" with "cat" and "dog" with "mouse"
    // f) Collect all unique words into a sorted Vec<String>
    // ─────────────────────────────────────────────────────────
    println!("\n--- Q9: String Processing ---");
    let sentence = "  The quick brown fox jumps over the lazy dog  ";
    // TODO: Write your solution here




    // ─────────────────────────────────────────────────────────
    // Q10 (10 pts): COMBINED CHALLENGE
    // Build a simple "Student Grade Book":
    //
    // Data: Use a HashMap<String, Vec<i32>> where:
    //       key = student name, value = list of scores
    //
    // Add these students and their scores:
    //   "Alice"  => [85, 90, 78, 92, 88]
    //   "Bob"    => [70, 65, 80, 75, 68]
    //   "Carol"  => [95, 98, 92, 97, 100]
    //   "David"  => [55, 60, 58, 62, 50]
    //
    // Then:
    //   1. Calculate average score per student
    //   2. Assign grades: A(90+), B(80+), C(70+), D(60+), F(below 60)
    //   3. Print a formatted report card for each student
    //   4. Find and print the top student
    //   5. Find students who are failing (grade F)
    //   6. Write the report to a file "report_card.txt"
    // ─────────────────────────────────────────────────────────
    println!("\n--- Q10: Combined Challenge — Grade Book ---");
    // TODO: Write your solution here




    println!("\n=== Phase 3 Test Complete! Check your answers. ===");
    println!("Revisit Day-21 to Day-30 for any questions you struggled with.");
}

// ── Helper functions — define them here below main ───────────

// TODO: Define safe_divide here
// fn safe_divide(a: f64, b: f64) -> Result<f64, String> { ... }

// TODO: Define MathError enum here
// TODO: impl fmt::Display for MathError

// TODO: Define sqrt_positive here

// TODO: Define read_and_parse here

// TODO: Define count_chars here
