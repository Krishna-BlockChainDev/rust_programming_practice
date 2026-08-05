// ============================================================
// DAY 22: The ? Operator — Propagating Errors
// Topic: Cleaner error handling with ?
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. The ? operator — what it does
//   2. Using ? to propagate errors up the call stack
//   3. Functions that return Result with ?
//   4. ? with Option<T>
//   5. Chaining ? operations
//
// KEY CONCEPT:
//   The ? operator is a shortcut for "if this is Err, return Err immediately"
//   It can ONLY be used in functions that return Result or Option
//
//   Instead of:
//     let val = match something() {
//         Ok(v)  => v,
//         Err(e) => return Err(e),
//     };
//   You write:
//     let val = something()?;
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-22 folder)
//
// ============================================================

use std::num::ParseIntError;
use std::fmt;

// ── Custom Error Type ─────────────────────────────────────────
#[derive(Debug)]
enum AppError {
    ParseError(ParseIntError),
    NegativeNumber(i32),
    DivisionByZero,
    InvalidInput(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::ParseError(e)       => write!(f, "Parse error: {}", e),
            AppError::NegativeNumber(n)   => write!(f, "Negative number not allowed: {}", n),
            AppError::DivisionByZero      => write!(f, "Cannot divide by zero"),
            AppError::InvalidInput(msg)   => write!(f, "Invalid input: {}", msg),
        }
    }
}

// Implement From<ParseIntError> so ? can auto-convert
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> AppError {
        AppError::ParseError(e)
    }
}

// ── Functions using ? ─────────────────────────────────────────

// Without ?: verbose match
fn parse_without_question_mark(s: &str) -> Result<i32, AppError> {
    let n = match s.parse::<i32>() {
        Ok(v)  => v,
        Err(e) => return Err(AppError::ParseError(e)),
    };
    Ok(n)
}

// With ?: clean and concise
fn parse_with_question_mark(s: &str) -> Result<i32, AppError> {
    let n = s.parse::<i32>()?;  // ? auto-returns Err if parse fails
    Ok(n)
}

// Chaining ? operations:
fn parse_and_sqrt(s: &str) -> Result<f64, AppError> {
    let n = s.parse::<i32>()?;          // ? on parse
    if n < 0 {
        return Err(AppError::NegativeNumber(n));
    }
    Ok((n as f64).sqrt())
}

// Multiple ? in one function:
fn complex_calculation(a: &str, b: &str) -> Result<i32, AppError> {
    let x = a.parse::<i32>()?;   // parse first number
    let y = b.parse::<i32>()?;   // parse second number
    if y == 0 {
        return Err(AppError::DivisionByZero);
    }
    Ok(x / y)
}

// ? with Option (returns None early):
fn get_first_char(s: &str) -> Option<char> {
    let c = s.chars().next()?;  // ? on Option — returns None if no first char
    Some(c.to_ascii_uppercase())
}

fn find_and_double(v: &[i32], index: usize) -> Option<i32> {
    let value = v.get(index)?;   // ? returns None if index OOB
    Some(value * 2)
}

// ─────────────────────────────────────────────────────────────

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: ? vs match comparison
    // --------------------------------------------------------
    println!("=== EXERCISE 1: ? vs match ===");

    let inputs = ["42", "abc", "-7", "100"];
    for input in inputs {
        let with_match = parse_without_question_mark(input);
        let with_q     = parse_with_question_mark(input);
        println!("  '{}': match={:?}  ?={:?}", input, with_match, with_q);
    }

    // --------------------------------------------------------
    // EXERCISE 2: Chaining ? operations
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Chaining ? ===");

    let test_cases = ["25", "16", "-4", "abc", "0", "81"];
    for tc in test_cases {
        match parse_and_sqrt(tc) {
            Ok(result) => println!("  sqrt('{}') = {:.4}", tc, result),
            Err(e)     => println!("  sqrt('{}') = ERROR: {}", tc, e),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 3: Multiple ? in one function
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Multiple ? ===");

    let pairs = [("100", "4"), ("abc", "2"), ("10", "0"), ("20", "3"), ("15", "xyz")];
    for (a, b) in pairs {
        match complex_calculation(a, b) {
            Ok(r)  => println!("  {} / {} = {}", a, b, r),
            Err(e) => println!("  {} / {} = ERROR: {}", a, b, e),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 4: ? with Option
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: ? with Option ===");

    let words = ["hello", "", "rust", " "];
    for word in words {
        match get_first_char(word) {
            Some(c) => println!("  '{}' first char: '{}'", word, c),
            None    => println!("  '{}' has no characters!", word),
        }
    }

    let numbers = vec![10, 20, 30, 40, 50];
    for idx in [0, 2, 4, 10] {
        match find_and_double(&numbers, idx) {
            Some(v) => println!("  numbers[{}] * 2 = {}", idx, v),
            None    => println!("  Index {} out of bounds", idx),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 5: A more realistic pipeline with ?
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Error Propagation Pipeline ===");

    fn validate_username(name: &str) -> Result<&str, AppError> {
        if name.is_empty() {
            return Err(AppError::InvalidInput(String::from("Username cannot be empty")));
        }
        if name.len() < 3 {
            return Err(AppError::InvalidInput(format!("'{}' is too short (min 3 chars)", name)));
        }
        if name.contains(' ') {
            return Err(AppError::InvalidInput(format!("'{}' cannot contain spaces", name)));
        }
        Ok(name)
    }

    fn validate_age(s: &str) -> Result<u32, AppError> {
        let age = s.parse::<u32>()?;
        if age < 1 || age > 120 {
            return Err(AppError::InvalidInput(format!("Age {} out of range 1-120", age)));
        }
        Ok(age)
    }

    fn create_user(name: &str, age_str: &str) -> Result<String, AppError> {
        let valid_name = validate_username(name)?;
        let valid_age  = validate_age(age_str)?;
        Ok(format!("User created: {} (age {})", valid_name, valid_age))
    }

    let test_users = [
        ("Krishna", "25"),
        ("",        "25"),
        ("Al",      "25"),
        ("Alice",   "abc"),
        ("Bob",     "200"),
        ("Jo hn",   "30"),
        ("Carol",   "28"),
    ];

    for (name, age) in test_users {
        match create_user(name, age) {
            Ok(msg)  => println!("  ✓ {}", msg),
            Err(e)   => println!("  ✗ Error: {}", e),
        }
    }

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write a function `read_two_nums(a: &str, b: &str) -> Result<(i32,i32), AppError>`
    // that parses both strings using ?, and returns them as a tuple

    // Challenge 2: Write a function `sum_strings(inputs: &[&str]) -> Result<i32, AppError>`
    // that parses all strings and returns their sum, failing on first error

    // Challenge 3: Modify complex_calculation to also check if result is negative
    // and return Err(AppError::NegativeNumber(result)) in that case

    // Challenge 4: Write a function using ? that:
    //   1. Parses a string to i32
    //   2. Checks it's positive
    //   3. Checks it's even
    //   4. Returns it doubled
    //   Use multiple ? operations and custom error returns

    // --------------------------------------------------------
    // SUMMARY:
    //   fn f() -> Result<T, E> { let v = something()?; Ok(v) }
    //   ?  on Result  → returns Err early if Err, else unwraps Ok
    //   ?  on Option  → returns None early if None, else unwraps Some
    //   Can ONLY use ? in functions returning Result or Option
    //   Chaining: a()?; b()?; c()?; Ok(result)
    //   From<E> trait needed for auto-conversion of error types
    //   Much cleaner than writing match for every operation!
    // --------------------------------------------------------
}
