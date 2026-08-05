// ============================================================
// DAY 6: Control Flow — if / else if / else
// Topic: Making decisions in your programs
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Basic if/else statements
//   2. else if chains
//   3. if as an expression (returns a value)
//   4. Nested if statements
//   5. Practical examples
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-6 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Basic if / else
    // if checks a condition (must be bool — NOT a number like C!)
    // --------------------------------------------------------
    let temperature = 35;

    println!("=== Basic if/else ===");
    if temperature > 30 {
        println!("It's hot today! Stay hydrated.");
    } else {
        println!("The weather is pleasant.");
    }

    // --------------------------------------------------------
    // EXERCISE 2: else if chain
    // Check multiple conditions in order
    // Only the FIRST matching block runs
    // --------------------------------------------------------
    let score = 75;

    println!("\n=== Grade Calculator ===");
    println!("Score: {}", score);

    if score >= 90 {
        println!("Grade: A — Excellent!");
    } else if score >= 80 {
        println!("Grade: B — Good!");
    } else if score >= 70 {
        println!("Grade: C — Average");
    } else if score >= 60 {
        println!("Grade: D — Below Average");
    } else {
        println!("Grade: F — Fail");
    }

    // --------------------------------------------------------
    // EXERCISE 3: if with boolean variables
    // --------------------------------------------------------
    let is_logged_in = true;
    let has_permission = false;

    println!("\n=== Access Control ===");
    if is_logged_in && has_permission {
        println!("Access granted! Welcome.");
    } else if is_logged_in && !has_permission {
        println!("Logged in but no permission to access this resource.");
    } else {
        println!("Please login first.");
    }

    // --------------------------------------------------------
    // EXERCISE 4: if as an expression (returns a value!)
    // Rust allows using if to assign a value to a variable
    // IMPORTANT: both branches must return the SAME type
    // --------------------------------------------------------
    let age = 20;

    let can_vote = if age >= 18 { "Yes" } else { "No" };
    println!("\nCan vote (age {}): {}", age, can_vote);

    let ticket_price = if age < 12 {
        0       // free for kids
    } else if age < 18 {
        150     // student price
    } else if age >= 60 {
        100     // senior discount
    } else {
        300     // regular price
    };
    println!("Ticket price: ₹{}", ticket_price);

    // --------------------------------------------------------
    // EXERCISE 5: Nested if
    // You can put if inside if
    // --------------------------------------------------------
    let username = "admin";
    let password = "rust123";

    println!("\n=== Login System ===");
    if username == "admin" {
        if password == "rust123" {
            println!("Login successful! Welcome, admin.");
        } else {
            println!("Wrong password!");
        }
    } else {
        println!("Unknown user: {}", username);
    }

    // --------------------------------------------------------
    // EXERCISE 6: Checking ranges
    // --------------------------------------------------------
    let number = 42;

    println!("\n=== Number Checker ===");
    if number < 0 {
        println!("{} is negative", number);
    } else if number == 0 {
        println!("{} is zero", number);
    } else if number % 2 == 0 {
        println!("{} is positive and EVEN", number);
    } else {
        println!("{} is positive and ODD", number);
    }

    // --------------------------------------------------------
    // EXERCISE 7: FizzBuzz (classic programming challenge!)
    // Rule:
    //   Divisible by 3 AND 5 → "FizzBuzz"
    //   Divisible by 3 only  → "Fizz"
    //   Divisible by 5 only  → "Buzz"
    //   Otherwise            → print the number
    // --------------------------------------------------------
    println!("\n=== FizzBuzz (1 to 20) ===");
    let mut n = 1;
    while n <= 20 {
        if n % 3 == 0 && n % 5 == 0 {
            println!("{}: FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{}: Fizz", n);
        } else if n % 5 == 0 {
            println!("{}: Buzz", n);
        } else {
            println!("{}", n);
        }
        n += 1;
    }

    // --------------------------------------------------------
    // EXERCISE 8: Season Checker
    // --------------------------------------------------------
    let month = 7; // July

    println!("\n=== Season Checker ===");
    let season = if month >= 3 && month <= 5 {
        "Spring"
    } else if month >= 6 && month <= 8 {
        "Summer"
    } else if month >= 9 && month <= 11 {
        "Autumn"
    } else {
        "Winter"
    };
    println!("Month {} is in {}", month, season);

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write an age group classifier
    // 0-12: Child, 13-17: Teenager, 18-59: Adult, 60+: Senior
    // let age = 45;
    // if ...

    // Challenge 2: Write a traffic light program
    // "red" → "Stop", "yellow" → "Slow down", "green" → "Go"
    // let light = "green";
    // let action = if light == "red" { "Stop" } else if ...

    // Challenge 3: Check if a year is a leap year
    // Leap year: divisible by 4 AND (not divisible by 100 OR divisible by 400)
    // let year = 2024;
    // let is_leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    // println!("{} is leap year: {}", year, is_leap);

    // Challenge 4: Electricity bill calculator
    // 0-100 units: ₹3/unit
    // 101-300 units: ₹5/unit
    // 300+ units: ₹7/unit
    // let units = 250;
    // let bill = if units <= 100 { units * 3 } else if units <= 300 { ... }

    // --------------------------------------------------------
    // SUMMARY:
    //   if condition { }                    -> basic if
    //   if cond { } else { }               -> if-else
    //   if cond { } else if cond { } ...   -> else if chain
    //   let x = if cond { val1 } else { val2 };  -> if as expression
    //   Conditions MUST be bool (not 0/1 like C)
    //   Both branches of if expression must return same type
    // --------------------------------------------------------
}
