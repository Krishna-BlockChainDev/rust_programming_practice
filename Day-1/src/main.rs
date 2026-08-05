// ============================================================
// DAY 1: Hello World & Cargo Basics
// Topic: Your very first Rust program!
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. How to print text to the screen using println!
//   2. What the main() function is
//   3. How comments work in Rust
//   4. Basic program structure
//   5. How to run with: cargo run
//
// HOW TO RUN THIS FILE:
//   Open terminal in the Day-1 folder and type:
//   $ cargo run
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Hello World
    // println! is a macro (note the !) that prints a line
    // --------------------------------------------------------
    println!("Hello, World!");

    // --------------------------------------------------------
    // EXERCISE 2: Print your name
    // Change "Krishna" to YOUR name below and run the program
    // --------------------------------------------------------
    println!("Hello, my name is Krishna!");

    // --------------------------------------------------------
    // EXERCISE 3: Print multiple lines
    // Each println! prints on a new line
    // --------------------------------------------------------
    println!("I am learning Rust programming.");
    println!("Today is Day 1!");
    println!("Let's go! 🦀");

    // --------------------------------------------------------
    // EXERCISE 4: Print with placeholders
    // {} is a placeholder - it gets replaced with the value
    // --------------------------------------------------------
    println!("I am {} years old.", 25);
    println!("My favorite number is {}.", 7);

    // --------------------------------------------------------
    // EXERCISE 5: Print multiple placeholders
    // You can use multiple {} in one println!
    // --------------------------------------------------------
    println!("My name is {} and I am {} years old.", "Krishna", 25);

    // --------------------------------------------------------
    // EXERCISE 6: print! vs println!
    // print! does NOT add a new line at the end
    // println! DOES add a new line at the end
    // --------------------------------------------------------
    print!("This is on ");
    print!("the same line. ");
    println!("And this ends the line.");

    // --------------------------------------------------------
    // EXERCISE 7: Escape characters
    // \n = new line inside a string
    // \t = tab inside a string
    // --------------------------------------------------------
    println!("Line 1\nLine 2\nLine 3");
    println!("Name:\tKrishna");

    // --------------------------------------------------------
    // EXERCISE 8: Debug printing with {:?}
    // {:?} is used to print values in debug format
    // Useful for complex types like arrays later
    // --------------------------------------------------------
    println!("{:?}", 42);
    println!("{:?}", true);

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // Try these on your own! Uncomment and fill in:
    // --------------------------------------------------------

    // Challenge 1: Print your full name
    // println!("My full name is ___.");

    // Challenge 2: Print your city and country
    // println!("I live in ___, ___.");

    // Challenge 3: Print a math result using placeholder
    // println!("5 + 3 = {}", 5 + 3);

    // Challenge 4: Print three things on one line using print!
    // print!("Rust ");
    // print!("is ");
    // println!("awesome!");

    // --------------------------------------------------------
    // SUMMARY:
    //   println!("text")         -> prints with newline
    //   print!("text")           -> prints without newline
    //   println!("{}", value)    -> prints with placeholder
    //   println!("{:?}", value)  -> debug print
    //   // this is a single line comment
    //   /* this is a block comment */
    // --------------------------------------------------------
}
