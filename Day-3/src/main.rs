// ============================================================
// DAY 3: Data Types
// Topic: Rust's built-in primitive data types
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Integer types (i8, i16, i32, i64, u8, u16, u32, u64, usize)
//   2. Floating point types (f32, f64)
//   3. Boolean type (bool)
//   4. Character type (char)
//   5. Type casting / conversion
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-3 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Integer Types
    //
    // Signed integers can be negative:   i8, i16, i32, i64, i128
    // Unsigned integers are positive:    u8, u16, u32, u64, u128
    // The number = how many bits used
    //
    //   i8  = -128 to 127
    //   u8  =    0 to 255
    //   i32 = about -2 billion to 2 billion  (most common)
    //   u32 = about 0 to 4 billion
    //   i64 = very large range
    //   usize = size of pointer (depends on OS, 64-bit usually)
    // --------------------------------------------------------
    let a: i8 = 100;
    let b: u8 = 255;          // max value for u8
    let c: i32 = -1_000_000; // underscore = visual separator (like comma)
    let d: u32 = 4_000_000;
    let e: i64 = 9_000_000_000;
    let f: usize = 10;        // often used for indexing

    println!("i8:    {}", a);
    println!("u8:    {}", b);
    println!("i32:   {}", c);
    println!("u32:   {}", d);
    println!("i64:   {}", e);
    println!("usize: {}", f);

    // --------------------------------------------------------
    // EXERCISE 2: Floating Point Types
    // f32 = 32-bit float (less precise)
    // f64 = 64-bit float (more precise, DEFAULT in Rust)
    // --------------------------------------------------------
    let pi: f64 = 3.14159265358979;
    let gravity: f32 = 9.81;
    let temperature = 36.6; // Rust infers f64 by default

    println!("\nPI (f64): {}", pi);
    println!("Gravity (f32): {}", gravity);
    println!("Temperature: {}", temperature);

    // Formatting floats — control decimal places with {:.2}
    println!("PI to 2 decimals: {:.2}", pi);
    println!("PI to 4 decimals: {:.4}", pi);

    // --------------------------------------------------------
    // EXERCISE 3: Boolean Type
    // Can only be `true` or `false`
    // Often used in conditions (we'll use in Day 6)
    // --------------------------------------------------------
    let is_rust_fun: bool = true;
    let is_easy: bool = false;

    println!("\nIs Rust fun? {}", is_rust_fun);
    println!("Is it easy at first? {}", is_easy);

    // Booleans from comparisons
    let x = 10;
    let y = 20;
    let is_greater = x > y;     // false, because 10 is not > 20
    let is_equal = x == y;       // false
    let is_not_equal = x != y;   // true

    println!("Is x > y? {}", is_greater);
    println!("Is x == y? {}", is_equal);
    println!("Is x != y? {}", is_not_equal);

    // --------------------------------------------------------
    // EXERCISE 4: Character Type (char)
    // Use SINGLE quotes for char (not double quotes!)
    // char in Rust is 4 bytes (supports Unicode, emoji, etc.)
    // --------------------------------------------------------
    let letter: char = 'A';
    let digit_char: char = '9';  // This is the CHARACTER '9', not number 9
    let emoji: char = '🦀';      // Rust supports emojis as char!
    let hindi: char = 'क';       // Unicode characters work too

    println!("\nLetter: {}", letter);
    println!("Digit char: {}", digit_char);
    println!("Emoji: {}", emoji);
    println!("Hindi: {}", hindi);

    // --------------------------------------------------------
    // EXERCISE 5: Numeric Operations
    // --------------------------------------------------------
    let sum = 5 + 10;
    let difference = 95 - 4;
    let product = 4 * 30;
    let quotient = 56 / 6;      // Integer division — truncates decimal!
    let remainder = 43 % 5;     // Modulo (remainder after division)

    println!("\n5 + 10 = {}", sum);
    println!("95 - 4 = {}", difference);
    println!("4 * 30 = {}", product);
    println!("56 / 6 = {} (integer division, truncated)", quotient);
    println!("43 % 5 = {}", remainder);

    // Float division keeps decimals:
    let float_div = 56.0 / 6.0;
    println!("56.0 / 6.0 = {:.4}", float_div);

    // --------------------------------------------------------
    // EXERCISE 6: Type Casting with `as`
    // Rust does NOT automatically convert between types
    // You must use `as` keyword to convert explicitly
    // --------------------------------------------------------
    let integer: i32 = 42;
    let float_from_int = integer as f64;   // i32 → f64
    println!("\n42 as f64: {}", float_from_int);

    let large: f64 = 3.99;
    let truncated = large as i32;          // f64 → i32 (truncates decimal!)
    println!("3.99 as i32: {} (decimal lost!)", truncated);

    let small_num: u8 = 200;
    let as_i32 = small_num as i32;
    println!("200u8 as i32: {}", as_i32);

    // --------------------------------------------------------
    // EXERCISE 7: Min/Max values (useful to know)
    // --------------------------------------------------------
    println!("\nMax i32: {}", i32::MAX);
    println!("Min i32: {}", i32::MIN);
    println!("Max u8:  {}", u8::MAX);
    println!("Max f64: {}", f64::MAX);

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Declare an age (u8), height in cm (u16), 
    // weight in kg (f32), and print them all
    // let age: u8 = ___;
    // let height: u16 = ___;
    // let weight: f32 = ___;

    // Challenge 2: Calculate area of a rectangle
    // let width: f64 = 5.5;
    // let height: f64 = 3.2;
    // let area = width * height;
    // println!("Area: {:.2}", area);

    // Challenge 3: Use % to check if a number is even or odd
    // let number = 17;
    // let remainder = number % 2;
    // if remainder == 0 { println!("Even") } else { println!("Odd") }

    // Challenge 4: Convert 100 degrees Celsius to Fahrenheit
    // Formula: F = C * 9 / 5 + 32
    // let celsius: f64 = 100.0;
    // let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
    // println!("{} C = {} F", celsius, fahrenheit);

    // --------------------------------------------------------
    // SUMMARY:
    //   i8/i16/i32/i64   -> signed integers (negative & positive)
    //   u8/u16/u32/u64   -> unsigned integers (only positive)
    //   f32/f64          -> floating point (decimals), f64 is default
    //   bool             -> true or false
    //   char             -> single character in single quotes 'A'
    //   value as Type    -> type casting/conversion
    //   {:.2}            -> format float to 2 decimal places
    // --------------------------------------------------------
}
