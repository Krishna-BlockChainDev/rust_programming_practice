// ============================================================
// DAY 12: Borrowing & References
// Topic: Borrow values without taking ownership
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. References with & (immutable borrow)
//   2. Mutable references with &mut
//   3. The borrowing rules
//   4. Why borrowing is better than cloning in many cases
//   5. Dangling references (and why Rust prevents them)
//
// BORROWING RULES:
//   Rule 1: You can have MANY immutable references (&T) at a time
//   Rule 2: OR you can have exactly ONE mutable reference (&mut T)
//   Rule 3: NOT both at the same time
//   Rule 4: References must always be valid (no dangling pointers)
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-12 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Basic Reference (&)
    // A reference lets you REFER to a value without owning it
    // Using & to borrow means: no ownership transfer
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Immutable References ===");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // pass a REFERENCE, not s1 itself
    // s1 is still valid because we only borrowed it!
    println!("The length of '{}' is {}.", s1, len);

    // You can have MULTIPLE immutable references at the same time
    let r1 = &s1;
    let r2 = &s1;
    let r3 = &s1;
    println!("r1={}, r2={}, r3={} — all valid at once!", r1, r2, r3);

    // --------------------------------------------------------
    // EXERCISE 2: Mutable Reference (&mut)
    // If you want to modify a borrowed value, use &mut
    // RULE: Only ONE mutable reference allowed at a time!
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Mutable References ===");

    let mut s = String::from("hello");
    println!("Before: {}", s);

    change(&mut s);  // pass a mutable reference
    println!("After change: {}", s);

    // You can modify through a mutable reference:
    let mut greeting = String::from("Good");
    {
        let r = &mut greeting;
        r.push_str(" Morning!");
        println!("Via mutable ref: {}", r);
    }  // r goes out of scope here — mutable borrow ends
    println!("greeting after scope: {}", greeting);

    // --------------------------------------------------------
    // EXERCISE 3: Borrowing Rules in Action
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Borrowing Rules ===");

    let mut data = String::from("Rust");

    // This works — ONE mutable reference:
    let m1 = &mut data;
    m1.push_str(" is great!");
    println!("m1: {}", m1);
    // m1 is no longer used after this point

    // Now we can create another mutable reference:
    let m2 = &mut data;
    m2.push_str(" Let's learn it!");
    println!("m2: {}", m2);

    // The following would be a COMPILE ERROR (uncomment to see):
    // let m3 = &mut data;
    // let m4 = &mut data;  // ERROR: cannot borrow as mutable more than once!
    // println!("{} {}", m3, m4);

    // --------------------------------------------------------
    // EXERCISE 4: Immutable and Mutable references
    // You CANNOT have a mutable ref while immutable refs are active
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Mixed References ===");

    let mut s = String::from("hello");
    let r1 = &s;     // immutable borrow #1
    let r2 = &s;     // immutable borrow #2 — OK!
    println!("r1 = {}, r2 = {}", r1, r2);
    // r1 and r2 are no longer used after this point (Non-Lexical Lifetimes)

    // Now we can create a mutable reference (r1 and r2 are "done"):
    let r3 = &mut s;
    r3.push_str(" world");
    println!("r3 = {}", r3);

    // --------------------------------------------------------
    // EXERCISE 5: References in functions — the big advantage!
    // Use references to avoid moving AND avoid cloning
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: References in Functions ===");

    let sentence = String::from("Rust programming is fun!");

    // Pass by reference — sentence is NOT moved, still usable after
    let word_count = count_words(&sentence);
    let char_count = count_chars(&sentence);
    let has_rust   = contains_word(&sentence, "Rust");

    // We can still use sentence here because we only borrowed it!
    println!("Sentence:   {}", sentence);
    println!("Words:      {}", word_count);
    println!("Characters: {}", char_count);
    println!("Has 'Rust': {}", has_rust);

    // --------------------------------------------------------
    // EXERCISE 6: Mutable reference in a function
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Mutate via Reference ===");

    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("Before sort: {:?}", numbers);

    sort_vec(&mut numbers);  // pass mutable reference
    println!("After sort:  {:?}", numbers);

    // --------------------------------------------------------
    // EXERCISE 7: Dereferencing with *
    // To access the value behind a reference, use *
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Dereferencing ===");

    let x = 10;
    let r = &x;      // r is a reference to x
    println!("x = {}", x);
    println!("r = {}", r);      // Rust auto-derefs for println
    println!("*r = {}", *r);    // Manual dereference

    let mut val = 5;
    let ref_val = &mut val;
    *ref_val += 10;   // Dereference and modify!
    println!("val after dereference-modify: {}", val);

    // --------------------------------------------------------
    // EXERCISE 8: Comparison of approaches
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: Move vs Clone vs Borrow ===");

    let original = String::from("Hello, World!");

    // Option A: MOVE (original becomes invalid)
    // let moved = original;   ← DON'T do this if you need original

    // Option B: CLONE (expensive but keeps both valid)
    let cloned = original.clone();
    println!("Original: {}", original);
    println!("Cloned:   {}", cloned);

    // Option C: BORROW (cheapest, no data copy, original still valid)
    let borrowed = &original;
    println!("Borrowed: {}", borrowed);
    println!("Original still valid: {}", original);  // Yes!

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write a function `first_word(s: &String) -> &str`
    // that returns a reference to the first word of the string
    // fn first_word(s: &String) -> &str {
    //     let bytes = s.as_bytes();
    //     for (i, &byte) in bytes.iter().enumerate() {
    //         if byte == b' ' { return &s[0..i]; }
    //     }
    //     &s[..]
    // }

    // Challenge 2: Write a function that takes two &str references
    // and returns the longer one as a &str
    // fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    //     if s1.len() > s2.len() { s1 } else { s2 }
    // }

    // Challenge 3: Predict which of these will compile. Why?
    // let mut s = String::from("test");
    // let r1 = &s;
    // let r2 = &mut s;  // ERROR or OK?
    // println!("{} {}", r1, r2);

    // Challenge 4: Write a function that takes a &mut Vec<i32>
    // and doubles every element in place
    // fn double_all(v: &mut Vec<i32>) { for x in v { *x *= 2; } }

    // --------------------------------------------------------
    // SUMMARY:
    //   &T        -> immutable reference (borrow, read-only)
    //   &mut T    -> mutable reference (borrow, can modify)
    //   fn f(s: &String) -> use reference, s is borrowed
    //   fn f(s: &mut String) -> mutable borrow
    //   Rules:
    //     - Many immutable refs allowed at once
    //     - Only ONE mutable ref at a time
    //     - No mix of mutable + immutable at same time
    //   *ref      -> dereference (access value behind reference)
    //   Benefit:  Borrow instead of clone → cheaper, no copy
    // --------------------------------------------------------
}

// ── Helper Functions ─────────────────────────────────────────

fn calculate_length(s: &String) -> usize {
    s.len()  // s is borrowed — we don't own it, so we can't drop it
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn count_words(s: &String) -> usize {
    s.split_whitespace().count()
}

fn count_chars(s: &String) -> usize {
    s.chars().count()
}

fn contains_word(s: &String, word: &str) -> bool {
    s.contains(word)
}

fn sort_vec(v: &mut Vec<i32>) {
    v.sort();
}
