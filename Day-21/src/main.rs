// ============================================================
// DAY 21: Error Handling — Result<T, E>
// Topic: Handling operations that can fail
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Result<T, E> — Ok(value) or Err(error)
//   2. Handling Result with match
//   3. unwrap(), expect(), unwrap_or()
//   4. is_ok(), is_err()
//   5. Converting between Result and Option
//   6. Practical error handling examples
//
// KEY CONCEPT:
//   Result<T, E> is the Rust way to handle operations that CAN fail.
//   Ok(value)  = success, contains the result value
//   Err(error) = failure, contains error information
//   The compiler FORCES you to handle both cases!
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-21 folder)
//
// ============================================================

use std::num::ParseIntError;

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Basic Result
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Basic Result ===");

    // Result<T, E> has two variants:
    let ok_result: Result<i32, String> = Ok(42);
    let err_result: Result<i32, String> = Err(String::from("Something went wrong!"));

    println!("ok_result:  {:?}", ok_result);
    println!("err_result: {:?}", err_result);

    // --------------------------------------------------------
    // EXERCISE 2: Handling Result with match
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: match on Result ===");

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero!"))
        } else {
            Ok(a / b)
        }
    }

    let operations = [(10.0, 2.0), (15.0, 3.0), (8.0, 0.0), (100.0, 4.0)];
    for (a, b) in operations {
        match divide(a, b) {
            Ok(result) => println!("  {} / {} = {:.2}", a, b, result),
            Err(e)     => println!("  {} / {} = ERROR: {}", a, b, e),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 3: Result methods — is_ok, is_err
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Result Methods ===");

    let r1 = divide(10.0, 2.0);
    let r2 = divide(10.0, 0.0);

    println!("r1.is_ok():  {}", r1.is_ok());
    println!("r1.is_err(): {}", r1.is_err());
    println!("r2.is_ok():  {}", r2.is_ok());
    println!("r2.is_err(): {}", r2.is_err());

    // --------------------------------------------------------
    // EXERCISE 4: unwrap, expect, unwrap_or
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: unwrap / expect / unwrap_or ===");

    // unwrap() — gets Ok value, PANICS on Err
    let val = divide(20.0, 4.0).unwrap();
    println!("unwrap: {}", val);

    // expect("msg") — like unwrap but with custom panic message
    let val2 = divide(30.0, 5.0).expect("Division should work!");
    println!("expect: {}", val2);

    // unwrap_or(default) — returns default value on Err
    let val3 = divide(10.0, 0.0).unwrap_or(0.0);  // 0.0 on error
    println!("unwrap_or: {}", val3);

    // unwrap_or_else(|err| ...) — run closure on error
    let val4 = divide(10.0, 0.0).unwrap_or_else(|e| {
        println!("  Error occurred: {}", e);
        -1.0  // fallback value
    });
    println!("unwrap_or_else: {}", val4);

    // --------------------------------------------------------
    // EXERCISE 5: Parsing strings — a common real-world case
    // parse() returns a Result
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: String Parsing ===");

    let inputs = ["42", "100", "abc", "-7", "99999999999999999999", "0"];

    for input in inputs {
        match input.parse::<i32>() {
            Ok(n)  => println!("  '{}' → {}", input, n),
            Err(e) => println!("  '{}' → PARSE ERROR: {}", input, e),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 6: map() and map_err() on Result
    // map() transforms the Ok value
    // map_err() transforms the Err value
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: map() on Result ===");

    // parse "5" and then square it
    let result = "5".parse::<i32>()
        .map(|n| n * n);   // square it if parsing succeeds
    println!("parse '5' then square: {:?}", result);

    let result2 = "abc".parse::<i32>()
        .map(|n| n * n);   // won't execute because parse fails
    println!("parse 'abc' then square: {:?}", result2);

    // map_err — transform the error type
    let result3 = "42".parse::<i32>()
        .map_err(|e| format!("Custom error: {}", e));
    println!("map_err: {:?}", result3);

    // --------------------------------------------------------
    // EXERCISE 7: Result in real functions
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Functions returning Result ===");

    fn parse_age(s: &str) -> Result<u32, String> {
        match s.parse::<u32>() {
            Ok(age) => {
                if age > 150 {
                    Err(format!("Age {} is unrealistically high!", age))
                } else {
                    Ok(age)
                }
            },
            Err(e) => Err(format!("'{}' is not a valid age: {}", s, e)),
        }
    }

    let test_ages = ["25", "abc", "200", "0", "65"];
    for age_str in test_ages {
        match parse_age(age_str) {
            Ok(age) => println!("  Valid age: {}", age),
            Err(e)  => println!("  Error: {}", e),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 8: Converting Result to Option
    // .ok()  — converts Ok(v) to Some(v), Err to None
    // .err() — converts Err(e) to Some(e), Ok to None
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: Result ↔ Option ===");

    let r: Result<i32, &str> = Ok(42);
    let opt = r.ok();         // Some(42)
    println!("Ok(42).ok() = {:?}", opt);

    let r2: Result<i32, &str> = Err("oops");
    let opt2 = r2.ok();       // None
    println!("Err().ok()  = {:?}", opt2);

    // parse().ok() is a common pattern to ignore errors
    let number: Option<i32> = "123".parse().ok();
    let bad: Option<i32> = "abc".parse().ok();
    println!("parse '123'.ok() = {:?}", number);
    println!("parse 'abc'.ok() = {:?}", bad);

    // --------------------------------------------------------
    // EXERCISE 9: Collecting Results
    // --------------------------------------------------------
    println!("\n=== EXERCISE 9: Vec of Results ===");

    let strings = vec!["1", "2", "three", "4", "five"];

    // Collect all results (stops on first error):
    let all_parsed: Result<Vec<i32>, _> = strings.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("All parsed: {:?}", all_parsed);

    // Collect only successes, skip errors:
    let only_good: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Only good parses: {:?}", only_good);

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write a function `sqrt_positive(n: f64) -> Result<f64, String>`
    // Returns Err if n < 0, Ok(sqrt) otherwise

    // Challenge 2: Write a function `get_element(v: &Vec<i32>, i: usize) -> Result<i32, String>`
    // Returns Ok(element) if i is valid, Err("Index out of bounds") otherwise

    // Challenge 3: Chain operations:
    // "  42  ".trim().parse::<i32>() — handle the Result and print the number

    // Challenge 4: Given a Vec<&str> = ["10", "20", "thirty", "40"]
    // Sum only the valid numbers (skip errors) and print the sum

    // --------------------------------------------------------
    // SUMMARY:
    //   Result<T, E>          -> Ok(T) or Err(E)
    //   Ok(value)             -> success
    //   Err(error)            -> failure
    //   match r { Ok(v)=>.., Err(e)=>.. }  -> safe handling
    //   r.is_ok()             -> check for success
    //   r.is_err()            -> check for error
    //   r.unwrap()            -> get Ok value (panics on Err!)
    //   r.expect("msg")       -> unwrap with custom panic msg
    //   r.unwrap_or(default)  -> get Ok or default
    //   r.map(|v| ...)        -> transform Ok value
    //   r.map_err(|e| ...)    -> transform Err value
    //   r.ok()                -> convert to Option<T>
    //   "42".parse::<i32>()   -> common Result usage
    //   collect::<Result<Vec<_>,_>>() -> collect all or fail
    //   filter_map(|x| x.parse().ok()) -> skip errors
    // --------------------------------------------------------
}
