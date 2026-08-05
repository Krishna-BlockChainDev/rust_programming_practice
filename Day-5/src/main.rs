// ============================================================
// DAY 5: Operators & Expressions
// Topic: All kinds of operators in Rust
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Arithmetic operators: + - * / %
//   2. Comparison operators: == != < > <= >=
//   3. Logical operators: && || !
//   4. Compound assignment: += -= *= /= %=
//   5. Expressions vs Statements in Rust
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-5 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Arithmetic Operators
    // --------------------------------------------------------
    let a = 20;
    let b = 6;

    let add  = a + b;   // Addition
    let sub  = a - b;   // Subtraction
    let mul  = a * b;   // Multiplication
    let div  = a / b;   // Division (integer — truncates!)
    let rem  = a % b;   // Remainder / Modulo

    println!("=== Arithmetic Operators ===");
    println!("{} + {} = {}", a, b, add);
    println!("{} - {} = {}", a, b, sub);
    println!("{} * {} = {}", a, b, mul);
    println!("{} / {} = {} (integer division)", a, b, div);
    println!("{} % {} = {} (remainder)", a, b, rem);

    // Float division preserves decimals
    let fa: f64 = 20.0;
    let fb: f64 = 6.0;
    println!("{} / {} = {:.4} (float division)", fa, fb, fa / fb);

    // --------------------------------------------------------
    // EXERCISE 2: Comparison Operators
    // These return true or false (bool)
    // --------------------------------------------------------
    let x = 10;
    let y = 20;

    println!("\n=== Comparison Operators ===");
    println!("{} == {} : {}", x, y, x == y);   // Equal
    println!("{} != {} : {}", x, y, x != y);   // Not equal
    println!("{} <  {} : {}", x, y, x < y);    // Less than
    println!("{} >  {} : {}", x, y, x > y);    // Greater than
    println!("{} <= {} : {}", x, y, x <= y);   // Less than or equal
    println!("{} >= {} : {}", x, y, x >= y);   // Greater than or equal

    // --------------------------------------------------------
    // EXERCISE 3: Logical Operators
    // && = AND (both must be true)
    // || = OR  (at least one must be true)
    // !  = NOT (flips true to false and vice versa)
    // --------------------------------------------------------
    let is_sunny = true;
    let is_warm = false;
    let is_raining = false;

    println!("\n=== Logical Operators ===");
    println!("is_sunny AND is_warm: {}", is_sunny && is_warm);
    println!("is_sunny OR  is_warm: {}", is_sunny || is_warm);
    println!("NOT is_sunny: {}", !is_sunny);
    println!("NOT is_raining: {}", !is_raining);

    // Combining logical operators
    let can_go_out = is_sunny && !is_raining;
    println!("Can go out (sunny and not raining): {}", can_go_out);

    // --------------------------------------------------------
    // EXERCISE 4: Compound Assignment Operators
    // These are shortcuts: x += 5 is the same as x = x + 5
    // --------------------------------------------------------
    let mut score = 100;

    println!("\n=== Compound Assignment ===");
    println!("Initial score: {}", score);

    score += 10;   // same as: score = score + 10
    println!("After += 10: {}", score);

    score -= 5;    // same as: score = score - 5
    println!("After -= 5: {}", score);

    score *= 2;    // same as: score = score * 2
    println!("After *= 2: {}", score);

    score /= 3;    // same as: score = score / 3
    println!("After /= 3: {}", score);

    score %= 7;    // same as: score = score % 7
    println!("After %= 7: {}", score);

    // --------------------------------------------------------
    // EXERCISE 5: Expressions vs Statements
    // In Rust, EVERYTHING that returns a value is an EXPRESSION
    // Statements do NOT return values
    //
    // Key rule: expressions do NOT end with semicolon
    // Statements DO end with semicolon
    // --------------------------------------------------------

    // A block {} can be an expression!
    // The last line without ; is the return value
    let result = {
        let p = 5;
        let q = 10;
        p + q    // No semicolon! This is the value returned from the block
    };
    println!("\nBlock expression result: {}", result);

    // if is also an expression in Rust:
    let number = 7;
    let is_odd = if number % 2 != 0 { true } else { false };
    println!("Is {} odd? {}", number, is_odd);

    // Even simpler way using an expression:
    let description = if number > 5 { "big" } else { "small" };
    println!("{} is a {} number", number, description);

    // --------------------------------------------------------
    // EXERCISE 6: Operator Precedence
    // Just like in math: * and / happen before + and -
    // Use parentheses () to control order
    // --------------------------------------------------------
    println!("\n=== Operator Precedence ===");
    let result1 = 2 + 3 * 4;       // 3*4=12, then 2+12=14
    let result2 = (2 + 3) * 4;     // 2+3=5, then 5*4=20
    println!("2 + 3 * 4 = {}", result1);
    println!("(2 + 3) * 4 = {}", result2);

    // --------------------------------------------------------
    // EXERCISE 7: Practical example — BMI Calculator
    // BMI = weight(kg) / height(m)^2
    // --------------------------------------------------------
    println!("\n=== BMI Calculator ===");
    let weight: f64 = 70.0;    // kg
    let height: f64 = 1.75;   // meters
    let bmi = weight / (height * height);

    println!("Weight: {} kg", weight);
    println!("Height: {} m", height);
    println!("BMI: {:.2}", bmi);

    let category = if bmi < 18.5 {
        "Underweight"
    } else if bmi < 25.0 {
        "Normal weight"
    } else if bmi < 30.0 {
        "Overweight"
    } else {
        "Obese"
    };
    println!("Category: {}", category);

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Calculate simple interest
    // Formula: SI = (principal * rate * time) / 100
    // let principal: f64 = 10000.0;
    // let rate: f64 = 5.0;  // 5% per year
    // let time: f64 = 3.0;  // 3 years
    // let si = (principal * rate * time) / 100.0;
    // println!("Simple Interest: {:.2}", si);

    // Challenge 2: Check if a number is divisible by both 3 and 5
    // let num = 15;
    // let divisible = (num % 3 == 0) && (num % 5 == 0);
    // println!("{} divisible by 3 and 5: {}", num, divisible);

    // Challenge 3: Use compound assignment to simulate a bank account
    // let mut balance = 1000;
    // balance += 500;   // deposit
    // balance -= 200;   // withdrawal
    // println!("Final balance: {}", balance);

    // Challenge 4: Calculate the area and perimeter of a circle
    // Formula: area = PI * r * r,  perimeter = 2 * PI * r
    // const PI: f64 = 3.14159;
    // let radius: f64 = 7.0;
    // let area = PI * radius * radius;
    // let perimeter = 2.0 * PI * radius;
    // println!("Area: {:.2}, Perimeter: {:.2}", area, perimeter);

    // --------------------------------------------------------
    // SUMMARY:
    //   +  -  *  /  %    -> arithmetic
    //   == != < > <= >=  -> comparison (returns bool)
    //   &&  ||  !        -> logical AND, OR, NOT
    //   += -= *= /= %=   -> compound assignment
    //   { expr }         -> blocks are expressions
    //   if cond { } else { }  -> if is an expression in Rust
    // --------------------------------------------------------
}
