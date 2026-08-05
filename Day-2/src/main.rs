// ============================================================
// DAY 2: Variables & Mutability
// Topic: How Rust handles variables differently from other languages
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Declaring variables with `let`
//   2. Immutability by default (Rust is safe by design!)
//   3. Making variables mutable with `mut`
//   4. Constants with `const`
//   5. Shadowing — re-declaring a variable with same name
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-2 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Basic variable declaration with `let`
    // In Rust, variables are IMMUTABLE by default
    // This means once set, you cannot change the value
    // --------------------------------------------------------
    let name = "Krishna";
    let age = 25;
    println!("Name: {}", name);
    println!("Age: {}", age);

    // --------------------------------------------------------
    // EXERCISE 2: Trying to change an immutable variable
    // The line below would cause a COMPILE ERROR — try it!
    // Uncomment it to see the error message from Rust compiler:
    // --------------------------------------------------------
    // age = 26; // ERROR: cannot assign twice to immutable variable

    // --------------------------------------------------------
    // EXERCISE 3: Mutable variables with `mut`
    // Add `mut` keyword to allow changing the variable's value
    // --------------------------------------------------------
    let mut score = 0;
    println!("Initial score: {}", score);

    score = 10;   // This is allowed because we used `mut`
    println!("Updated score: {}", score);

    score = score + 5;  // score becomes 15
    println!("Score after adding 5: {}", score);

    // --------------------------------------------------------
    // EXERCISE 4: Constants with `const`
    // - Always immutable (no `mut` allowed)
    // - Must have explicit type annotation
    // - Named in SCREAMING_SNAKE_CASE by convention
    // - Can be declared in any scope including global
    // --------------------------------------------------------
    const MAX_POINTS: u32 = 100_000;  // _ is a visual separator
    const PI: f64 = 3.14159;
    println!("Max points: {}", MAX_POINTS);
    println!("PI value: {}", PI);

    // --------------------------------------------------------
    // EXERCISE 5: Shadowing
    // You can re-declare a variable with the same name using `let`
    // This is called "shadowing" — the new variable shadows the old one
    // It's different from `mut` — you can even change the TYPE!
    // --------------------------------------------------------
    let x = 5;
    println!("x is: {}", x);   // x = 5

    let x = x + 1;             // shadows the previous x
    println!("x is: {}", x);   // x = 6

    let x = x * 2;             // shadows again
    println!("x is: {}", x);   // x = 12

    // Shadowing can also change the type:
    let spaces = "   ";         // spaces is a &str (text)
    println!("spaces (text): '{}'", spaces);

    let spaces = spaces.len();  // now spaces is a number (usize)
    println!("spaces (number): {}", spaces);

    // --------------------------------------------------------
    // EXERCISE 6: Type inference vs explicit type annotation
    // Rust can usually figure out the type automatically (inference)
    // But you can also specify it explicitly with :type
    // --------------------------------------------------------
    let inferred = 42;           // Rust infers this is i32
    let explicit: i32 = 42;      // You write the type yourself
    let float_val: f64 = 3.14;   // Explicit f64 (64-bit float)
    let is_active: bool = true;  // Explicit bool

    println!("inferred: {}", inferred);
    println!("explicit: {}", explicit);
    println!("float: {}", float_val);
    println!("active: {}", is_active);

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Create a mutable variable `counter` starting at 0,
    // then increase it by 1 three times and print each step
    // let mut counter = 0;
    // ...

    // Challenge 2: Declare a constant for your birth year
    // const BIRTH_YEAR: u32 = ____;
    // println!("Born in: {}", BIRTH_YEAR);

    // Challenge 3: Use shadowing to convert a temperature
    // let temp = 100;  // Celsius
    // let temp = temp * 9 / 5 + 32;  // Convert to Fahrenheit
    // println!("Temperature in F: {}", temp);

    // Challenge 4: Declare variables for your name, age, and city
    // then print them all in one sentence

    // --------------------------------------------------------
    // SUMMARY:
    //   let x = 5;           -> immutable variable
    //   let mut x = 5;       -> mutable variable (can change)
    //   const NAME: type = value;  -> constant (always immutable)
    //   let x = x + 1;       -> shadowing (creates new variable)
    //   let x: i32 = 5;      -> explicit type annotation
    // --------------------------------------------------------
}
