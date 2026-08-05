// ============================================================
// DAY 23: Custom Error Types
// Topic: Define your own error types for better error handling
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Why custom errors are better than String errors
//   2. Defining error enums
//   3. Implementing Display trait for errors
//   4. Implementing From trait for error conversion
//   5. Using Box<dyn Error> for flexible error handling
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-23 folder)
//
// ============================================================

use std::fmt;
use std::num::ParseIntError;

// ── Custom Error Types ────────────────────────────────────────

// A simple custom error enum
#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeSquareRoot(f64),
    Overflow,
    UnderRange { value: i32, min: i32 },
}

// Implement Display — this controls how errors are printed
impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::DivisionByZero              => write!(f, "Cannot divide by zero"),
            MathError::NegativeSquareRoot(n)       => write!(f, "Cannot take sqrt of negative: {}", n),
            MathError::Overflow                    => write!(f, "Arithmetic overflow"),
            MathError::UnderRange { value, min }   => write!(f, "Value {} is below minimum {}", value, min),
        }
    }
}

// Implement std::error::Error (required to be a proper error type)
impl std::error::Error for MathError {}

// ── Another error type ────────────────────────────────────────

#[derive(Debug)]
enum ValidationError {
    EmptyField(String),
    TooShort { field: String, min_len: usize },
    TooLong  { field: String, max_len: usize },
    InvalidFormat(String),
    OutOfRange { field: String, value: i32, min: i32, max: i32 },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValidationError::EmptyField(field) =>
                write!(f, "Field '{}' cannot be empty", field),
            ValidationError::TooShort { field, min_len } =>
                write!(f, "Field '{}' must be at least {} characters", field, min_len),
            ValidationError::TooLong { field, max_len } =>
                write!(f, "Field '{}' cannot exceed {} characters", field, max_len),
            ValidationError::InvalidFormat(msg) =>
                write!(f, "Invalid format: {}", msg),
            ValidationError::OutOfRange { field, value, min, max } =>
                write!(f, "Field '{}' value {} is outside range [{}, {}]", field, value, min, max),
        }
    }
}

impl std::error::Error for ValidationError {}

// ── Combined Error (multiple error types) ────────────────────

#[derive(Debug)]
enum AppError {
    Math(MathError),
    Validation(ValidationError),
    Parse(ParseIntError),
    Io(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Math(e)       => write!(f, "Math error: {}", e),
            AppError::Validation(e) => write!(f, "Validation error: {}", e),
            AppError::Parse(e)      => write!(f, "Parse error: {}", e),
            AppError::Io(msg)       => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Implement From for automatic conversion with ?
impl From<MathError> for AppError {
    fn from(e: MathError) -> AppError { AppError::Math(e) }
}
impl From<ValidationError> for AppError {
    fn from(e: ValidationError) -> AppError { AppError::Validation(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> AppError { AppError::Parse(e) }
}

// ── Functions using custom errors ─────────────────────────────

fn safe_divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn safe_sqrt(n: f64) -> Result<f64, MathError> {
    if n < 0.0 {
        Err(MathError::NegativeSquareRoot(n))
    } else {
        Ok(n.sqrt())
    }
}

fn validate_username(name: &str) -> Result<&str, ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyField(String::from("username")));
    }
    if name.len() < 3 {
        return Err(ValidationError::TooShort { field: String::from("username"), min_len: 3 });
    }
    if name.len() > 20 {
        return Err(ValidationError::TooLong { field: String::from("username"), max_len: 20 });
    }
    if name.contains(' ') {
        return Err(ValidationError::InvalidFormat(
            String::from("username cannot contain spaces")));
    }
    Ok(name)
}

fn validate_age(age: i32) -> Result<i32, ValidationError> {
    if age < 1 || age > 120 {
        return Err(ValidationError::OutOfRange {
            field: String::from("age"),
            value: age,
            min: 1,
            max: 120,
        });
    }
    Ok(age)
}

// Function that uses ? with From conversion
fn register_user(name: &str, age_str: &str) -> Result<String, AppError> {
    let valid_name = validate_username(name)?;   // ValidationError → AppError via From
    let age_num = age_str.parse::<i32>()?;       // ParseIntError → AppError via From
    let valid_age = validate_age(age_num)?;       // ValidationError → AppError via From
    Ok(format!("Registered: {} (age {})", valid_name, valid_age))
}

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: MathError
    // --------------------------------------------------------
    println!("=== EXERCISE 1: MathError ===");

    let cases = [(10, 2), (5, 0), (100, 4), (0, 0)];
    for (a, b) in cases {
        match safe_divide(a, b) {
            Ok(r)  => println!("  {} / {} = {}", a, b, r),
            Err(e) => println!("  {} / {} = ERROR: {}", a, b, e),  // uses Display
        }
    }

    let sqrts = [25.0, 0.0, -4.0, 16.0];
    for n in sqrts {
        match safe_sqrt(n) {
            Ok(r)  => println!("  sqrt({}) = {:.4}", n, r),
            Err(e) => println!("  sqrt({}) = ERROR: {}", n, e),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 2: ValidationError
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: ValidationError ===");

    let usernames = ["", "Al", "Krishna", "this_name_is_way_too_long_for_a_username", "Jo hn"];
    for name in usernames {
        match validate_username(name) {
            Ok(valid) => println!("  '{}' ✓ valid", valid),
            Err(e)    => println!("  '{}' ✗ {}", name, e),
        }
    }

    let ages = [0, 25, 120, 121, -5];
    for age in ages {
        match validate_age(age) {
            Ok(a)  => println!("  Age {} ✓ valid", a),
            Err(e) => println!("  Age {} ✗ {}", age, e),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 3: Combined AppError with From conversions
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Combined Error Type ===");

    let registrations = [
        ("Krishna", "25"),
        ("",        "25"),
        ("Bob",     "abc"),
        ("Al",      "30"),
        ("Alice",   "200"),
        ("Carol",   "28"),
    ];

    for (name, age) in registrations {
        match register_user(name, age) {
            Ok(msg)  => println!("  ✓ {}", msg),
            Err(e)   => println!("  ✗ {}", e),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 4: Debug vs Display
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Debug vs Display ===");

    let err = MathError::UnderRange { value: -5, min: 0 };
    println!("Debug:   {:?}", err);   // uses Debug
    println!("Display: {}", err);     // uses Display

    let verr = ValidationError::TooShort { field: String::from("name"), min_len: 5 };
    println!("Debug:   {:?}", verr);
    println!("Display: {}", verr);

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Create a `FileError` enum with:
    //   NotFound(String)  -- filename
    //   PermissionDenied(String)
    //   InvalidContent(String)
    // Implement Display and error::Error

    // Challenge 2: Add a variant to MathError for `InvalidInput(String)`
    // and use it in a function that rejects non-numeric strings

    // Challenge 3: Create a `NetworkError` with:
    //   ConnectionFailed { host: String, port: u16 }
    //   Timeout(u64)  -- seconds
    //   InvalidUrl(String)
    // Implement Display

    // Challenge 4: Write a function `process(input: &str) -> Result<i32, AppError>`
    // that: parses input → validates it's positive → computes sqrt → returns as i32

    // --------------------------------------------------------
    // SUMMARY:
    //   enum MyError { V1, V2(T) }       -> define custom error
    //   impl Display for MyError { }     -> human-readable message
    //   impl std::error::Error for MyError {} -> makes it a proper error
    //   impl From<OtherError> for MyError -> auto-conversion with ?
    //   ? automatically converts using From impl
    //   {:?} uses Debug, {} uses Display
    //   Prefer enums over String errors — they're structured!
    // --------------------------------------------------------
}
