// ============================================================
// DAY 18: Option<T>
// Topic: Handling values that might not exist (no null in Rust!)
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. What is Option<T> and why it exists
//   2. Some(value) and None
//   3. Handling Option with match
//   4. Useful Option methods: unwrap, unwrap_or, is_some, is_none
//   5. if let with Option
//   6. map, and_then on Option (functional style)
//
// KEY INSIGHT:
//   Rust has NO null! Instead, use Option<T>:
//     Some(value) — there IS a value
//     None        — there is NO value (like null, but safe!)
//
//   The compiler FORCES you to handle None — no surprise crashes!
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-18 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Creating Option values
    // Option<T> is an enum: Some(T) or None
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Creating Options ===");

    let some_number: Option<i32> = Some(42);
    let no_number:   Option<i32> = None;
    let some_text:   Option<&str> = Some("hello");
    let no_text:     Option<&str> = None;

    println!("some_number: {:?}", some_number);
    println!("no_number:   {:?}", no_number);
    println!("some_text:   {:?}", some_text);
    println!("no_text:     {:?}", no_text);

    // --------------------------------------------------------
    // EXERCISE 2: Handling Option with match
    // The safest way — you MUST handle both cases
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: match on Option ===");

    fn print_option(opt: Option<i32>) {
        match opt {
            Some(value) => println!("Got a value: {}", value),
            None        => println!("No value found!"),
        }
    }

    print_option(Some(100));
    print_option(None);
    print_option(Some(-5));

    // --------------------------------------------------------
    // EXERCISE 3: Option methods
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Option Methods ===");

    let x: Option<i32> = Some(7);
    let y: Option<i32> = None;

    // is_some() and is_none() — check without consuming
    println!("x.is_some(): {}", x.is_some());
    println!("x.is_none(): {}", x.is_none());
    println!("y.is_some(): {}", y.is_some());
    println!("y.is_none(): {}", y.is_none());

    // unwrap() — gets the value, but PANICS if None!
    // Only use when you're SURE it's Some
    let val = x.unwrap();
    println!("x.unwrap(): {}", val);

    // unwrap_or(default) — safe unwrap with fallback value
    println!("x.unwrap_or(0): {}", x.unwrap_or(0));
    println!("y.unwrap_or(0): {}", y.unwrap_or(0));  // returns 0 safely

    // unwrap_or_else(|| expr) — fallback using a closure
    println!("y.unwrap_or_else: {}", y.unwrap_or_else(|| 42));

    // expect("message") — like unwrap but with a custom panic message
    let val2 = x.expect("x should have a value!");
    println!("x.expect(): {}", val2);

    // --------------------------------------------------------
    // EXERCISE 4: if let with Option
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: if let with Option ===");

    let user_input: Option<&str> = Some("Krishna");

    if let Some(name) = user_input {
        println!("Welcome, {}!", name);
    } else {
        println!("No name provided, using Guest");
    }

    let empty_input: Option<&str> = None;
    if let Some(name) = empty_input {
        println!("Name: {}", name);
    } else {
        println!("Using default: Guest");
    }

    // --------------------------------------------------------
    // EXERCISE 5: Functions that return Option
    // Use Option when a function might not find/compute a result
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Functions returning Option ===");

    fn find_first_even(numbers: &[i32]) -> Option<i32> {
        for &n in numbers {
            if n % 2 == 0 {
                return Some(n);
            }
        }
        None  // no even number found
    }

    fn safe_divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 {
            None  // can't divide by zero
        } else {
            Some(a / b)
        }
    }

    fn find_in_list<'a>(list: &'a [&str], target: &str) -> Option<usize> {
        for (i, &item) in list.iter().enumerate() {
            if item == target {
                return Some(i);
            }
        }
        None
    }

    let nums = [1, 3, 7, 8, 11, 14];
    match find_first_even(&nums) {
        Some(n) => println!("First even: {}", n),
        None    => println!("No even numbers found"),
    }

    let odd_nums = [1, 3, 5, 7];
    match find_first_even(&odd_nums) {
        Some(n) => println!("First even: {}", n),
        None    => println!("No even numbers in {:?}", odd_nums),
    }

    match safe_divide(10.0, 3.0) {
        Some(r) => println!("10 / 3 = {:.4}", r),
        None    => println!("Division failed"),
    }
    match safe_divide(10.0, 0.0) {
        Some(r) => println!("10 / 0 = {}", r),
        None    => println!("Cannot divide by zero!"),
    }

    let fruits = ["Apple", "Banana", "Cherry", "Mango"];
    match find_in_list(&fruits, "Cherry") {
        Some(i) => println!("'Cherry' found at index {}", i),
        None    => println!("Not found"),
    }
    match find_in_list(&fruits, "Grape") {
        Some(i) => println!("'Grape' found at index {}", i),
        None    => println!("'Grape' not in list"),
    }

    // --------------------------------------------------------
    // EXERCISE 6: map() — transform the value inside Option
    // If Some(x), apply a function and return Some(result)
    // If None, just return None (doesn't execute the function)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: map() on Option ===");

    let score: Option<i32> = Some(85);
    let doubled = score.map(|s| s * 2);
    let as_string = score.map(|s| s.to_string());

    println!("score:     {:?}", score);
    println!("doubled:   {:?}", doubled);
    println!("as_string: {:?}", as_string);

    let no_score: Option<i32> = None;
    let no_doubled = no_score.map(|s| s * 2);
    println!("no_score map result: {:?}", no_doubled);  // None

    // --------------------------------------------------------
    // EXERCISE 7: and_then() — chain Option operations
    // Like map, but the function itself returns Option
    // Useful for chaining multiple optional operations
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: and_then() (chaining) ===");

    fn parse_number(s: &str) -> Option<i32> {
        s.parse::<i32>().ok()
    }

    fn double_if_positive(n: i32) -> Option<i32> {
        if n > 0 { Some(n * 2) } else { None }
    }

    // Chain: "42" → parse → double if positive
    let result1 = parse_number("42").and_then(double_if_positive);
    let result2 = parse_number("-5").and_then(double_if_positive);
    let result3 = parse_number("abc").and_then(double_if_positive);

    println!("'42' → parse → double: {:?}", result1);
    println!("'-5' → parse → double: {:?}", result2);
    println!("'abc' → parse → double: {:?}", result3);

    // --------------------------------------------------------
    // EXERCISE 8: Option in structs
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: Option in Structs ===");

    #[derive(Debug)]
    struct User {
        name: String,
        email: Option<String>,     // email is optional
        phone: Option<String>,     // phone is optional
        age: Option<u32>,          // age is optional
    }

    impl User {
        fn new(name: &str) -> User {
            User {
                name: String::from(name),
                email: None,
                phone: None,
                age: None,
            }
        }

        fn with_email(mut self, email: &str) -> User {
            self.email = Some(String::from(email));
            self
        }

        fn with_age(mut self, age: u32) -> User {
            self.age = Some(age);
            self
        }

        fn contact_info(&self) {
            println!("User: {}", self.name);
            println!("  Email: {}", self.email.as_deref().unwrap_or("not provided"));
            println!("  Phone: {}", self.phone.as_deref().unwrap_or("not provided"));
            println!("  Age:   {}", self.age.map(|a| a.to_string()).unwrap_or(String::from("unknown")));
        }
    }

    let user1 = User::new("Krishna").with_email("krishna@example.com").with_age(25);
    let user2 = User::new("Anonymous");

    user1.contact_info();
    println!();
    user2.contact_info();

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write a function `last_element(v: &Vec<i32>) -> Option<i32>`
    // that returns the last element if it exists
    // Test with empty and non-empty vecs

    // Challenge 2: Write a function `parse_age(s: &str) -> Option<u32>`
    // that parses a string to u32. If the parsed value is > 150, return None.
    // (Hint: use .parse::<u32>().ok() then .filter(|&a| a <= 150))

    // Challenge 3: Given a Vec<Option<i32>>, collect only the Some values
    // into a new Vec<i32>. Print both original and filtered.
    // let mixed = vec![Some(1), None, Some(3), None, Some(5)];
    // let values: Vec<i32> = mixed.into_iter().flatten().collect();

    // Challenge 4: Use unwrap_or to provide defaults for:
    //   - Missing username (default: "guest")
    //   - Missing score (default: 0)
    //   - Missing level (default: 1)

    // --------------------------------------------------------
    // SUMMARY:
    //   Option<T>              -> enum: Some(T) or None
    //   Some(value)            -> there IS a value
    //   None                   -> no value (safe null)
    //   match opt { Some(v)=>., None=>. }  -> safe handling
    //   opt.is_some()          -> check if Some
    //   opt.is_none()          -> check if None
    //   opt.unwrap()           -> get value (panics if None!)
    //   opt.unwrap_or(default) -> safe get with fallback
    //   opt.unwrap_or_else(||..)-> safe get with closure fallback
    //   opt.expect("msg")      -> unwrap with custom panic msg
    //   opt.map(|v| ...)       -> transform the inner value
    //   opt.and_then(|v| ...)  -> chain Option-returning operations
    //   if let Some(v) = opt { }  -> short pattern match
    // --------------------------------------------------------
}
