// ============================================================
// DAY 7: Loops — loop, while, for
// Topic: Repeating actions in Rust
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. `loop` — infinite loop (until break)
//   2. `while` — loop with condition
//   3. `for` — loop over a range or collection
//   4. `break` — exit a loop
//   5. `continue` — skip current iteration
//   6. Returning values from loops
//   7. Loop labels (nested loop control)
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-7 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: `loop` — Infinite loop + break
    // `loop` runs forever until you use `break`
    // Useful when you don't know how many times to repeat
    // --------------------------------------------------------
    println!("=== loop example ===");
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 5 {
            println!("Reached 5, breaking out!");
            break;  // Exit the loop
        }
    }

    // --------------------------------------------------------
    // EXERCISE 2: Returning a value from `loop`
    // You can return a value by putting it after `break`
    // --------------------------------------------------------
    println!("\n=== loop with return value ===");
    let mut attempts = 0;

    let result = loop {
        attempts += 1;

        if attempts == 3 {
            break attempts * 10;  // Return this value from loop
        }
    };
    println!("Loop returned: {} after {} attempts", result, attempts);

    // --------------------------------------------------------
    // EXERCISE 3: `while` loop
    // Runs while a condition is true
    // --------------------------------------------------------
    println!("\n=== while loop ===");
    let mut number = 10;

    while number > 0 {
        print!("{} ", number);
        number -= 2;
    }
    println!(); // Just prints a newline

    // Countdown example
    println!("Countdown:");
    let mut countdown = 5;
    while countdown > 0 {
        println!("{} ...", countdown);
        countdown -= 1;
    }
    println!("Blast off! 🚀");

    // --------------------------------------------------------
    // EXERCISE 4: `for` loop with a range
    // Range syntax:
    //   1..5  = 1, 2, 3, 4       (excludes 5)
    //   1..=5 = 1, 2, 3, 4, 5    (includes 5)
    // --------------------------------------------------------
    println!("\n=== for loop with range ===");

    // Exclusive range (1 to 4)
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    // Inclusive range (1 to 5)
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();

    // Looping backwards using rev()
    println!("Reverse:");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();

    // --------------------------------------------------------
    // EXERCISE 5: `for` loop over an array
    // --------------------------------------------------------
    println!("\n=== for loop over array ===");
    let fruits = ["Apple", "Banana", "Cherry", "Mango", "Orange"];

    for fruit in &fruits {
        println!("Fruit: {}", fruit);
    }

    // With index using enumerate()
    println!("\nWith index:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}: {}", index + 1, fruit);
    }

    // --------------------------------------------------------
    // EXERCISE 6: `continue` — skip an iteration
    // continue skips the rest of the current loop body
    // and goes to the next iteration
    // --------------------------------------------------------
    println!("\n=== continue example (skip even numbers) ===");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;  // Skip even numbers
        }
        print!("{} ", i);  // Only odd numbers print
    }
    println!();

    // --------------------------------------------------------
    // EXERCISE 7: Nested loops with labels
    // Use 'label: loop to name a loop
    // Then `break 'label` to break out of a specific loop
    // --------------------------------------------------------
    println!("\n=== Nested loops with labels ===");

    'outer: for x in 1..=3 {
        for y in 1..=3 {
            if x == 2 && y == 2 {
                println!("Breaking outer loop at x={}, y={}", x, y);
                break 'outer;  // breaks the OUTER loop entirely
            }
            println!("x={}, y={}", x, y);
        }
    }

    // --------------------------------------------------------
    // EXERCISE 8: Multiplication table (practical!)
    // --------------------------------------------------------
    println!("\n=== Multiplication Table (1 to 5) ===");
    for i in 1..=5 {
        for j in 1..=5 {
            print!("{:4}", i * j);  // {:4} = right-aligned, 4 chars wide
        }
        println!();
    }

    // --------------------------------------------------------
    // EXERCISE 9: Sum using a loop
    // --------------------------------------------------------
    println!("\n=== Sum of 1 to 100 ===");
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("Sum = {}", sum);  // Should be 5050

    // --------------------------------------------------------
    // EXERCISE 10: Factorial using while loop
    // --------------------------------------------------------
    println!("\n=== Factorial ===");
    let n = 6;
    let mut factorial: u64 = 1;
    let mut i = 1;

    while i <= n {
        factorial *= i;
        i += 1;
    }
    println!("{}! = {}", n, factorial);  // 6! = 720

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Print all numbers from 1 to 50 that are divisible by 3
    // for i in 1..=50 {
    //     if i % 3 == 0 { print!("{} ", i); }
    // }

    // Challenge 2: Calculate the sum of even numbers from 1 to 100
    // let mut even_sum = 0;
    // for i in 1..=100 { if i % 2 == 0 { even_sum += i; } }
    // println!("Sum of evens: {}", even_sum);

    // Challenge 3: Use loop to find first number greater than 1000 divisible by 7
    // let mut num = 1;
    // let found = loop {
    //     if num > 1000 && num % 7 == 0 { break num; }
    //     num += 1;
    // };
    // println!("Found: {}", found);

    // Challenge 4: Print a right triangle pattern using nested for loops
    // for i in 1..=5 {
    //     for _ in 1..=i { print!("* "); }
    //     println!();
    // }

    // --------------------------------------------------------
    // SUMMARY:
    //   loop { }                  -> infinite loop, needs break
    //   break value;              -> exit loop and return value
    //   while condition { }       -> loop while condition is true
    //   for x in 1..5 { }        -> range (1,2,3,4)
    //   for x in 1..=5 { }       -> inclusive range (1,2,3,4,5)
    //   for x in collection { }  -> iterate over items
    //   continue;                 -> skip to next iteration
    //   break;                    -> exit loop
    //   'label: loop { }         -> named loop
    //   break 'label;            -> break a specific loop
    //   .enumerate()             -> get (index, value) pairs
    //   .rev()                   -> reverse a range
    // --------------------------------------------------------
}
