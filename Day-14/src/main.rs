// ============================================================
// DAY 14: Structs
// Topic: Creating custom data types with named fields
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Defining a struct
//   2. Creating struct instances
//   3. Accessing and modifying fields
//   4. Struct update syntax
//   5. Tuple structs
//   6. Unit-like structs
//   7. Printing structs with #[derive(Debug)]
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-14 folder)
//
// ============================================================

// ── Struct Definitions (outside main) ────────────────────────

// Basic struct — named fields with types
// #[derive(Debug)] lets us print the struct with {:?}
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    email: String,
    is_active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// Tuple struct — like a tuple but with a name
// Used when field names would be redundant
#[derive(Debug)]
struct Color(u8, u8, u8);  // RGB color

#[derive(Debug)]
struct Meters(f64);         // Newtype pattern — wraps f64

// Unit-like struct — no fields
// Used as markers or for implementing traits
struct AlwaysEqual;

// ─────────────────────────────────────────────────────────────

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Creating a struct instance
    // Use the struct name and provide values for ALL fields
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Creating Structs ===");

    let user1 = Person {
        name: String::from("Krishna"),
        age: 25,
        email: String::from("krishna@example.com"),
        is_active: true,
    };

    // Accessing fields with dot notation
    println!("Name:   {}", user1.name);
    println!("Age:    {}", user1.age);
    println!("Email:  {}", user1.email);
    println!("Active: {}", user1.is_active);

    // Printing the whole struct (needs #[derive(Debug)])
    println!("Full struct: {:?}", user1);
    println!("Pretty print: {:#?}", user1);  // {:#?} is "pretty" debug

    // --------------------------------------------------------
    // EXERCISE 2: Mutable struct — changing fields
    // NOTE: The ENTIRE struct must be mut (not individual fields)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Mutable Struct ===");

    let mut rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    println!("Before: {:?}", rect);
    rect.width  = 15.0;  // modify field
    rect.height = 8.0;
    println!("After:  {:?}", rect);

    // --------------------------------------------------------
    // EXERCISE 3: Functions that create structs
    // A function that returns a struct is called a "constructor"
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Struct Constructor Function ===");

    fn create_person(name: &str, age: u32, email: &str) -> Person {
        Person {
            name: String::from(name),     // explicit field names
            age,                          // shorthand when param name == field name
            email: String::from(email),
            is_active: true,              // default value
        }
    }

    let user2 = create_person("Alice", 30, "alice@example.com");
    println!("user2: {:?}", user2);

    // --------------------------------------------------------
    // EXERCISE 4: Field init shorthand
    // When variable name matches field name, you can skip "field: variable"
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Field Init Shorthand ===");

    let name = String::from("Bob");
    let age = 28;
    let email = String::from("bob@example.com");
    let is_active = true;

    let user3 = Person { name, age, email, is_active };  // shorthand!
    println!("user3: {:#?}", user3);

    // --------------------------------------------------------
    // EXERCISE 5: Struct update syntax
    // Create a new struct based on an existing one,
    // but change some fields. Use `..existing_instance`
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Struct Update Syntax ===");

    let user4 = Person {
        email: String::from("newuser@example.com"),  // override email
        age: 22,                                      // override age
        ..user2  // take remaining fields (name, is_active) from user2
    };
    println!("user4: {:#?}", user4);
    // Note: user2's name was MOVED into user4, so user2.name is no longer valid
    // (But user2.age and user2.is_active are still valid as they're Copy types)

    // --------------------------------------------------------
    // EXERCISE 6: Tuple Structs
    // Fields accessed by position (.0, .1, .2)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Tuple Structs ===");

    let red   = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue  = Color(0, 0, 255);
    let custom = Color(128, 64, 200);

    println!("Red:    ({}, {}, {})", red.0, red.1, red.2);
    println!("Green:  {:?}", green);
    println!("Blue:   {:?}", blue);
    println!("Custom: {:?}", custom);

    let distance = Meters(5.75);
    println!("Distance: {} meters", distance.0);

    // Destructuring tuple structs
    let Color(r, g, b) = custom;
    println!("Destructured: r={}, g={}, b={}", r, g, b);

    // --------------------------------------------------------
    // EXERCISE 7: Structs in calculations
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Rectangle Calculator ===");

    let r1 = Rectangle { width: 12.5, height: 8.0 };
    let r2 = Rectangle { width: 5.0, height: 20.0 };

    fn area(r: &Rectangle) -> f64 {
        r.width * r.height
    }

    fn perimeter(r: &Rectangle) -> f64 {
        2.0 * (r.width + r.height)
    }

    fn can_hold(outer: &Rectangle, inner: &Rectangle) -> bool {
        outer.width > inner.width && outer.height > inner.height
    }

    println!("r1: {:?}", r1);
    println!("  Area:      {:.2}", area(&r1));
    println!("  Perimeter: {:.2}", perimeter(&r1));

    println!("r2: {:?}", r2);
    println!("  Area:      {:.2}", area(&r2));
    println!("  Perimeter: {:.2}", perimeter(&r2));

    println!("Can r1 hold r2? {}", can_hold(&r1, &r2));
    println!("Can r2 hold r1? {}", can_hold(&r2, &r1));

    // --------------------------------------------------------
    // EXERCISE 8: Structs in a Vec (collection of structs)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: Vec of Structs ===");

    let mut students: Vec<Person> = Vec::new();
    students.push(create_person("Alice", 20, "alice@uni.edu"));
    students.push(create_person("Bob",   22, "bob@uni.edu"));
    students.push(create_person("Carol", 21, "carol@uni.edu"));

    println!("Student List:");
    for (i, student) in students.iter().enumerate() {
        println!("  {}. {} (age: {})", i + 1, student.name, student.age);
    }

    // --------------------------------------------------------
    // EXERCISE 9: Point distance calculation
    // --------------------------------------------------------
    println!("\n=== EXERCISE 9: Point Distance ===");

    fn distance(p1: &Point, p2: &Point) -> f64 {
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        (dx * dx + dy * dy).sqrt()
    }

    let origin = Point { x: 0.0, y: 0.0 };
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 6.0, y: 8.0 };

    println!("origin: {:?}", origin);
    println!("p1:     {:?}", p1);
    println!("p2:     {:?}", p2);
    println!("Distance origin→p1: {:.2}", distance(&origin, &p1));
    println!("Distance p1→p2:     {:.2}", distance(&p1, &p2));

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Create a struct `BankAccount` with fields:
    //   owner: String, balance: f64, account_number: u64
    // Create two accounts and print them

    // Challenge 2: Create a struct `Student` with:
    //   name: String, grade: u8, marks: [u32; 5]
    // Write a function that takes &Student and prints their average mark

    // Challenge 3: Create a tuple struct `Celsius(f64)` and `Fahrenheit(f64)`
    // Write a function to convert between them

    // Challenge 4: Create a Vec of 5 Rectangle structs
    // Find and print the one with the largest area

    // --------------------------------------------------------
    // SUMMARY:
    //   struct Name { field: Type, ... }  -> define a struct
    //   let x = Name { field: value, .. } -> create instance
    //   x.field                           -> access field
    //   let mut x = Name { ... }          -> mutable instance
    //   x.field = new_value;              -> modify field
    //   Name { field, .. }                -> shorthand init
    //   Name { field: val, ..existing }   -> update syntax
    //   struct Wrapper(Type);             -> tuple struct
    //   wrapper.0                         -> access tuple struct field
    //   #[derive(Debug)]                  -> enable {:?} printing
    //   {:#?}                             -> pretty-print structs
    // --------------------------------------------------------
}
