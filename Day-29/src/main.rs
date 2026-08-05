// ============================================================
// DAY 29: Command Line Arguments
// Topic: Reading arguments passed to your program
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. std::env::args() — read CLI arguments
//   2. Parsing arguments manually
//   3. Environment variables with std::env::var()
//   4. Simple argument validation
//
// HOW TO RUN:
//   $ cargo run                       -> no args
//   $ cargo run -- hello world        -> with args
//   $ cargo run -- --name Krishna     -> with flags
//   $ cargo run -- add 5 3            -> with sub-command
//
// ============================================================

use std::env;

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Reading all arguments
    // args() returns an iterator over String arguments
    // args()[0] is always the program name (executable path)
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Reading Arguments ===");

    let args: Vec<String> = env::args().collect();

    println!("Total arguments: {}", args.len());
    for (i, arg) in args.iter().enumerate() {
        println!("  args[{}] = '{}'", i, arg);
    }

    // --------------------------------------------------------
    // EXERCISE 2: Checking argument count
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Argument Count Check ===");

    if args.len() == 1 {
        println!("No extra arguments provided.");
        println!("Try: cargo run -- hello world");
    } else {
        println!("You provided {} extra argument(s):", args.len() - 1);
        for arg in &args[1..] {
            println!("  '{}'", arg);
        }
    }

    // --------------------------------------------------------
    // EXERCISE 3: Simple command dispatcher
    // Parse the first argument as a "command"
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Command Dispatcher ===");

    if args.len() >= 2 {
        let command = &args[1];
        match command.as_str() {
            "hello" => {
                let name = args.get(2).map(String::as_str).unwrap_or("World");
                println!("Hello, {}!", name);
            },
            "add" => {
                if args.len() >= 4 {
                    let a: Result<f64, _> = args[2].parse();
                    let b: Result<f64, _> = args[3].parse();
                    match (a, b) {
                        (Ok(x), Ok(y)) => println!("{} + {} = {}", x, y, x + y),
                        _ => println!("Error: 'add' needs two numbers"),
                    }
                } else {
                    println!("Usage: cargo run -- add <num1> <num2>");
                }
            },
            "shout" => {
                let words: Vec<&str> = args[2..].iter().map(String::as_str).collect();
                println!("{}", words.join(" ").to_uppercase());
            },
            "count" => {
                if args.len() >= 3 {
                    let text = &args[2];
                    println!("Characters: {}", text.len());
                    println!("Words: {}", text.split_whitespace().count());
                } else {
                    println!("Usage: cargo run -- count <text>");
                }
            },
            unknown => {
                println!("Unknown command: '{}'", unknown);
                println!("Available: hello, add, shout, count");
            }
        }
    } else {
        println!("No command given. Try: cargo run -- hello");
        println!("Commands: hello, add <n1> <n2>, shout <text>, count <text>");
    }

    // --------------------------------------------------------
    // EXERCISE 4: Named flags (--name value style)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Named Flags ===");

    fn get_flag(args: &[String], flag: &str) -> Option<String> {
        let flag_str = format!("--{}", flag);
        for i in 0..args.len() {
            if args[i] == flag_str {
                return args.get(i + 1).cloned();
            }
        }
        None
    }

    fn has_flag(args: &[String], flag: &str) -> bool {
        let flag_str = format!("--{}", flag);
        args.contains(&flag_str)
    }

    let name    = get_flag(&args, "name").unwrap_or_else(|| String::from("World"));
    let greeting = get_flag(&args, "greeting").unwrap_or_else(|| String::from("Hello"));
    let verbose = has_flag(&args, "verbose");

    println!("{}, {}!", greeting, name);
    if verbose {
        println!("(verbose mode: all details shown)");
    }

    // Try: cargo run -- --name Krishna --greeting Namaste --verbose

    // --------------------------------------------------------
    // EXERCISE 5: Environment variables
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Environment Variables ===");

    // Read a specific env var
    match env::var("HOME") {
        Ok(home) => println!("HOME = {}", home),
        Err(_)   => println!("HOME not set"),
    }

    match env::var("PATH") {
        Ok(path) => println!("PATH has {} chars", path.len()),
        Err(_)   => println!("PATH not set"),
    }

    // Custom env var with default
    let app_env = env::var("APP_ENV").unwrap_or_else(|_| String::from("development"));
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| String::from("info"));
    println!("APP_ENV:   {}", app_env);
    println!("LOG_LEVEL: {}", log_level);

    // Try: APP_ENV=production cargo run

    // --------------------------------------------------------
    // EXERCISE 6: Parsing numeric arguments
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Numeric Argument Parsing ===");

    fn parse_num_arg(args: &[String], index: usize) -> Result<f64, String> {
        args.get(index)
            .ok_or_else(|| format!("Missing argument at position {}", index))
            .and_then(|s| s.parse::<f64>()
                .map_err(|_| format!("'{}' is not a valid number", s)))
    }

    // Demo with simulated args
    let demo_args = vec![
        String::from("program"),
        String::from("42.5"),
        String::from("bad"),
        String::from("100"),
    ];

    for i in 1..demo_args.len() {
        match parse_num_arg(&demo_args, i) {
            Ok(n)  => println!("  arg[{}] = {} (valid number)", i, n),
            Err(e) => println!("  arg[{}] = ERROR: {}", i, e),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 7: Printing help message
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Help System ===");

    if has_flag(&args, "help") || has_flag(&args, "h") {
        print_help();
    } else {
        println!("Run with --help to see usage");
    }

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write a "mini calculator" that reads:
    //   cargo run -- 10 + 5    → prints 15
    //   cargo run -- 10 - 3    → prints 7
    //   cargo run -- 6 * 7     → prints 42
    //   cargo run -- 10 / 2    → prints 5

    // Challenge 2: Write a word counter that accepts a sentence as args:
    //   cargo run -- "the quick brown fox"
    //   Prints: Words: 4, Chars: 19, Unique: 4

    // Challenge 3: Add a --reverse flag:
    //   cargo run -- hello world --reverse
    //   → prints: "dlrow olleh"

    // Challenge 4: Build a simple config reader:
    //   Read APP_HOST, APP_PORT, APP_ENV from environment
    //   Print a formatted config summary

    // --------------------------------------------------------
    // SUMMARY:
    //   use std::env;
    //   env::args()              -> iterator over CLI args
    //   env::args().collect::<Vec<String>>() -> collect to Vec
    //   args[0]                  -> program name (always present)
    //   args[1..]                -> user-provided arguments
    //   args.get(i)              -> safe access (Option<&String>)
    //   arg.parse::<i32>()      -> parse string to number
    //   env::var("KEY")         -> read environment variable
    //   env::vars()             -> all environment variables
    // --------------------------------------------------------
}

fn print_help() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("USAGE: cargo run -- [COMMAND] [OPTIONS]");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("COMMANDS:");
    println!("  hello [name]          Greet a name");
    println!("  add <n1> <n2>         Add two numbers");
    println!("  shout <words>         Print in uppercase");
    println!("  count <text>          Count characters/words");
    println!("OPTIONS:");
    println!("  --name <name>         Set name");
    println!("  --greeting <text>     Set greeting");
    println!("  --verbose             Enable verbose mode");
    println!("  --help, -h            Show this help");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
