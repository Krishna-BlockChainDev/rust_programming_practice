// ============================================================
// DAY 4: String Types
// Topic: Working with text in Rust — String vs &str
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. &str  — string slice (fixed, borrowed, usually literal)
//   2. String — owned, growable, heap-allocated string
//   3. Creating and converting between them
//   4. Basic string operations (length, contains, replace, etc.)
//   5. String formatting with format!
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-4 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: &str — String Slice (string literal)
    // This is a reference to text stored in the program binary
    // It is IMMUTABLE — you cannot change it
    // Written with double quotes: "hello"
    // --------------------------------------------------------
    let greeting: &str = "Hello, World!";
    let language: &str = "Rust";
    println!("Greeting: {}", greeting);
    println!("Language: {}", language);

    // --------------------------------------------------------
    // EXERCISE 2: String — Owned String
    // String::from() creates an owned, heap-allocated string
    // It is GROWABLE — you can add text to it
    // --------------------------------------------------------
    let mut owned_string: String = String::from("Hello");
    println!("\nOwned string: {}", owned_string);

    // Push a single character
    owned_string.push('!');
    println!("After push('!'): {}", owned_string);

    // Push a string slice
    owned_string.push_str(" Welcome to Rust.");
    println!("After push_str: {}", owned_string);

    // --------------------------------------------------------
    // EXERCISE 3: Creating Strings different ways
    // --------------------------------------------------------
    let s1 = String::from("Hello");           // From a literal
    let s2 = "World".to_string();             // .to_string() method
    let s3 = String::new();                   // Empty string
    let s4 = format!("{} {}", s1, s2);        // Using format! macro

    println!("\ns1: {}", s1);
    println!("s2: {}", s2);
    println!("s3 (empty): '{}'", s3);
    println!("s4 (formatted): {}", s4);

    // --------------------------------------------------------
    // EXERCISE 4: String Concatenation
    // Option A: + operator (moves s1!)
    // Option B: format! macro (safer, doesn't move)
    // --------------------------------------------------------
    let first = String::from("Hello");
    let second = String::from(" World");

    // Option A: + operator (note: first is MOVED here)
    let combined = first + &second; // &second converts to &str
    println!("\nCombined with +: {}", combined);
    // Note: `first` is no longer valid here (it was moved)

    // Option B: format! (preferred — keeps all variables valid)
    let a = String::from("Good");
    let b = String::from("Morning");
    let c = format!("{} {}!", a, b);
    println!("Formatted: {}", c);
    println!("a still valid: {}", a); // a is still usable!
    println!("b still valid: {}", b); // b is still usable!

    // --------------------------------------------------------
    // EXERCISE 5: String Methods
    // --------------------------------------------------------
    let sentence = String::from("  Hello, Rust Programming!  ");

    // Length (number of bytes, not characters)
    println!("\nLength: {}", sentence.len());

    // Check if empty
    println!("Is empty: {}", sentence.is_empty());

    // Trim whitespace from both ends
    let trimmed = sentence.trim();
    println!("Trimmed: '{}'", trimmed);

    // Check if contains a word
    println!("Contains 'Rust': {}", sentence.contains("Rust"));
    println!("Contains 'Java': {}", sentence.contains("Java"));

    // Convert to uppercase/lowercase
    println!("Uppercase: {}", trimmed.to_uppercase());
    println!("Lowercase: {}", trimmed.to_lowercase());

    // Replace text
    let replaced = trimmed.replace("Rust", "Rust 🦀");
    println!("Replaced: {}", replaced);

    // Starts with / ends with
    println!("Starts with 'Hello': {}", trimmed.starts_with("Hello"));
    println!("Ends with '!': {}", trimmed.ends_with('!'));

    // --------------------------------------------------------
    // EXERCISE 6: Converting &str <-> String
    // --------------------------------------------------------
    let str_slice: &str = "I am a slice";
    let owned: String = str_slice.to_string();     // &str → String
    let back_to_slice: &str = &owned;              // String → &str

    println!("\nstr_slice: {}", str_slice);
    println!("owned:     {}", owned);
    println!("back:      {}", back_to_slice);

    // --------------------------------------------------------
    // EXERCISE 7: String Splitting
    // --------------------------------------------------------
    let csv = "name,age,city,country";
    let parts: Vec<&str> = csv.split(',').collect();

    println!("\nCSV split result:");
    for part in &parts {
        println!("  - {}", part);
    }

    println!("Number of parts: {}", parts.len());

    // --------------------------------------------------------
    // EXERCISE 8: Reading user input (bonus!)
    // Rust can read from terminal with std::io
    // --------------------------------------------------------
    // Uncomment this section to try interactive input:
    //
    // use std::io;
    // let mut input = String::new();
    // println!("Enter your name: ");
    // io::stdin().read_line(&mut input).expect("Failed to read");
    // let input = input.trim();  // remove newline
    // println!("Hello, {}!", input);

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Create a String with your full name
    // then print its length and uppercase version
    // let full_name = String::from("___");
    // println!("Name: {}", full_name);
    // println!("Length: {}", full_name.len());
    // println!("Upper: {}", full_name.to_uppercase());

    // Challenge 2: Join first and last name using format!
    // let first_name = "Krishna";
    // let last_name = "Rajput";
    // let full = format!("{} {}", first_name, last_name);
    // println!("Full name: {}", full);

    // Challenge 3: Take the sentence "i love rust programming"
    // and print it with first letter capitalized using to_uppercase
    // Hint: This is tricky! Look up .chars().next()

    // Challenge 4: Count words in a sentence
    // let sentence = "the quick brown fox";
    // let word_count = sentence.split_whitespace().count();
    // println!("Word count: {}", word_count);

    // --------------------------------------------------------
    // SUMMARY:
    //   &str               -> immutable string slice (literal)
    //   String             -> owned, mutable, growable string
    //   String::from("..") -> create owned string
    //   "..".to_string()   -> convert &str to String
    //   &my_string         -> borrow String as &str
    //   s.push_str("..")   -> append string to String
    //   s.push('c')        -> append char to String
    //   format!("{}", ..)  -> format into new String
    //   s.len()            -> number of bytes
    //   s.trim()           -> remove whitespace
    //   s.contains("x")   -> check if contains substring
    //   s.replace("a","b") -> replace text
    //   s.split(',')       -> split by delimiter
    // --------------------------------------------------------
}
