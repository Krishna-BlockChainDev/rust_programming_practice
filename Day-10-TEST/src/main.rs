// ╔══════════════════════════════════════════════════════════════╗
// ║           🦀  RUST LEARNING — PHASE 1 TEST                  ║
// ║               Days 1 to 10 — Self Assessment                ║
// ╚══════════════════════════════════════════════════════════════╝
//
// INSTRUCTIONS:
//   - Read each question carefully
//   - Write your answer by replacing the TODO comments
//   - DO NOT look at previous day files while solving (test yourself!)
//   - After finishing, you can verify your answers by running:
//       $ cargo run
//   - A correct solution will compile and print expected output
//
// TOPICS COVERED:
//   ✅ Day 1  — println!, print!, placeholders, comments
//   ✅ Day 2  — Variables, mutability, const, shadowing
//   ✅ Day 3  — Data types (int, float, bool, char)
//   ✅ Day 4  — Strings (&str, String, methods)
//   ✅ Day 5  — Operators (arithmetic, comparison, logical)
//   ✅ Day 6  — if / else if / else
//   ✅ Day 7  — Loops (loop, while, for, break, continue)
//   ✅ Day 8  — Functions (parameters, return values)
//   ✅ Day 9  — Tuples and Arrays
//   ✅ Day 10 — Mini project concepts
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// SCORING GUIDE (self-check after completing):
//   Q1-Q5   → Easy     (Day 1-2 basics)
//   Q6-Q12  → Medium   (Day 3-6 logic)
//   Q13-Q18 → Medium+  (Day 7-8 functions & loops)
//   Q19-Q23 → Hard     (Day 9-10 compound types)
//   Q24-Q25 → Challenge (full program logic)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║     RUST PHASE 1 TEST — Days 1-10       ║");
    println!("╚══════════════════════════════════════════╝\n");

    // ══════════════════════════════════════════════════════
    // SECTION A: BASICS — println, Variables, Types
    // ══════════════════════════════════════════════════════

    // ─────────────────────────────────────────────────────
    // Q1. [Day 1] Print your full name and the text
    //     "I am learning Rust!" on TWO separate lines.
    //     Expected output:
    //       Krishna Rajput
    //       I am learning Rust!
    // ─────────────────────────────────────────────────────
    println!("--- Q1: Print name and message ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q2. [Day 1] Use ONE println! to print:
    //     "My name is Krishna and I am 25 years old."
    //     Use {} placeholders — do NOT hardcode the sentence.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q2: Placeholders ---");
    let name = "Krishna";
    let age = 25;
    // TODO: Use name and age variables with println! and {}


    // ─────────────────────────────────────────────────────
    // Q3. [Day 2] Declare an immutable variable `city` with
    //     value "Bangalore". Then declare a mutable variable
    //     `population` with value 12_000_000. Change the
    //     population to 13_500_000, then print both.
    //     Expected output:
    //       City: Bangalore
    //       Population: 13500000
    // ─────────────────────────────────────────────────────
    println!("\n--- Q3: Variables and Mutability ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q4. [Day 2] Demonstrate SHADOWING:
    //     Start with x = 5
    //     Shadow x to be x * 2 (now 10)
    //     Shadow x again to be x + 3 (now 13)
    //     Print x at the end.
    //     Expected output: Final x: 13
    // ─────────────────────────────────────────────────────
    println!("\n--- Q4: Shadowing ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q5. [Day 2] Declare a constant MAX_SCORE of type u32
    //     with value 100. Print it.
    //     Expected output: Max Score: 100
    // ─────────────────────────────────────────────────────
    println!("\n--- Q5: Constants ---");
    // TODO: Declare const here (outside or inside main, your choice)
    // TODO: Print it


    // ─────────────────────────────────────────────────────
    // Q6. [Day 3] Declare variables for each type below and
    //     print them all:
    //       - a signed 32-bit integer: -42
    //       - an unsigned 8-bit integer: 200
    //       - a 64-bit float: 3.14159
    //       - a boolean: true
    //       - a character: 'R'
    // ─────────────────────────────────────────────────────
    println!("\n--- Q6: Data Types ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q7. [Day 3] Type casting:
    //     Start with f: f64 = 9.99
    //     Cast it to i32 and store in n.
    //     Print both f and n.
    //     Expected output:
    //       f64 value: 9.99
    //       After cast to i32: 9
    // ─────────────────────────────────────────────────────
    println!("\n--- Q7: Type Casting ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q8. [Day 4] String operations:
    //     Create a String from "  Hello, Rust!  "
    //     Then:
    //       a) Print trimmed version
    //       b) Print uppercase version (of trimmed)
    //       c) Print whether it contains "Rust"
    //       d) Print its length (after trimming)
    // ─────────────────────────────────────────────────────
    println!("\n--- Q8: String Operations ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q9. [Day 4] Use format! macro to build a sentence:
    //     Given first = "Rust" and second = "Programming"
    //     Create combined = "Rust Programming" using format!
    //     Then print it.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q9: String Formatting ---");
    let first = "Rust";
    let second = "Programming";
    // TODO: Use format! to combine them and print


    // ─────────────────────────────────────────────────────
    // Q10. [Day 5] Calculate and print each of the following
    //      using variables a=17 and b=5:
    //      a) a + b
    //      b) a - b
    //      c) a * b
    //      d) a / b  (integer division)
    //      e) a % b  (remainder)
    //      f) a as f64 / b as f64  (float division, 2 decimal places)
    // ─────────────────────────────────────────────────────
    println!("\n--- Q10: Arithmetic Operators ---");
    let a = 17;
    let b = 5;
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q11. [Day 5] Use compound assignment operators:
    //      Start: score = 50
    //      Add 20  → score
    //      Multiply by 2  → score
    //      Subtract 10  → score
    //      Divide by 3  → score
    //      Print score after EACH step.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q11: Compound Assignment ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q12. [Day 6] Write an if-else chain that:
    //      Given temperature = 38 (in Celsius), prints:
    //        < 0    → "Freezing"
    //        0-10   → "Very Cold"
    //        11-20  → "Cold"
    //        21-30  → "Pleasant"
    //        31-40  → "Hot"
    //        > 40   → "Extreme Heat"
    // ─────────────────────────────────────────────────────
    println!("\n--- Q12: Temperature Classifier ---");
    let temperature = 38;
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q13. [Day 6] Use `if` as an EXPRESSION:
    //      Given number = 91:
    //      Assign label = "odd" or "even" using if expression
    //      Print: "91 is odd" or "91 is even"
    // ─────────────────────────────────────────────────────
    println!("\n--- Q13: if as Expression ---");
    let number = 91;
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q14. [Day 7] Use a `for` loop to:
    //      Print all numbers from 1 to 10 that are
    //      divisible by 3 OR divisible by 5.
    //      Expected output: 3 5 6 9 10
    // ─────────────────────────────────────────────────────
    println!("\n--- Q14: for Loop with Condition ---");
    // TODO: Write your answer below
    println!(); // for newline after output


    // ─────────────────────────────────────────────────────
    // Q15. [Day 7] Use a `while` loop to find the first
    //      number greater than 1 whose square exceeds 200.
    //      Print: "First number whose square > 200 is: X"
    //      (Answer should be 15, since 15^2 = 225 > 200)
    // ─────────────────────────────────────────────────────
    println!("\n--- Q15: while Loop ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q16. [Day 7] Use a `loop` with `break` to:
    //      Count from 1, keep doubling, and stop when
    //      the value exceeds 1000. Return and print the value.
    //      Expected: the first power of 2 that exceeds 1000 (1024)
    // ─────────────────────────────────────────────────────
    println!("\n--- Q16: loop with break and return value ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q17. [Day 7] Use `continue` in a for loop:
    //      Loop from 1 to 20, skip multiples of 4,
    //      print all others on one line.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q17: continue in loop ---");
    // TODO: Write your answer below
    println!(); // newline after output


    // ─────────────────────────────────────────────────────
    // Q18. [Day 8] Call the functions defined BELOW main():
    //      a) Call greet("Krishna") — should print a greeting
    //      b) Call calculate_area(6.0, 4.5) and print the result
    //      c) Call classify_number(-7) and print the result
    // ─────────────────────────────────────────────────────
    println!("\n--- Q18: Calling Functions ---");
    // TODO: Call greet("Krishna")

    // TODO: Call calculate_area(6.0, 4.5), store result, print it

    // TODO: Call classify_number(-7), store result, print it


    // ─────────────────────────────────────────────────────
    // Q19. [Day 9] Tuples:
    //      Create a tuple `book` with:
    //        title: "The Alchemist" (&str)
    //        author: "Paulo Coelho" (&str)
    //        year: 1988 (u32)
    //        price: 299.0 (f64)
    //      a) Access each field using .0, .1, .2, .3 and print
    //      b) Destructure the tuple into 4 variables and print
    // ─────────────────────────────────────────────────────
    println!("\n--- Q19: Tuples ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q20. [Day 9] Arrays:
    //      Create an array of 6 integers: [15, 3, 9, 27, 6, 18]
    //      a) Print the full array using {:?}
    //      b) Print first and last element
    //      c) Calculate and print the sum of all elements
    //      d) Sort the array and print it
    // ─────────────────────────────────────────────────────
    println!("\n--- Q20: Arrays ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q21. [Day 9] Array iteration with enumerate:
    //      Given weekdays = ["Mon","Tue","Wed","Thu","Fri"]
    //      Print each with its number:
    //        1. Mon
    //        2. Tue
    //        ... etc.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q21: Array with enumerate ---");
    let weekdays = ["Mon", "Tue", "Wed", "Thu", "Fri"];
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q22. [Day 9] Slices:
    //      Given numbers = [10, 20, 30, 40, 50, 60, 70, 80]
    //      a) Print a slice of the first 3 elements
    //      b) Print a slice of the last 3 elements
    //      c) Print a slice of elements at index 2, 3, 4
    // ─────────────────────────────────────────────────────
    println!("\n--- Q22: Array Slices ---");
    let numbers = [10, 20, 30, 40, 50, 60, 70, 80];
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q23. [Day 5 + Day 6] Logical operators:
    //      Given: year = 2024
    //      Check if it's a leap year using this rule:
    //        divisible by 4 AND (NOT divisible by 100 OR divisible by 400)
    //      Print: "2024 is a leap year: true/false"
    // ─────────────────────────────────────────────────────
    println!("\n--- Q23: Leap Year Check ---");
    let year = 2024;
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q24. [CHALLENGE — Days 7+8] FizzBuzz extended:
    //      Call the function fizzbuzz_range(1, 30) defined
    //      below main. It should print for numbers 1-30:
    //        divisible by 3 & 5 → "FizzBuzz"
    //        divisible by 3 only → "Fizz"
    //        divisible by 5 only → "Buzz"
    //        otherwise → the number itself
    // ─────────────────────────────────────────────────────
    println!("\n--- Q24: FizzBuzz Extended ---");
    // TODO: Call fizzbuzz_range(1, 30)


    // ─────────────────────────────────────────────────────
    // Q25. [CHALLENGE — All concepts]
    //      Call the function student_report defined below.
    //      It takes an array of 5 test scores [u32; 5]
    //      and prints:
    //        - All scores
    //        - Sum
    //        - Average (as f64, 2 decimal places)
    //        - Highest score
    //        - Lowest score
    //        - Grade based on average (A/B/C/D/F)
    //      Call it with scores: [88, 72, 95, 60, 78]
    // ─────────────────────────────────────────────────────
    println!("\n--- Q25: Student Report ---");
    // TODO: Call student_report([88, 72, 95, 60, 78])


    println!("\n╔══════════════════════════════════════════╗");
    println!("║         TEST COMPLETE! Good job 🦀       ║");
    println!("╚══════════════════════════════════════════╝");
}

// ══════════════════════════════════════════════════════════════
// FUNCTION STUBS — YOU MUST IMPLEMENT THESE FUNCTIONS BELOW
// These are used by Q18, Q24, Q25
// ══════════════════════════════════════════════════════════════

// ─────────────────────────────────────────────────────────────
// For Q18a: greet(name)
// Should print: "Hello, {name}! Welcome to Rust programming."
// ─────────────────────────────────────────────────────────────
fn greet(name: &str) {
    // TODO: Implement this function
    // println!("Hello, {}! ...", name);
}

// ─────────────────────────────────────────────────────────────
// For Q18b: calculate_area(width, height) -> f64
// Returns the area: width * height
// ─────────────────────────────────────────────────────────────
fn calculate_area(width: f64, height: f64) -> f64 {
    // TODO: Implement and return the area
    0.0 // placeholder — replace this
}

// ─────────────────────────────────────────────────────────────
// For Q18c: classify_number(n: i32) -> &'static str
// Returns:
//   "Positive Even" if n > 0 and even
//   "Positive Odd"  if n > 0 and odd
//   "Negative Even" if n < 0 and even
//   "Negative Odd"  if n < 0 and odd
//   "Zero"          if n == 0
// ─────────────────────────────────────────────────────────────
fn classify_number(n: i32) -> &'static str {
    // TODO: Implement using if/else
    "" // placeholder — replace this
}

// ─────────────────────────────────────────────────────────────
// For Q24: fizzbuzz_range(start: u32, end: u32)
// Prints FizzBuzz for numbers from start to end (inclusive)
// ─────────────────────────────────────────────────────────────
fn fizzbuzz_range(start: u32, end: u32) {
    // TODO: Implement using a for loop
}

// ─────────────────────────────────────────────────────────────
// For Q25: student_report(scores: [u32; 5])
// Prints full report: all scores, sum, average, min, max, grade
// ─────────────────────────────────────────────────────────────
fn student_report(scores: [u32; 5]) {
    // TODO: Implement this function
    // Hints:
    //   - Use a for loop to find sum, min, max
    //   - Average = sum as f64 / 5.0
    //   - Grade: >=90→A, >=80→B, >=70→C, >=60→D, else→F
}

// ══════════════════════════════════════════════════════════════
// END OF TEST FILE
//
// After completing all questions:
//   1. Run: cargo run
//   2. If it compiles and output looks correct → you passed!
//   3. Compare your logic with Day 1-10 exercise files
//      to verify your understanding.
//
// EXPECTED: This file should compile with 0 errors when done.
// HINT: Start with Q1 and go in order. Don't skip questions!
// ══════════════════════════════════════════════════════════════
