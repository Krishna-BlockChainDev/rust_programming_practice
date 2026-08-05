// ============================================================
// DAY 13: Slices
// Topic: References to a contiguous part of a collection
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. String slices (&str)
//   2. Array slices (&[T])
//   3. Slice ranges (start..end, start..=end, ..end, start..)
//   4. Using slices in functions
//   5. first_word() — a classic slice example
//
// KEY INSIGHT:
//   A slice is a REFERENCE to a portion of data.
//   It does NOT own the data — it just "points" to part of it.
//   Slices let you work with parts of strings/arrays
//   without copying the data!
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-13 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: String Slices (&str)
    // &str is literally a slice — a reference to part of a String
    // String literals ("hello") are also &str slices into program binary
    // --------------------------------------------------------
    println!("=== EXERCISE 1: String Slices ===");

    let s = String::from("Hello, World!");

    // Slice ranges: [start_index..end_index]  (end is EXCLUSIVE)
    let hello = &s[0..5];    // "Hello"  (bytes 0,1,2,3,4)
    let world = &s[7..12];   // "World"  (bytes 7,8,9,10,11)
    let comma = &s[5..6];    // ","

    println!("Full string: {}", s);
    println!("hello slice: {}", hello);
    println!("world slice: {}", world);
    println!("comma slice: {}", comma);

    // Shorthand forms:
    let from_start = &s[..5];    // same as &s[0..5]  = "Hello,"
    let to_end     = &s[7..];    // from index 7 to end = "World!"
    let entire     = &s[..];     // the whole string as a slice

    println!("from_start: {}", from_start);
    println!("to_end:     {}", to_end);
    println!("entire:     {}", entire);

    // --------------------------------------------------------
    // EXERCISE 2: String literals ARE slices
    // When you write "hello", Rust stores it in the binary
    // and gives you a &str pointing to it
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: String Literals as &str ===");

    let literal: &str = "I am a string literal";  // stored in binary, immutable
    let owned: String = String::from("I am an owned String");

    // You can get a &str from a String by using & or &[..]
    let slice_of_owned: &str = &owned;
    let partial: &str = &owned[0..5];  // "I am "

    println!("literal:        {}", literal);
    println!("owned:          {}", owned);
    println!("slice_of_owned: {}", slice_of_owned);
    println!("partial:        {}", partial);

    // --------------------------------------------------------
    // EXERCISE 3: first_word function (classic example!)
    // Returns a &str slice pointing into the original string
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: first_word() ===");

    let sentence = String::from("hello world from Rust");
    let word = first_word(&sentence);
    println!("Sentence:   '{}'", sentence);
    println!("First word: '{}'", word);

    let one_word = String::from("Rustacean");
    println!("One word:   '{}'", first_word(&one_word));

    // --------------------------------------------------------
    // EXERCISE 4: Array Slices (&[T])
    // Same concept — a reference to part of an array or vector
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Array Slices ===");

    let numbers = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    let first_three  = &numbers[0..3];   // [10, 20, 30]
    let middle       = &numbers[3..7];   // [40, 50, 60, 70]
    let last_three   = &numbers[7..];    // [80, 90, 100]
    let all_of_it    = &numbers[..];     // whole array as slice

    println!("Full array:   {:?}", numbers);
    println!("first_three:  {:?}", first_three);
    println!("middle:       {:?}", middle);
    println!("last_three:   {:?}", last_three);
    println!("all_of_it:    {:?}", all_of_it);

    // --------------------------------------------------------
    // EXERCISE 5: Slices in functions
    // Functions that accept slices work with both arrays and Vecs!
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Slice Functions ===");

    let arr = [5, 2, 8, 1, 9, 3, 7];
    let vec_data: Vec<i32> = vec![15, 42, 7, 23, 61, 8];

    // The same function works for both!
    println!("Array sum:     {}", sum_slice(&arr));
    println!("Vec sum:       {}", sum_slice(&vec_data));
    println!("Array max:     {}", max_slice(&arr));
    println!("Vec max:       {}", max_slice(&vec_data));

    // Passing a slice OF an array:
    let partial_sum = sum_slice(&arr[1..5]);  // only elements 1-4
    println!("Partial sum (index 1-4): {}", partial_sum);

    // --------------------------------------------------------
    // EXERCISE 6: Mutable slices
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Mutable Slice ===");

    let mut data = [1, 2, 3, 4, 5];
    println!("Before: {:?}", data);

    double_slice(&mut data[1..4]);  // mutate elements at index 1,2,3
    println!("After doubling index 1-3: {:?}", data);

    // --------------------------------------------------------
    // EXERCISE 7: Iterating slices
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Iterating Slices ===");

    let scores: &[u32] = &[78, 92, 85, 67, 95, 88];

    print!("Scores: ");
    for score in scores {
        print!("{} ", score);
    }
    println!();

    // Using enumerate on a slice
    for (i, score) in scores.iter().enumerate() {
        println!("  Student {}: {}", i + 1, score);
    }

    // --------------------------------------------------------
    // EXERCISE 8: last_word() function
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: last_word() ===");

    let sentence2 = String::from("The quick brown fox");
    println!("Sentence:  '{}'", sentence2);
    println!("Last word: '{}'", last_word(&sentence2));

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write a function `nth_word(s: &str, n: usize) -> &str`
    // that returns the nth word (1-based) in a sentence
    // (Use split_whitespace().nth(n-1).unwrap_or(""))

    // Challenge 2: Write a function `count_vowels(s: &str) -> usize`
    // that counts vowels (a,e,i,o,u) in a string slice

    // Challenge 3: Write a function `is_sorted(slice: &[i32]) -> bool`
    // that returns true if the slice is in ascending order

    // Challenge 4: Given a Vec<String>, get a slice of only the first 3
    // and pass it to a function that prints each one

    // --------------------------------------------------------
    // SUMMARY:
    //   &s[0..5]       -> string slice (bytes 0 to 4)
    //   &s[..5]        -> from start to index 4
    //   &s[5..]        -> from index 5 to end
    //   &s[..]         -> entire string as slice
    //   &arr[1..4]     -> array slice (elements 1,2,3)
    //   &str           -> string slice type
    //   &[T]           -> slice of type T
    //   Slices borrow — they don't own the data
    //   Functions with &str/&[T] params work with
    //     both owned types AND slices
    // --------------------------------------------------------
}

// ── Helper Functions ─────────────────────────────────────────

/// Returns a &str pointing to the first word in the string
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];  // return slice up to the space
        }
    }
    &s[..]  // No space found — the whole string is one word
}

/// Returns a &str pointing to the last word
fn last_word(s: &str) -> &str {
    s.split_whitespace().last().unwrap_or("")
}

/// Sum all elements in a slice
fn sum_slice(slice: &[i32]) -> i32 {
    let mut total = 0;
    for &x in slice {
        total += x;
    }
    total
}

/// Find max element in a slice
fn max_slice(slice: &[i32]) -> i32 {
    let mut max = slice[0];
    for &x in slice {
        if x > max { max = x; }
    }
    max
}

/// Double each element in a mutable slice
fn double_slice(slice: &mut [i32]) {
    for x in slice {
        *x *= 2;
    }
}
