// ============================================================
// DAY 8: Functions
// Topic: Defining and calling functions in Rust
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Defining functions with `fn`
//   2. Function parameters and arguments
//   3. Return values (with -> type)
//   4. Implicit return (last expression, no semicolon)
//   5. Multiple parameters
//   6. Functions calling other functions
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-8 folder)
//
// ============================================================

// --------------------------------------------------------
// Functions can be defined OUTSIDE of main()
// Rust doesn't care about the order — you can call a function
// defined below the call site!
// --------------------------------------------------------

// EXAMPLE 1: A simple function with no parameters and no return value
fn say_hello() {
    println!("Hello from a function!");
}

// EXAMPLE 2: Function with a parameter
// Parameters MUST have a type annotation!
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// EXAMPLE 3: Function with multiple parameters
fn introduce(name: &str, age: u32, city: &str) {
    println!("My name is {}, I am {} years old, from {}.", name, age, city);
}

// EXAMPLE 4: Function with a return value
// Use -> to specify the return type
// The last expression WITHOUT semicolon is returned automatically
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon! This is the return value (implicit return)
}

// You can also use `return` explicitly (needed for early returns)
fn subtract(a: i32, b: i32) -> i32 {
    return a - b;  // Explicit return — semicolon is needed here
}

// EXAMPLE 5: Function that returns a bool
fn is_even(n: i32) -> bool {
    n % 2 == 0  // returns true if even, false if odd
}

// EXAMPLE 6: Function with early return
fn check_age(age: u32) -> &'static str {
    if age < 18 {
        return "Minor";  // Early return with explicit `return`
    }
    "Adult"  // Implicit return for the happy path
}

// EXAMPLE 7: Function calling another function
fn square(n: i32) -> i32 {
    n * n
}

fn cube(n: i32) -> i32 {
    n * square(n)  // calls square() function
}

// EXAMPLE 8: Temperature converter functions
fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// EXAMPLE 9: Function returning a tuple (multiple values)
// Rust functions can only return ONE value, but tuples let you pack multiple!
fn min_max(a: i32, b: i32, c: i32) -> (i32, i32) {
    let min = if a < b { if a < c { a } else { c } } else { if b < c { b } else { c } };
    let max = if a > b { if a > c { a } else { c } } else { if b > c { b } else { c } };
    (min, max)  // return a tuple
}

// EXAMPLE 10: Recursive function (a function that calls itself)
fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;          // Base case: stop recursion
    }
    n * factorial(n - 1)   // Recursive call
}

// --------------------------------------------------------
// MAIN FUNCTION — entry point of the program
// --------------------------------------------------------
fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Call simple functions
    // --------------------------------------------------------
    println!("=== Simple Functions ===");
    say_hello();
    greet("Krishna");
    greet("Rust Learner");

    // --------------------------------------------------------
    // EXERCISE 2: Functions with multiple parameters
    // --------------------------------------------------------
    println!("\n=== Multiple Parameters ===");
    introduce("Krishna", 25, "Bangalore");
    introduce("Alice", 30, "Mumbai");

    // --------------------------------------------------------
    // EXERCISE 3: Functions with return values
    // --------------------------------------------------------
    println!("\n=== Return Values ===");
    let sum = add(10, 20);
    let diff = subtract(50, 15);
    println!("10 + 20 = {}", sum);
    println!("50 - 15 = {}", diff);

    // Use return value directly in println
    println!("100 + 200 = {}", add(100, 200));

    // --------------------------------------------------------
    // EXERCISE 4: Boolean return functions
    // --------------------------------------------------------
    println!("\n=== Boolean Returns ===");
    for n in [4, 7, 10, 13, 20] {
        println!("{} is even: {}", n, is_even(n));
    }

    // --------------------------------------------------------
    // EXERCISE 5: Early return
    // --------------------------------------------------------
    println!("\n=== Early Return ===");
    println!("Age 15: {}", check_age(15));
    println!("Age 25: {}", check_age(25));

    // --------------------------------------------------------
    // EXERCISE 6: Chained function calls
    // --------------------------------------------------------
    println!("\n=== Chained Functions ===");
    println!("square(5) = {}", square(5));
    println!("cube(3) = {}", cube(3));
    println!("cube(4) = {}", cube(4));

    // --------------------------------------------------------
    // EXERCISE 7: Temperature converter
    // --------------------------------------------------------
    println!("\n=== Temperature Converter ===");
    let celsius_vals = [0.0, 25.0, 37.0, 100.0];
    for c in celsius_vals {
        println!("{:.1}°C = {:.1}°F", c, celsius_to_fahrenheit(c));
    }
    println!("98.6°F = {:.1}°C", fahrenheit_to_celsius(98.6));

    // --------------------------------------------------------
    // EXERCISE 8: Tuple return
    // --------------------------------------------------------
    println!("\n=== Min/Max from Tuple Return ===");
    let (minimum, maximum) = min_max(42, 7, 19);  // destructure the tuple
    println!("Among 42, 7, 19 — Min: {}, Max: {}", minimum, maximum);

    // --------------------------------------------------------
    // EXERCISE 9: Recursive factorial
    // --------------------------------------------------------
    println!("\n=== Factorial (Recursive) ===");
    for n in 0..=8 {
        println!("{}! = {}", n, factorial(n));
    }

    // --------------------------------------------------------
    // EXERCISE 10: Functions defined inline (closures preview)
    // We'll cover closures fully on Day 37, but here's a taste
    // --------------------------------------------------------
    println!("\n=== Inline Function (closure preview) ===");
    let double = |x: i32| x * 2;    // A closure: like a function but inline
    println!("double(5) = {}", double(5));
    println!("double(12) = {}", double(12));

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write a function `power(base, exp)` that
    // calculates base^exp using a loop (not recursion)
    // fn power(base: i32, exp: u32) -> i32 { ... }

    // Challenge 2: Write a function `is_palindrome(s: &str) -> bool`
    // that returns true if a string is the same forwards and backwards
    // Hint: s == s.chars().rev().collect::<String>()

    // Challenge 3: Write a function `celsius_to_kelvin(c: f64) -> f64`
    // Formula: K = C + 273.15

    // Challenge 4: Write a function `gcd(a: u32, b: u32) -> u32`
    // that finds the Greatest Common Divisor using Euclidean algorithm
    // while b != 0 { let temp = b; b = a % b; a = temp; } return a;

    // --------------------------------------------------------
    // SUMMARY:
    //   fn name() { }                     -> no params, no return
    //   fn name(x: i32) { }               -> with parameter
    //   fn name(x: i32) -> i32 { x + 1 } -> with return value
    //   last expression without ; = return value (implicit return)
    //   return value;                      -> explicit return
    //   fn name() -> (i32, i32) { (a, b) } -> return multiple via tuple
    //   let (a, b) = name();               -> destructure tuple return
    //   Recursive: function calling itself with a base case
    // --------------------------------------------------------
}
