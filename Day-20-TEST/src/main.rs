// ╔══════════════════════════════════════════════════════════════╗
// ║         🦀  RUST LEARNING — PHASE 2 TEST                    ║
// ║              Days 11 to 20 — Self Assessment                ║
// ╚══════════════════════════════════════════════════════════════╝
//
// INSTRUCTIONS:
//   - Read each question carefully
//   - Write your answer by replacing TODO comments
//   - DO NOT look at previous day files while solving
//   - Run with: cargo run  (inside Day-20-TEST folder)
//
// TOPICS COVERED:
//   ✅ Day 11 — Ownership, move, copy, clone
//   ✅ Day 12 — References & borrowing (&, &mut)
//   ✅ Day 13 — Slices (&str, &[T])
//   ✅ Day 14 — Structs (defining, creating, field access)
//   ✅ Day 15 — impl blocks, methods, associated functions
//   ✅ Day 16 — Enums (variants, data in variants)
//   ✅ Day 17 — Pattern matching (match, if let, while let)
//   ✅ Day 18 — Option<T> (Some, None, methods)
//   ✅ Day 19 — Vec<T> (push, pop, iterate, sort)
//   ✅ Day 20 — Combined project concepts
//
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// SCORING:
//   Q1-Q5   → Ownership & Borrowing
//   Q6-Q8   → Slices
//   Q9-Q13  → Structs & impl
//   Q14-Q17 → Enums & Matching
//   Q18-Q21 → Option<T>
//   Q22-Q25 → Vectors
//   Q26-Q28 → Challenge (combined concepts)
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

// ── Struct & Enum Definitions (you will add more below) ───────
// Add your structs and enums HERE (above fn main)
// Q9, Q10, Q11, Q14, Q26 require definitions here




// ─────────────────────────────────────────────────────────────

fn main() {
    println!("╔══════════════════════════════════════════╗");
    println!("║     RUST PHASE 2 TEST — Days 11-20      ║");
    println!("╚══════════════════════════════════════════╝\n");

    // ══════════════════════════════════════════════════════
    // SECTION A: OWNERSHIP & BORROWING (Days 11-12)
    // ══════════════════════════════════════════════════════

    // ─────────────────────────────────────────────────────
    // Q1. [Day 11] Will the code below compile? Why?
    //     Fix it so BOTH s1 and s2 are valid after the assignment.
    //     Expected: print both "hello" strings successfully.
    // ─────────────────────────────────────────────────────
    println!("--- Q1: Ownership / Clone ---");
    let s1 = String::from("hello");
    let s2 = s1; // FIX THIS: after this line, both s1 and s2 should be usable
    // TODO: Fix the line above so both work, then uncomment:
    // println!("s1 = {}, s2 = {}", s1, s2);


    // ─────────────────────────────────────────────────────
    // Q2. [Day 11] Copy types:
    //     Declare x = 100 (i32). Assign y = x.
    //     Print BOTH x and y. Explain in a comment why this works.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q2: Copy Types ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q3. [Day 12] Pass a String to the function `get_length`
    //     defined below WITHOUT moving it. After the call,
    //     print both the original string and its length.
    //     Expected:
    //       String: "Rust Programming"
    //       Length: 16
    // ─────────────────────────────────────────────────────
    println!("\n--- Q3: Immutable Reference ---");
    let text = String::from("Rust Programming");
    // TODO: Call get_length() using a reference and print both


    // ─────────────────────────────────────────────────────
    // Q4. [Day 12] Use a mutable reference:
    //     Given `message = String::from("Hello")`
    //     Call the function `append_world` (defined below)
    //     to modify it. Print the result.
    //     Expected: "Hello, World!"
    // ─────────────────────────────────────────────────────
    println!("\n--- Q4: Mutable Reference ---");
    let mut message = String::from("Hello");
    // TODO: Call append_world(&mut message) then print message


    // ─────────────────────────────────────────────────────
    // Q5. [Day 12] Borrowing rules — PREDICT the output:
    //     Looking at this code, explain what error would occur
    //     and WHY. Write your explanation as a comment.
    //     (Do NOT uncomment — just explain)
    //
    //     let mut s = String::from("test");
    //     let r1 = &s;
    //     let r2 = &s;
    //     let r3 = &mut s;  // Would this cause an error?
    //     println!("{} {} {}", r1, r2, r3);
    // ─────────────────────────────────────────────────────
    println!("\n--- Q5: Borrowing Rules (comment answer) ---");
    // TODO: Write your explanation as a comment here


    // ══════════════════════════════════════════════════════
    // SECTION B: SLICES (Day 13)
    // ══════════════════════════════════════════════════════

    // ─────────────────────────────────────────────────────
    // Q6. [Day 13] Given: s = "Hello World Rust"
    //     a) Get a slice of "Hello"
    //     b) Get a slice of "Rust" (last 4 chars)
    //     c) Get a slice of "World"
    //     Print all three slices.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q6: String Slices ---");
    let s = "Hello World Rust";
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q7. [Day 13] Array slice:
    //     Given arr = [10, 20, 30, 40, 50, 60, 70]
    //     a) Get a slice of the first 3 elements
    //     b) Get a slice of elements at index 2, 3, 4
    //     c) Call sum_of_slice() (defined below) with the full array
    //     Print results.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q7: Array Slices ---");
    let arr = [10, 20, 30, 40, 50, 60, 70];
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q8. [Day 13] Call the function `first_word_of`
    //     (defined below) with "Rust is amazing!" and print the result.
    //     Expected output: First word: "Rust"
    // ─────────────────────────────────────────────────────
    println!("\n--- Q8: Slice Function ---");
    // TODO: Call first_word_of("Rust is amazing!") and print result


    // ══════════════════════════════════════════════════════
    // SECTION C: STRUCTS & METHODS (Days 14-15)
    // ══════════════════════════════════════════════════════

    // ─────────────────────────────────────────────────────
    // Q9. [Day 14] Define a struct `Book` (above main) with:
    //       title: String
    //       author: String
    //       year: u32
    //       price: f64
    //     Create an instance and print all fields.
    //     Also print it with {:?} (requires #[derive(Debug)])
    // ─────────────────────────────────────────────────────
    println!("\n--- Q9: Struct Definition & Instance ---");
    // TODO: Create a Book instance and print it
    // (Remember to define the struct ABOVE main!)


    // ─────────────────────────────────────────────────────
    // Q10. [Day 15] Add an impl block to `Book` (above main) with:
    //      a) Associated function `new(title, author, year, price) -> Book`
    //      b) Method `discounted_price(&self, percent: f64) -> f64`
    //         that returns price after discount
    //      c) Method `is_affordable(&self) -> bool` — true if price < 500.0
    //      Create a book using `Book::new(...)` and test all methods.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q10: impl Methods ---");
    // TODO: Use Book::new() and call the methods
    // Example: let b = Book::new("Rust Book", "Klabnik", 2019, 799.0);
    // println!("Price after 20% discount: {}", b.discounted_price(20.0));


    // ─────────────────────────────────────────────────────
    // Q11. [Day 14] Struct update syntax:
    //      Create a second book based on Q10's book but with
    //      a different price (₹499.0). Use struct update syntax.
    //      Print both books.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q11: Struct Update Syntax ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q12. [Day 14] Tuple struct:
    //      Define a tuple struct `Rgb(u8, u8, u8)` (above main).
    //      Create red, green, blue colors using it.
    //      Print each using .0, .1, .2 fields.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q12: Tuple Struct ---");
    // TODO: Create Rgb instances and print them


    // ─────────────────────────────────────────────────────
    // Q13. [Day 15] Mutable method:
    //      Add a method `apply_discount(&mut self, percent: f64)`
    //      to Book (in the impl block above main) that permanently
    //      reduces the price.
    //      Create a mutable book, apply 10% discount, print before & after.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q13: Mutable Method ---");
    // TODO: Write your answer below


    // ══════════════════════════════════════════════════════
    // SECTION D: ENUMS & PATTERN MATCHING (Days 16-17)
    // ══════════════════════════════════════════════════════

    // ─────────────────────────────────────────────────────
    // Q14. [Day 16] Define an enum `Weather` (above main) with:
    //        Sunny, Rainy, Cloudy, Snowy, Windy
    //      Add a method `description(&self) -> &str`
    //      that returns a description for each variant.
    //      Create different weather values and print descriptions.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q14: Basic Enum ---");
    // TODO: Create Weather variants and print descriptions


    // ─────────────────────────────────────────────────────
    // Q15. [Day 16] Define an enum `Shape` (above main) with:
    //        Circle(f64)          -- radius
    //        Rectangle(f64, f64)  -- width, height
    //        Triangle(f64, f64, f64) -- three sides
    //      Add a method `area(&self) -> f64`
    //      Create one of each and print name + area.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q15: Enum with Data ---");
    // TODO: Create Shape instances and print their areas


    // ─────────────────────────────────────────────────────
    // Q16. [Day 17] Write a match expression for an i32 `score`:
    //      score = 73
    //      Match ranges:
    //        90-100 → "Excellent"
    //        75-89  → "Good"
    //        60-74  → "Average"
    //        _      → "Needs Improvement"
    //      Print the category.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q16: match with Ranges ---");
    let score = 73;
    // TODO: Write the match expression


    // ─────────────────────────────────────────────────────
    // Q17. [Day 17] Use `if let` to:
    //      Given: opt_val: Option<i32> = Some(42)
    //      If it has a value, print "Value is: X"
    //      Otherwise print "No value"
    //      Then do the same with None.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q17: if let with Option ---");
    let opt_val: Option<i32> = Some(42);
    // TODO: Use if let


    // ══════════════════════════════════════════════════════
    // SECTION E: OPTION<T> (Day 18)
    // ══════════════════════════════════════════════════════

    // ─────────────────────────────────────────────────────
    // Q18. [Day 18] Call the functions below and handle results:
    //      a) find_max(&[3, 7, 1, 9, 4]) → print the max
    //      b) find_max(&[]) → print "No max found (empty)"
    //      Use match or if let.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q18: Option from Function ---");
    // TODO: Call find_max and handle the Option result


    // ─────────────────────────────────────────────────────
    // Q19. [Day 18] Use unwrap_or:
    //      Given: name: Option<&str> = None
    //      Print the name, or "Anonymous" if None.
    //      Also: score: Option<i32> = Some(88)
    //      Print the score or 0 if None.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q19: unwrap_or ---");
    let name: Option<&str> = None;
    let score_opt: Option<i32> = Some(88);
    // TODO: Print using unwrap_or


    // ─────────────────────────────────────────────────────
    // Q20. [Day 18] Use map() on Option:
    //      Given: length: Option<u32> = Some(10)
    //      Use map() to create: area: Option<u32> = Some(100) (length squared)
    //      Print both.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q20: Option map() ---");
    let length: Option<u32> = Some(10);
    // TODO: Use map() to compute area


    // ─────────────────────────────────────────────────────
    // Q21. [Day 18] Write a function `safe_sqrt(n: f64) -> Option<f64>`
    //      below main. Returns None if n < 0, Some(sqrt) otherwise.
    //      Call it with 25.0, -4.0, 0.0 and print results.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q21: Function returning Option ---");
    // TODO: Call safe_sqrt and print results


    // ══════════════════════════════════════════════════════
    // SECTION F: VECTORS (Day 19)
    // ══════════════════════════════════════════════════════

    // ─────────────────────────────────────────────────────
    // Q22. [Day 19] Create a Vec<String> of 5 programming languages.
    //      a) Print original list
    //      b) Sort alphabetically
    //      c) Print sorted list
    //      d) Remove the last element and print what was removed
    //      e) Insert "Rust" at index 1
    //      f) Print final list
    // ─────────────────────────────────────────────────────
    println!("\n--- Q22: Vec Operations ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q23. [Day 19] Given scores = vec![78, 92, 65, 88, 71, 95, 50]:
    //      a) Calculate and print the sum
    //      b) Calculate and print the average (f64, 2 decimal places)
    //      c) Find and print max and min using iter methods
    //      d) Filter scores >= 75 into a new Vec and print it
    // ─────────────────────────────────────────────────────
    println!("\n--- Q23: Vec Statistics ---");
    let scores_vec = vec![78, 92, 65, 88, 71, 95, 50];
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q24. [Day 19] Mutable Vec iteration:
    //      Given: let mut nums = vec![1, 2, 3, 4, 5];
    //      Use a for loop with &mut to triple every element.
    //      Print before and after.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q24: Mutable Vec Iteration ---");
    let mut nums = vec![1, 2, 3, 4, 5];
    // TODO: Triple every element using &mut iteration


    // ─────────────────────────────────────────────────────
    // Q25. [Day 19] Vec of structs:
    //      Create a Vec of 3 Book instances (using Book::new from Q10).
    //      Sort them by price (ascending).
    //      Print each book's title and price.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q25: Vec of Structs ---");
    // TODO: Write your answer below


    // ══════════════════════════════════════════════════════
    // SECTION G: CHALLENGE QUESTIONS (Days 11-20 combined)
    // ══════════════════════════════════════════════════════

    // ─────────────────────────────────────────────────────
    // Q26. [CHALLENGE] Define a struct `Player` (above main) with:
    //        name: String, level: u32, score: i64, alive: bool
    //      Add impl with:
    //        new(name, level) -> Player  (score=0, alive=true)
    //        add_score(&mut self, points: i64)
    //        take_damage(&mut self, amount: i64)  -- if score < 0, set alive=false
    //        status(&self) -> &str  -- "Alive" or "Dead"
    //      Create 2 players, give them scores/damage, print their status.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q26: Struct + impl Challenge ---");
    // TODO: Create Player instances and call methods


    // ─────────────────────────────────────────────────────
    // Q27. [CHALLENGE] Pattern matching with enum + Vec:
    //      Define enum `Task` (above main) with:
    //        Todo(String)
    //        InProgress(String, u32)  -- name, % complete
    //        Done(String)
    //      Create a Vec<Task> with at least one of each.
    //      Use a for loop with match to print each task's status.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q27: Enum + Vec + match ---");
    // TODO: Write your answer below


    // ─────────────────────────────────────────────────────
    // Q28. [CHALLENGE] Option + Vec:
    //      Given a Vec<Option<i32>> = [Some(1), None, Some(3), None, Some(5)]
    //      a) Count how many are Some and how many are None
    //      b) Collect all Some values into a Vec<i32>
    //      c) Sum the Some values
    //      Print all results.
    // ─────────────────────────────────────────────────────
    println!("\n--- Q28: Option + Vec Challenge ---");
    let mixed: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    // TODO: Write your answer below


    println!("\n╔══════════════════════════════════════════╗");
    println!("║     PHASE 2 TEST COMPLETE! Great job 🦀  ║");
    println!("╚══════════════════════════════════════════╝");
}

// ══════════════════════════════════════════════════════════════
// HELPER FUNCTIONS — implement these (called from main above)
// ══════════════════════════════════════════════════════════════

// For Q3: Takes a &String, returns its length
fn get_length(s: &String) -> usize {
    // TODO: Implement
    0  // placeholder
}

// For Q4: Takes &mut String and appends ", World!"
fn append_world(s: &mut String) {
    // TODO: Implement
}

// For Q7: Returns the sum of a slice of i32
fn sum_of_slice(s: &[i32]) -> i32 {
    // TODO: Implement
    0  // placeholder
}

// For Q8: Returns the first word of a string slice
fn first_word_of(s: &str) -> &str {
    // TODO: Implement
    // Hint: iterate bytes, find first space, return slice up to that point
    ""  // placeholder
}

// For Q18: Returns Some(max) or None if the slice is empty
fn find_max(nums: &[i32]) -> Option<i32> {
    // TODO: Implement
    None  // placeholder
}

// For Q21: Returns Some(sqrt(n)) or None if n < 0
fn safe_sqrt(n: f64) -> Option<f64> {
    // TODO: Implement
    None  // placeholder
}

// ══════════════════════════════════════════════════════════════
// HOW TO VERIFY YOUR ANSWERS:
//   1. Run: cargo run
//   2. If it compiles with no errors → great start!
//   3. Check each section's printed output for correctness
//   4. Look at Day 11-20 files to verify your understanding
//
// EXPECTED: All helper functions should be fully implemented.
// ══════════════════════════════════════════════════════════════
