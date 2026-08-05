// ============================================================
// DAY 17: Pattern Matching
// Topic: The powerful `match` expression and pattern syntax
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. `match` — exhaustive pattern matching
//   2. Matching literals, ranges, multiple patterns
//   3. Binding with @ in patterns
//   4. Guards — extra conditions in match arms
//   5. `if let` — short-hand for single-pattern matching
//   6. `while let` — loop while pattern matches
//
// KEY RULE: `match` is EXHAUSTIVE — you must handle ALL cases!
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-17 folder)
//
// ============================================================

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),  // Quarter carries the state name
}

#[derive(Debug)]
enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Print(String),
    Repeat(u32),
}

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: match on an enum (basic)
    // Every arm: pattern => expression
    // match is exhaustive — all variants must be covered
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Basic match ===");

    let coin = Coin::Quarter(String::from("Alaska"));
    let value = match coin {
        Coin::Penny          => { println!("Lucky penny!"); 1 },
        Coin::Nickel         => 5,
        Coin::Dime           => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {}!", state);
            25
        }
    };
    println!("Coin value: {} cents", value);

    // --------------------------------------------------------
    // EXERCISE 2: match on integers + wildcard _
    // Use _ as a catch-all (like "else" for match)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: match on integers ===");

    for n in 0..=10 {
        let label = match n {
            0     => "zero",
            1     => "one",
            2     => "two",
            3..=5 => "three to five",  // range pattern
            6 | 7 => "six or seven",   // multiple values with |
            _     => "eight or more",  // wildcard — catches the rest
        };
        println!("  {} → {}", n, label);
    }

    // --------------------------------------------------------
    // EXERCISE 3: match with guard conditions
    // Add `if condition` to a match arm for extra filtering
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Match Guards ===");

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    for &n in &numbers {
        let category = match n {
            x if x % 2 == 0 && x < 6 => "small even",
            x if x % 2 == 0          => "large even",
            x if x % 2 != 0 && x < 6 => "small odd",
            _                          => "large odd",
        };
        println!("  {} → {}", n, category);
    }

    // --------------------------------------------------------
    // EXERCISE 4: Binding with @ in patterns
    // Capture the value that matched into a variable with @
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: @ Bindings ===");

    let num = 15;
    match num {
        n @ 1..=10  => println!("{} is between 1 and 10", n),
        n @ 11..=20 => println!("{} is between 11 and 20", n),
        n           => println!("{} is outside 1-20", n),
    }

    // --------------------------------------------------------
    // EXERCISE 5: match on structs/tuples
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: match on Tuples ===");

    let points = [(0, 0), (1, 0), (0, 1), (3, 4), (-1, 5)];
    for &(x, y) in &points {
        let location = match (x, y) {
            (0, 0)          => "origin",
            (x, 0) if x > 0 => "positive x-axis",
            (0, y) if y > 0 => "positive y-axis",
            (x, y) if x > 0 && y > 0 => "first quadrant",
            _               => "elsewhere",
        };
        println!("  ({}, {}) → {}", x, y, location);
    }

    // --------------------------------------------------------
    // EXERCISE 6: match on Command enum with named fields
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: match on Complex Enum ===");

    let commands = vec![
        Command::Quit,
        Command::Move { x: 10, y: 20 },
        Command::Print(String::from("Hello Rust!")),
        Command::Repeat(5),
    ];

    for cmd in &commands {
        match cmd {
            Command::Quit               => println!("Quitting..."),
            Command::Move { x, y }     => println!("Moving to ({}, {})", x, y),
            Command::Print(msg)         => println!("Printing: {}", msg),
            Command::Repeat(n)          => println!("Repeating {} times", n),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 7: `if let` — simplified match for one pattern
    // Use when you only care about ONE variant
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: if let ===");

    let some_value: Option<i32> = Some(42);

    // Long way with match:
    match some_value {
        Some(v) => println!("Got a value: {}", v),
        None    => {},  // do nothing — this feels wasteful
    }

    // Short way with if let:
    if let Some(v) = some_value {
        println!("Got a value (if let): {}", v);
    }

    // if let with else:
    let config_value: Option<&str> = None;
    if let Some(v) = config_value {
        println!("Config: {}", v);
    } else {
        println!("No config found, using default");
    }

    // --------------------------------------------------------
    // EXERCISE 8: `while let` — loop while pattern matches
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: while let ===");

    let mut stack = vec![1, 2, 3, 4, 5];
    print!("Popping: ");
    while let Some(top) = stack.pop() {  // loop while pop() returns Some(value)
        print!("{} ", top);
    }
    println!("\nStack is now empty: {:?}", stack);

    // --------------------------------------------------------
    // EXERCISE 9: Destructuring in match
    // --------------------------------------------------------
    println!("\n=== EXERCISE 9: Destructuring Patterns ===");

    // Destructure a struct in a match
    #[derive(Debug)]
    struct Point { x: i32, y: i32 }

    let p = Point { x: 5, y: -3 };
    match p {
        Point { x: 0, y }     => println!("On y-axis at y={}", y),
        Point { x, y: 0 }     => println!("On x-axis at x={}", x),
        Point { x, y }        => println!("Point at ({}, {})", x, y),
    }

    // --------------------------------------------------------
    // EXERCISE 10: Nested match
    // --------------------------------------------------------
    println!("\n=== EXERCISE 10: Nested match ===");

    let score: i32 = 78;
    let passed: bool = true;

    let result = match passed {
        true => match score {
            90..=100 => "Pass with Distinction",
            75..=89  => "Pass with Merit",
            _        => "Pass",
        },
        false => "Fail",
    };
    println!("Score {}, Passed {}: → {}", score, passed, result);

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write a match expression that categorizes numbers
    // as "Prime" for 2,3,5,7,11,13 and "Not prime" for others
    // (up to 15 using literal matching)

    // Challenge 2: Use if let to extract a value from a Result<i32, &str>
    // Print the value if Ok, print "Error: msg" if Err

    // Challenge 3: Create a match on a tuple (bool, bool) for:
    // (true, true) → "Both true"
    // (true, false) → "First only"
    // (false, true) → "Second only"
    // (false, false) → "Neither"

    // Challenge 4: Use while let with a Vec<Option<i32>>
    // Pop values and print only the Some values, skip None

    // --------------------------------------------------------
    // SUMMARY:
    //   match value { pat => expr, _ => default }   -> exhaustive match
    //   1..=5                 -> range pattern
    //   1 | 2 | 3            -> or pattern
    //   _ => ...             -> wildcard (catch all)
    //   x if x > 0 => ...   -> match guard
    //   n @ 1..=10 => ...   -> @ binding (capture matched value)
    //   if let Pat = val { } -> single-pattern shorthand
    //   while let Pat = expr { } -> loop while pattern matches
    //   All match arms must return same type
    //   match is exhaustive — all cases must be handled
    // --------------------------------------------------------
}
