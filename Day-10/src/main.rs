// ============================================================
// DAY 10: MINI PROJECT — Console Calculator
// Topic: Apply Days 1-9 concepts in a real program
// Time: 30-45 minutes
// ============================================================
//
// PROJECT: A simple calculator that can:
//   1. Add, Subtract, Multiply, Divide two numbers
//   2. Calculate power (x^n)
//   3. Find the square root
//   4. Calculate percentage
//   5. Show a summary of all operations
//
// CONCEPTS USED TODAY:
//   - Variables and mutability (Day 2)
//   - Data types: f64, bool (Day 3)
//   - Strings (Day 4)
//   - Operators (Day 5)
//   - if/else (Day 6)
//   - Loops (Day 7)
//   - Functions (Day 8)
//   - Arrays and Tuples (Day 9)
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-10 folder)
//
// ============================================================

// --------------------------------------------------------
// HELPER FUNCTIONS
// Each operation is its own function
// --------------------------------------------------------

/// Adds two numbers and returns the result
fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts b from a
fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two numbers
fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides a by b. Returns None if b is zero (to avoid crash)
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None  // Division by zero is not allowed!
    } else {
        Some(a / b)
    }
}

/// Calculates a raised to the power of n (integer exponent)
fn power(base: f64, exp: u32) -> f64 {
    let mut result = 1.0;
    for _ in 0..exp {
        result *= base;
    }
    result
}

/// Calculates the square root. Returns None if negative
fn square_root(n: f64) -> Option<f64> {
    if n < 0.0 {
        None  // Square root of negative number is not real
    } else {
        Some(n.sqrt())
    }
}

/// Calculates what percentage `part` is of `total`
fn percentage(part: f64, total: f64) -> f64 {
    if total == 0.0 {
        return 0.0;
    }
    (part / total) * 100.0
}

/// Checks if a number is prime
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}

/// Calculates GCD using Euclidean algorithm
fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Calculates LCM using GCD
fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

/// Prints a separator line
fn separator() {
    println!("{}", "─".repeat(45));
}

/// Prints the calculator header
fn print_header() {
    separator();
    println!("  🦀  RUST CONSOLE CALCULATOR  🦀");
    separator();
}

// --------------------------------------------------------
// MAIN PROGRAM
// --------------------------------------------------------
fn main() {
    print_header();

    // --------------------------------------------------------
    // DEMO 1: Basic Arithmetic
    // --------------------------------------------------------
    println!("\n📊 BASIC ARITHMETIC");
    separator();

    let a = 48.0_f64;
    let b = 6.0_f64;

    println!("a = {}, b = {}", a, b);
    println!("  a + b = {}", add(a, b));
    println!("  a - b = {}", subtract(a, b));
    println!("  a * b = {}", multiply(a, b));

    // Division with error handling
    match divide(a, b) {
        Some(result) => println!("  a / b = {:.4}", result),
        None         => println!("  a / b = ERROR: division by zero!"),
    }

    // Test division by zero
    match divide(10.0, 0.0) {
        Some(result) => println!("  10 / 0 = {}", result),
        None         => println!("  10 / 0 = ERROR: Cannot divide by zero!"),
    }

    // --------------------------------------------------------
    // DEMO 2: Power and Square Root
    // --------------------------------------------------------
    println!("\n📐 POWER & ROOTS");
    separator();

    let base = 3.0_f64;
    for exp in 0..=5_u32 {
        println!("  {}^{} = {}", base, exp, power(base, exp));
    }

    println!();
    let test_values = [0.0, 4.0, 9.0, 16.0, 25.0, -1.0];
    for val in test_values {
        match square_root(val) {
            Some(root) => println!("  √{:.0} = {:.4}", val, root),
            None       => println!("  √{:.0} = ERROR: cannot take root of negative number", val),
        }
    }

    // --------------------------------------------------------
    // DEMO 3: Percentage Calculator
    // --------------------------------------------------------
    println!("\n📈 PERCENTAGE CALCULATOR");
    separator();

    let marks_obtained: f64 = 432.0;
    let total_marks: f64 = 500.0;
    let pct = percentage(marks_obtained, total_marks);

    println!("  Marks: {}/{}", marks_obtained, total_marks);
    println!("  Percentage: {:.2}%", pct);

    let grade = if pct >= 90.0 {
        "A+ (Distinction)"
    } else if pct >= 80.0 {
        "A (First Class)"
    } else if pct >= 70.0 {
        "B (Second Class)"
    } else if pct >= 50.0 {
        "C (Pass)"
    } else {
        "F (Fail)"
    };
    println!("  Grade: {}", grade);

    // --------------------------------------------------------
    // DEMO 4: Prime Number Checker
    // --------------------------------------------------------
    println!("\n🔢 PRIME NUMBER CHECKER");
    separator();

    let numbers = [1, 2, 3, 4, 17, 18, 19, 23, 24, 97, 100];
    for n in numbers {
        let prime_status = if is_prime(n) { "PRIME ✓" } else { "not prime" };
        println!("  {} → {}", n, prime_status);
    }

    // --------------------------------------------------------
    // DEMO 5: GCD and LCM
    // --------------------------------------------------------
    println!("\n🔗 GCD & LCM");
    separator();

    let pairs: [(u64, u64); 4] = [(12, 18), (100, 75), (48, 64), (7, 13)];
    for (x, y) in pairs {
        println!("  GCD({}, {}) = {}  |  LCM({}, {}) = {}", x, y, gcd(x, y), x, y, lcm(x, y));
    }

    // --------------------------------------------------------
    // DEMO 6: Statistics Calculator
    // --------------------------------------------------------
    println!("\n📊 STATISTICS");
    separator();

    let dataset = [23.5, 45.0, 12.0, 67.8, 34.2, 89.1, 55.5, 28.0];
    println!("  Dataset: {:?}", dataset);

    // Sum
    let mut sum = 0.0_f64;
    for val in &dataset {
        sum += val;
    }
    println!("  Sum:     {:.2}", sum);

    // Count
    let count = dataset.len();
    println!("  Count:   {}", count);

    // Average / Mean
    let mean = sum / count as f64;
    println!("  Mean:    {:.2}", mean);

    // Min and Max
    let mut min = dataset[0];
    let mut max = dataset[0];
    for &val in &dataset {
        if val < min { min = val; }
        if val > max { max = val; }
    }
    println!("  Min:     {:.2}", min);
    println!("  Max:     {:.2}", max);
    println!("  Range:   {:.2}", max - min);

    // --------------------------------------------------------
    // DEMO 7: Temperature Conversion Table
    // --------------------------------------------------------
    println!("\n🌡️  TEMPERATURE CONVERSION TABLE");
    separator();
    println!("  {:>8} | {:>10} | {:>10}", "Celsius", "Fahrenheit", "Kelvin");
    println!("  {}", "-".repeat(35));

    for celsius in (-20..=120).step_by(20) {
        let c = celsius as f64;
        let f = c * 9.0 / 5.0 + 32.0;
        let k = c + 273.15;
        println!("  {:>8.1} | {:>10.1} | {:>10.2}", c, f, k);
    }

    // --------------------------------------------------------
    // DEMO 8: Multiplication table
    // --------------------------------------------------------
    println!("\n✖️  MULTIPLICATION TABLE (1–5 × 1–5)");
    separator();
    print!("  {:>4}", "");
    for i in 1..=5 {
        print!("{:>6}", i);
    }
    println!();
    println!("  {}", "-".repeat(34));

    for i in 1..=5 {
        print!("  {:>3} |", i);
        for j in 1..=5 {
            print!("{:>6}", i * j);
        }
        println!();
    }

    // --------------------------------------------------------
    // SUMMARY
    // --------------------------------------------------------
    separator();
    println!("  Day 10 Mini Project COMPLETE! 🎉");
    println!("  You used: functions, loops, if/else,");
    println!("  arrays, tuples, and operators.");
    separator();

    // --------------------------------------------------------
    // YOUR EXTENSION CHALLENGES:
    // --------------------------------------------------------
    // 1. Add a fibonacci sequence generator function
    //    fn fibonacci(n: u32) -> Vec<u64>
    //    Print the first 15 fibonacci numbers

    // 2. Add a compound interest calculator
    //    Formula: A = P * (1 + r/n)^(n*t)
    //    P=principal, r=rate, n=compounds/year, t=years

    // 3. Add a number to words converter
    //    e.g., 5 → "five", 11 → "eleven"
    //    Use an array of word names!

    // 4. Add a unit converter for length
    //    1 mile = 1.609 km
    //    1 inch = 2.54 cm
    //    1 foot = 30.48 cm
}
