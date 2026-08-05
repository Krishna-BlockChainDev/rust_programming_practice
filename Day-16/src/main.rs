// ============================================================
// DAY 16: Enums
// Topic: Define a type with a fixed set of possible values
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Basic enums (variants with no data)
//   2. Enums with data in variants
//   3. match with enums
//   4. impl on enums (methods)
//   5. Real-world enum use cases
//
// KEY CONCEPT:
//   An enum defines a type that can be ONE of several variants.
//   Unlike structs (which hold all fields), enums hold ONE variant.
//   Rust enums are very powerful — variants can carry data!
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-16 folder)
//
// ============================================================

// ── Enum Definitions ─────────────────────────────────────────

// Simple enum — just variant names, no data
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Enum with data in variants — each variant can hold different data!
#[derive(Debug)]
enum Shape {
    Circle(f64),             // holds one f64 (radius)
    Rectangle(f64, f64),     // holds two f64s (width, height)
    Triangle(f64, f64, f64), // holds three f64s (sides)
}

// Enum variant with named fields (like a struct)
#[derive(Debug)]
enum Message {
    Quit,                          // no data
    Move { x: i32, y: i32 },     // named fields like struct
    Write(String),                 // single String
    ChangeColor(u8, u8, u8),      // three u8s (RGB)
}

// Traffic light example
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Coin example
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// ── impl on Enums ─────────────────────────────────────────────

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_)       => "Circle",
            Shape::Rectangle(_, _) => "Rectangle",
            Shape::Triangle(_, _, _) => "Triangle",
        }
    }
}

impl TrafficLight {
    fn duration_seconds(&self) -> u32 {
        match self {
            TrafficLight::Red    => 60,
            TrafficLight::Yellow => 5,
            TrafficLight::Green  => 45,
        }
    }

    fn action(&self) -> &str {
        match self {
            TrafficLight::Red    => "STOP",
            TrafficLight::Yellow => "SLOW DOWN",
            TrafficLight::Green  => "GO",
        }
    }

    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red    => TrafficLight::Green,
            TrafficLight::Green  => TrafficLight::Yellow,
            TrafficLight::Yellow => TrafficLight::Red,
        }
    }
}

impl Coin {
    fn value_paise(&self) -> u32 {
        match self {
            Coin::Penny   => 1,
            Coin::Nickel  => 5,
            Coin::Dime    => 10,
            Coin::Quarter => 25,
        }
    }
}

// ─────────────────────────────────────────────────────────────

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Basic Enum Usage
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Basic Enums ===");

    let dir = Direction::North;
    println!("Direction: {:?}", dir);

    // match on simple enum
    match dir {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        Direction::East  => println!("Heading East!"),
        Direction::West  => println!("Heading West!"),
    }

    // Using enum in a Vec
    let directions = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
    for d in &directions {
        println!("  {:?}", d);
    }

    // --------------------------------------------------------
    // EXERCISE 2: Enums with data
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Enums with Data ===");

    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for shape in &shapes {
        println!("{}: area = {:.2}", shape.name(), shape.area());
    }

    // Pattern matching with data extraction
    for shape in &shapes {
        match shape {
            Shape::Circle(r) => println!("Circle with radius {}", r),
            Shape::Rectangle(w, h) => println!("Rectangle {}x{}", w, h),
            Shape::Triangle(a, b, c) => println!("Triangle sides: {},{},{}", a, b, c),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 3: Message enum (complex variant types)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Message Enum ===");

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello from enum!")),
        Message::ChangeColor(255, 128, 0),
    ];

    for msg in &messages {
        match msg {
            Message::Quit => println!("Quit the program"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Write: '{}'", text),
            Message::ChangeColor(r, g, b) => println!("Color: rgb({},{},{})", r, g, b),
        }
    }

    // --------------------------------------------------------
    // EXERCISE 4: TrafficLight with methods
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Traffic Light ===");

    let mut light = TrafficLight::Red;
    for _ in 0..6 {
        println!("Light: {:?} → {} (for {}s)", light, light.action(), light.duration_seconds());
        light = light.next();
    }

    // --------------------------------------------------------
    // EXERCISE 5: Coin value calculator
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Coin Calculator ===");

    let wallet = vec![
        Coin::Quarter,
        Coin::Quarter,
        Coin::Dime,
        Coin::Nickel,
        Coin::Penny,
        Coin::Penny,
        Coin::Penny,
    ];

    let total: u32 = wallet.iter().map(|c| c.value_paise()).sum();
    println!("Coins in wallet:");
    for coin in &wallet {
        println!("  {:?} = {} paise", coin, coin.value_paise());
    }
    println!("Total: {} paise", total);

    // --------------------------------------------------------
    // EXERCISE 6: Enum comparison with == (needs PartialEq)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Enum Comparison ===");

    let current_light = TrafficLight::Green;
    if current_light == TrafficLight::Green {
        println!("Go! The light is green.");
    }

    // --------------------------------------------------------
    // EXERCISE 7: Using enum as function parameter
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Enum in Functions ===");

    fn describe_direction(d: &Direction) -> String {
        match d {
            Direction::North => format!("You are going UP"),
            Direction::South => format!("You are going DOWN"),
            Direction::East  => format!("You are going RIGHT"),
            Direction::West  => format!("You are going LEFT"),
        }
    }

    for d in &[Direction::North, Direction::East, Direction::South, Direction::West] {
        println!("{}", describe_direction(d));
    }

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Create an enum `Season` with 4 variants.
    // Add a method that returns typical weather description for each.

    // Challenge 2: Create an enum `DayType` with Weekday(String) and Weekend.
    // Given a day name, classify it and print the type.

    // Challenge 3: Create an enum `MathOperation` with:
    //   Add(f64, f64), Subtract(f64, f64), Multiply(f64, f64), Divide(f64, f64)
    // Add a method `calculate(&self) -> Option<f64>` (None for divide by zero)

    // Challenge 4: Create an enum `CardSuit` (Hearts, Diamonds, Clubs, Spades)
    // and `CardValue` (Ace, Number(u8), Jack, Queen, King)
    // Print all 4 suits with their point values

    // --------------------------------------------------------
    // SUMMARY:
    //   enum Name { Variant1, Variant2 }         -> basic enum
    //   enum Name { V1(T), V2(T1, T2) }          -> enum with data
    //   enum Name { V { field: T } }              -> named fields
    //   match value { Variant => ..., }           -> match on enum
    //   match value { V(x) => use x, }            -> extract data
    //   match value { V { field } => ... }        -> named field extract
    //   impl EnumName { fn method(&self) { } }   -> methods on enum
    //   #[derive(Debug, PartialEq)]               -> enable print & ==
    // --------------------------------------------------------
}
