// ============================================================
// DAY 11: Ownership Basics
// Topic: Rust's most unique and important feature!
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. The 3 Rules of Ownership
//   2. Move semantics (what happens when you assign)
//   3. Clone — making deep copies
//   4. Copy types (primitives copy automatically)
//   5. Ownership and functions (passing values)
//
// WHY OWNERSHIP?
//   Rust has NO garbage collector, NO manual malloc/free
//   Instead, ownership rules guarantee memory safety at COMPILE TIME
//   This is what makes Rust fast AND safe!
//
// THE 3 RULES OF OWNERSHIP:
//   Rule 1: Every value has exactly ONE owner
//   Rule 2: When the owner goes out of scope, the value is DROPPED (freed)
//   Rule 3: There can only be one owner at a time
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-11 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Basic Ownership — Scope
    // When a variable goes out of scope, Rust calls `drop`
    // automatically to free the memory
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Scope & Ownership ===");

    {
        let s = String::from("hello");  // s is created, owns the String
        println!("Inside scope: {}", s);
    }  // <-- s goes out of scope HERE, memory is automatically freed!

    // println!("{}", s); // ERROR! s is no longer valid here

    // Primitives work fine:
    {
        let x = 5;
        println!("x in inner scope: {}", x);
    }
    // Note: primitives like integers are on the STACK, they are COPIED
    // so we don't worry about them as much

    // --------------------------------------------------------
    // EXERCISE 2: Move Semantics
    // When you assign a heap-allocated value (like String) to
    // another variable, the OWNERSHIP MOVES — the original is invalid!
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Move Semantics ===");

    let s1 = String::from("hello");
    let s2 = s1;  // s1 is MOVED to s2. s1 is no longer valid!

    // println!("{}", s1); // This would be a COMPILE ERROR:
                           // "value borrowed here after move"
    println!("s2 (after move from s1): {}", s2);
    println!("(s1 is no longer valid after the move)");

    // --------------------------------------------------------
    // EXERCISE 3: Clone — make a deep copy
    // If you WANT two separate copies, use .clone()
    // This is more expensive (copies heap data) but keeps both valid
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Clone ===");

    let original = String::from("Hello, Rust!");
    let copy = original.clone();  // Deep copy — both are valid!

    println!("original: {}", original);  // still valid
    println!("copy:     {}", copy);       // also valid

    // Modifying the copy doesn't affect original
    let mut copy2 = original.clone();
    copy2.push_str(" (modified)");
    println!("original: {}", original);
    println!("copy2:    {}", copy2);

    // --------------------------------------------------------
    // EXERCISE 4: Copy Types
    // Simple scalar types stored on the STACK are automatically COPIED
    // They don't need .clone()
    // Types that are Copy: i32, u32, f64, bool, char, tuples of Copy types
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Copy Types (primitives) ===");

    let a = 42;
    let b = a;  // `a` is COPIED, not moved — both are still valid!
    println!("a = {} (still valid!)", a);
    println!("b = {}", b);

    let x = true;
    let y = x;  // bool is Copy too
    println!("x = {} (still valid!)", x);
    println!("y = {}", y);

    let c = 'A';
    let d = c;  // char is Copy too
    println!("c = {} (still valid!)", c);
    println!("d = {}", d);

    // Tuples of Copy types are also Copy:
    let t1 = (1, 2, 3);
    let t2 = t1;  // Copied!
    println!("t1 = {:?}", t1);
    println!("t2 = {:?}", t2);

    // --------------------------------------------------------
    // EXERCISE 5: Ownership and Functions
    // Passing a value to a function MOVES it (for heap types)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Ownership in Functions ===");

    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("Function got ownership of: {}", some_string);
    }  // some_string goes out of scope and is DROPPED here

    fn makes_copy(some_integer: i32) {
        // some_integer is COPIED here — no ownership transfer
        println!("Function got copy of: {}", some_integer);
    }

    let my_string = String::from("hello ownership");
    takes_ownership(my_string);  // my_string MOVES into the function
    // println!("{}", my_string); // ERROR: value was moved!

    let my_number = 10;
    makes_copy(my_number);      // my_number is COPIED
    println!("my_number is still valid: {}", my_number); // still fine!

    // --------------------------------------------------------
    // EXERCISE 6: Returning Ownership
    // Functions can give ownership back by returning values
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Returning Ownership ===");

    fn gives_ownership() -> String {
        let s = String::from("mine now!");
        s  // returned — ownership moves to caller
    }

    fn takes_and_gives_back(s: String) -> String {
        // takes ownership, then gives it back
        println!("Inside function: {}", s);
        s  // return it — ownership goes back to caller
    }

    let s1 = gives_ownership();  // s1 gets ownership from function
    println!("s1: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);  // s2 moved in, s3 gets it back
    // println!("{}", s2); // ERROR: s2 was moved
    println!("s3: {}", s3);  // s3 is valid

    // --------------------------------------------------------
    // EXERCISE 7: Moving in loops
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Ownership in loops ===");

    let names = vec!["Alice", "Bob", "Charlie"];  // Vec of &str (Copy type)
    for name in &names {  // iterate by REFERENCE to avoid moving
        println!("Hello, {}!", name);
    }
    // names is still valid because we used &names
    println!("names still valid: {:?}", names);

    // --------------------------------------------------------
    // CONCEPT SUMMARY DIAGRAM:
    //
    //   Stack types (i32, bool, char, f32, etc.)
    //   ─────────────────────────────────────────
    //   let a = 5;
    //   let b = a;  ← COPY — both a and b are valid ✓
    //
    //   Heap types (String, Vec, etc.)
    //   ─────────────────────────────────────────
    //   let s1 = String::from("hi");
    //   let s2 = s1;    ← MOVE — s1 is invalid ✗
    //   let s3 = s1.clone(); ← CLONE — s1 still valid ✓
    //
    // --------------------------------------------------------

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Predict what happens — which lines would cause errors?
    // (Uncomment each block ONE AT A TIME to test)
    //
    // Block A:
    // let s1 = String::from("rust");
    // let s2 = s1;
    // println!("{} {}", s1, s2);  // Does this work? Why/why not?
    //
    // Block B:
    // let x = 100;
    // let y = x;
    // println!("{} {}", x, y);  // Does this work? Why/why not?
    //
    // Block C:
    // let s = String::from("ownership");
    // let len = calculate_len(s);  // imagine a function that takes s
    // println!("{}", s);  // Does this work after passing to function?

    // Challenge 2: Write a function `double_string(s: String) -> String`
    // that takes ownership of a String, doubles it, and returns it
    // fn double_string(s: String) -> String {
    //     let doubled = s.clone() + &s;
    //     doubled
    // }
    // let result = double_string(String::from("hello"));
    // println!("{}", result); // "hellohello"

    // Challenge 3: Explain in comments: WHY does Rust use ownership?
    // What problems does it prevent?

    // --------------------------------------------------------
    // SUMMARY:
    //   Rule 1: Each value has ONE owner
    //   Rule 2: Owner out of scope → value is dropped (freed)
    //   Rule 3: Only ONE owner at a time
    //   let b = a;  → MOVE for heap types (a invalid after)
    //   let b = a;  → COPY for stack types (a still valid)
    //   .clone()    → deep copy (both variables valid)
    //   Passing to function → moves ownership (for heap types)
    //   Returning from function → gives ownership to caller
    // --------------------------------------------------------
}
