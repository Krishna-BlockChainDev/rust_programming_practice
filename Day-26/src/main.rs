// ============================================================
// DAY 26: Modules & `mod`
// Topic: Organizing code into logical units
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Defining modules with `mod`
//   2. `pub` — making things public
//   3. `use` — bringing names into scope
//   4. Nested modules
//   5. Module paths (:: syntax)
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-26 folder)
//
// ============================================================

// ── Module Definitions ────────────────────────────────────────

// A simple module
mod greetings {
    // Private by default — only accessible within this module
    fn secret_greeting() -> &'static str {
        "This is private!"
    }

    // `pub` makes it accessible from outside the module
    pub fn hello(name: &str) -> String {
        format!("Hello, {}!", name)
    }

    pub fn goodbye(name: &str) -> String {
        format!("Goodbye, {}!", name)
    }

    pub fn formal(name: &str) -> String {
        format!("Good day, {}. How do you do?", name)
    }
}

// Nested modules
mod math {
    // nested module — private to math unless pub
    pub mod basic {
        pub fn add(a: i32, b: i32) -> i32 { a + b }
        pub fn subtract(a: i32, b: i32) -> i32 { a - b }
        pub fn multiply(a: i32, b: i32) -> i32 { a * b }

        pub fn divide(a: i32, b: i32) -> Option<i32> {
            if b == 0 { None } else { Some(a / b) }
        }
    }

    pub mod advanced {
        pub fn power(base: i64, exp: u32) -> i64 {
            (0..exp).fold(1, |acc, _| acc * base)
        }

        pub fn factorial(n: u64) -> u64 {
            (1..=n).product()
        }

        pub fn is_prime(n: u64) -> bool {
            if n < 2 { return false; }
            if n == 2 { return true; }
            if n % 2 == 0 { return false; }
            let mut i = 3;
            while i * i <= n {
                if n % i == 0 { return false; }
                i += 2;
            }
            true
        }
    }

    // Public function in the math module itself
    pub fn pi() -> f64 { std::f64::consts::PI }
}

// A module with structs and enums
mod shapes {
    #[derive(Debug)]
    pub struct Circle {
        pub radius: f64,
    }

    #[derive(Debug)]
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }

        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    impl Rectangle {
        pub fn new(width: f64, height: f64) -> Rectangle {
            Rectangle { width, height }
        }

        pub fn area(&self) -> f64 {
            self.width * self.height
        }
    }
}

// Module with constants
mod constants {
    pub const GRAVITY: f64 = 9.81;          // m/s²
    pub const SPEED_OF_LIGHT: f64 = 3e8;   // m/s
    pub const PLANCK: f64 = 6.626e-34;     // J·s

    pub fn earth_weight(mass_kg: f64) -> f64 {
        mass_kg * GRAVITY
    }
}

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Using a module with full path
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Module Full Path ===");

    // Call functions using full module path
    let msg1 = greetings::hello("Krishna");
    let msg2 = greetings::goodbye("Alice");
    let msg3 = greetings::formal("Mr. Patel");
    println!("{}", msg1);
    println!("{}", msg2);
    println!("{}", msg3);

    // This would ERROR — secret_greeting is private:
    // greetings::secret_greeting();

    // --------------------------------------------------------
    // EXERCISE 2: Nested module paths
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Nested Modules ===");

    let sum = math::basic::add(10, 5);
    let diff = math::basic::subtract(10, 5);
    let product = math::basic::multiply(4, 7);
    println!("10 + 5 = {}", sum);
    println!("10 - 5 = {}", diff);
    println!("4 * 7 = {}", product);

    match math::basic::divide(20, 4) {
        Some(r) => println!("20 / 4 = {}", r),
        None    => println!("Division by zero!"),
    }

    println!("PI = {:.6}", math::pi());
    println!("2^10 = {}", math::advanced::power(2, 10));
    println!("6! = {}", math::advanced::factorial(6));

    // --------------------------------------------------------
    // EXERCISE 3: `use` — bringing names into scope
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: use keyword ===");

    // Import specific items
    use math::advanced::is_prime;
    use math::advanced::factorial;

    println!("Primes up to 20:");
    for n in 2..=20 {
        if is_prime(n) {
            print!("{} ", n);
        }
    }
    println!();
    println!("10! = {}", factorial(10));

    // Import everything from a module with *
    use math::basic::*;
    println!("Using imported functions: {} + {} = {}", 3, 7, add(3, 7));

    // --------------------------------------------------------
    // EXERCISE 4: Module with struct
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Module Structs ===");

    use shapes::{Circle, Rectangle};

    let c = Circle::new(5.0);
    let r = Rectangle::new(10.0, 4.0);
    println!("Circle: {:?}", c);
    println!("Circle area: {:.4}", c.area());
    println!("Rectangle: {:?}", r);
    println!("Rectangle area: {:.4}", r.area());

    // --------------------------------------------------------
    // EXERCISE 5: Module constants
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Module Constants ===");

    use constants::*;
    println!("Gravity:         {} m/s²", GRAVITY);
    println!("Speed of light:  {} m/s", SPEED_OF_LIGHT);
    println!("Planck constant: {} J·s", PLANCK);
    println!("Weight of 70kg on Earth: {} N", earth_weight(70.0));

    // --------------------------------------------------------
    // EXERCISE 6: Aliasing with `as`
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Aliasing ===");

    use greetings::hello as hi;    // rename hello to hi
    use greetings::goodbye as bye;

    println!("{}", hi("Bob"));
    println!("{}", bye("Carol"));

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Create a module `temperature` with:
    //   pub fn celsius_to_fahrenheit(c: f64) -> f64
    //   pub fn fahrenheit_to_celsius(f: f64) -> f64
    //   pub fn celsius_to_kelvin(c: f64) -> f64
    // Use it from main

    // Challenge 2: Create a module `string_utils` with:
    //   pub fn reverse(s: &str) -> String
    //   pub fn count_vowels(s: &str) -> usize
    //   pub fn capitalize(s: &str) -> String
    // Use `use` to import them

    // Challenge 3: Create a nested module `geometry::d2` and `geometry::d3`
    // d2: circle_area, rectangle_area, triangle_area
    // d3: sphere_volume, cube_volume

    // Challenge 4: What visibility errors occur if you:
    //   - Add a private field to Circle::radius
    //   - Try to access it from main directly?

    // --------------------------------------------------------
    // SUMMARY:
    //   mod name { }              -> define a module
    //   pub fn / pub struct       -> make public
    //   module::item              -> access via path
    //   use module::item;         -> bring into local scope
    //   use module::*;            -> import all public items
    //   use module::item as alias -> rename on import
    //   mod outer { mod inner { } } -> nested modules
    //   Items are private by default in Rust
    //   Only `pub` items are accessible from outside module
    // --------------------------------------------------------
}
