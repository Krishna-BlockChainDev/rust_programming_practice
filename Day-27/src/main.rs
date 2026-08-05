// ============================================================
// DAY 27: Crates & External Dependencies
// Topic: Using external libraries (crates) in your Rust projects
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. What is a crate?
//   2. Adding dependencies to Cargo.toml
//   3. Using the `rand` crate for random numbers
//   4. Semantic versioning (SemVer)
//   5. Cargo commands for dependency management
//
// SETUP:
//   This day's Cargo.toml has: rand = "0.8"
//   First run `cargo build` to download it, then `cargo run`
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-27 folder)
//   (First run may take a moment to download the crate)
//
// ============================================================

// Import from the rand crate
use rand::Rng;              // Trait for random number generation
use rand::seq::SliceRandom; // For shuffling slices

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Understanding Cargo.toml dependencies
    // --------------------------------------------------------
    println!("=== EXERCISE 1: About Crates ===");
    println!("A CRATE is a Rust library/package.");
    println!("You add dependencies in Cargo.toml under [dependencies].");
    println!("Cargo downloads them from crates.io automatically.");
    println!("Example: rand = \"0.8\"  means 'rand' version 0.8.x\n");

    // --------------------------------------------------------
    // EXERCISE 2: Basic Random Numbers with rand
    // --------------------------------------------------------
    println!("=== EXERCISE 2: Random Numbers ===");

    // Create a thread-local random number generator
    let mut rng = rand::thread_rng();

    // Generate random integers in a range
    let random_i32: i32 = rng.gen_range(1..=100);
    let random_u8:  u8  = rng.gen_range(0..=255);
    let random_f64: f64 = rng.gen_range(0.0..=1.0);

    println!("Random i32 (1-100):  {}", random_i32);
    println!("Random u8  (0-255):  {}", random_u8);
    println!("Random f64 (0.0-1.0): {:.4}", random_f64);

    // Generate multiple random numbers
    print!("5 random dice rolls: ");
    for _ in 0..5 {
        let roll: u8 = rng.gen_range(1..=6);
        print!("{} ", roll);
    }
    println!();

    // --------------------------------------------------------
    // EXERCISE 3: Random bool (coin flip)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Coin Flip ===");

    let mut heads = 0;
    let mut tails = 0;

    for _ in 0..100 {
        let is_heads: bool = rng.gen();  // 50% chance
        if is_heads { heads += 1; } else { tails += 1; }
    }
    println!("100 coin flips:");
    println!("  Heads: {}", heads);
    println!("  Tails: {}", tails);

    // --------------------------------------------------------
    // EXERCISE 4: Shuffling a Vec with rand
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Shuffling ===");

    let mut deck = vec![
        "Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10",
        "Jack", "Queen", "King"
    ];

    println!("Before shuffle: {:?}", deck);
    deck.shuffle(&mut rng);
    println!("After shuffle:  {:?}", deck);

    // Draw top 5 cards
    println!("Your hand: {:?}", &deck[..5]);

    // --------------------------------------------------------
    // EXERCISE 5: Choosing random elements
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Random Choice ===");

    let fruits = vec!["Apple", "Banana", "Cherry", "Mango", "Orange"];

    // Pick a random element
    if let Some(fruit) = fruits.choose(&mut rng) {
        println!("Random fruit: {}", fruit);
    }

    // Pick multiple without repetition
    let chosen: Vec<&&str> = fruits.choose_multiple(&mut rng, 3).collect();
    println!("3 random fruits: {:?}", chosen);

    // --------------------------------------------------------
    // EXERCISE 6: Number guessing game (mini project preview!)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Number Guessing Game Demo ===");

    let secret: u32 = rng.gen_range(1..=20);
    let guesses = [10, 15, secret, 5];  // Simulated guesses

    println!("(Secret number is: {} — normally hidden!)", secret);
    for &guess in &guesses {
        if guess < secret {
            println!("Guess {}: Too low!", guess);
        } else if guess > secret {
            println!("Guess {}: Too high!", guess);
        } else {
            println!("Guess {}: CORRECT! You found it!", guess);
            break;
        }
    }

    // --------------------------------------------------------
    // EXERCISE 7: Generating random strings
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Random Password Generator ===");

    fn generate_password(length: usize) -> String {
        let mut rng = rand::thread_rng();
        let charset: Vec<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$"
            .chars().collect();

        (0..length)
            .map(|_| charset[rng.gen_range(0..charset.len())])
            .collect()
    }

    println!("Generated passwords:");
    for _ in 0..3 {
        println!("  {}", generate_password(12));
    }

    // --------------------------------------------------------
    // EXERCISE 8: Statistics with random data
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: Random Data Statistics ===");

    let mut data: Vec<f64> = (0..1000)
        .map(|_| rng.gen_range(0.0..100.0))
        .collect();

    let sum: f64 = data.iter().sum();
    let mean = sum / data.len() as f64;

    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let min = data.first().unwrap();
    let max = data.last().unwrap();
    let median = data[data.len() / 2];

    println!("1000 random numbers (0.0-100.0):");
    println!("  Min:    {:.2}", min);
    println!("  Max:    {:.2}", max);
    println!("  Mean:   {:.2}", mean);
    println!("  Median: {:.2}", median);

    // --------------------------------------------------------
    // CARGO COMMANDS REFERENCE:
    // --------------------------------------------------------
    println!("\n=== CARGO COMMANDS ===");
    println!("cargo add rand            -> add dependency");
    println!("cargo build               -> compile (downloads deps)");
    println!("cargo update              -> update dependencies");
    println!("cargo tree                -> show dependency tree");
    println!("cargo search some_crate   -> search crates.io");

    // --------------------------------------------------------
    // SEMVER — SEMANTIC VERSIONING:
    // --------------------------------------------------------
    println!("\n=== SEMANTIC VERSIONING ===");
    println!("rand = \"0.8\"    -> 0.8.x (patch updates OK)");
    println!("rand = \"^0.8\"   -> same as above");
    println!("rand = \"=0.8.5\" -> exactly 0.8.5");
    println!("rand = \">=0.8\"  -> 0.8 or higher");
    println!("Version: MAJOR.MINOR.PATCH");
    println!("  PATCH: bug fixes (safe to update)");
    println!("  MINOR: new features, backward compatible");
    println!("  MAJOR: breaking changes");

    // --------------------------------------------------------
    // POPULAR CRATES TO EXPLORE LATER:
    // --------------------------------------------------------
    println!("\n=== POPULAR CRATES ===");
    println!("rand     -> random numbers");
    println!("serde    -> serialization (JSON, YAML, etc.)");
    println!("tokio    -> async runtime");
    println!("reqwest  -> HTTP client");
    println!("clap     -> CLI argument parsing");
    println!("regex    -> regular expressions");
    println!("chrono   -> dates and times");
    println!("log      -> logging framework");

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Simulate rolling two dice 1000 times
    // Count how many times each sum (2-12) appears
    // Print a frequency table sorted by sum

    // Challenge 2: Use rand to generate a list of 20 random names
    // (first select from a list of first names and last names)
    // Then sort them alphabetically

    // Challenge 3: Simulate a lottery:
    //   Draw 6 unique numbers from 1-49
    //   Print the "winning numbers" sorted
    //   Hint: use choose_multiple or generate and check uniqueness

    // Challenge 4: Monte Carlo estimation of PI:
    //   Generate 100,000 random points (x,y) in [-1,1] range
    //   Count points inside the unit circle (x²+y² ≤ 1)
    //   PI ≈ 4 * (inside count) / (total count)

    // --------------------------------------------------------
    // SUMMARY:
    //   Add to Cargo.toml: [dependencies]\nrand = "0.8"
    //   use rand::Rng;                    -> number generation trait
    //   use rand::seq::SliceRandom;       -> shuffle/choose
    //   let mut rng = rand::thread_rng(); -> get rng instance
    //   rng.gen_range(1..=6)              -> random in range
    //   rng.gen::<bool>()                 -> random bool
    //   vec.shuffle(&mut rng)             -> shuffle vec in place
    //   vec.choose(&mut rng)              -> random element (Option)
    //   vec.choose_multiple(&mut rng, n)  -> n random elements
    //   cargo run                         -> auto downloads deps
    // --------------------------------------------------------
}
