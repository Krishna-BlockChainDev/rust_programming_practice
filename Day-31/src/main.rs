// ============================================================
// DAY 31: TRAITS — Defining and Implementing
// Topic: Traits (Rust's version of interfaces/type classes)
// Time: 30-45 minutes
// ============================================================
//
// WHAT IS A TRAIT?
//   A trait defines shared behavior across types.
//   Think of it like an "interface" in Java/Go, or a "protocol" in Swift.
//
// SYNTAX — DEFINING A TRAIT:
//   trait TraitName {
//       fn method_name(&self) -> ReturnType;          // required method
//       fn method_with_default(&self) -> String {    // default implementation
//           String::from("default")
//       }
//   }
//
// SYNTAX — IMPLEMENTING A TRAIT:
//   impl TraitName for TypeName {
//       fn method_name(&self) -> ReturnType {
//           // implementation
//       }
//   }
//
// KEY RULES:
//   1. You can define default implementations in a trait
//   2. Types can override default implementations
//   3. You can implement multiple traits for one type
//   4. Orphan rule: you can only implement a trait if EITHER
//      the trait OR the type is defined in your crate
//
// ============================================================

// ── Example 1: Simple Trait ───────────────────────────────────

// Define a trait called "Greet"
trait Greet {
    // Required method — every implementor must define this
    fn greet(&self) -> String;

    // Default method — implementors can override, but don't have to
    fn farewell(&self) -> String {
        String::from("Goodbye!")
    }

    // Default method that calls the required method
    fn greet_loudly(&self) -> String {
        self.greet().to_uppercase()
    }
}

struct Person {
    name: String,
    age: u32,
}

struct Robot {
    id: u32,
    model: String,
}

// Implement Greet for Person
impl Greet for Person {
    fn greet(&self) -> String {
        format!("Hello! I'm {} and I'm {} years old.", self.name, self.age)
    }

    // Override the default farewell
    fn farewell(&self) -> String {
        format!("See you later! — {}", self.name)
    }
}

// Implement Greet for Robot
impl Greet for Robot {
    fn greet(&self) -> String {
        format!("BEEP BOOP. I am Robot-{} model {}.", self.id, self.model)
    }
    // Robot uses the default farewell() and greet_loudly()
}

// ── Example 2: Trait with multiple methods ───────────────────

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;

    // Default method using the required methods
    fn describe(&self) -> String {
        format!("Area: {:.2}, Perimeter: {:.2}", self.area(), self.perimeter())
    }
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        // Heron's formula
        let s = self.perimeter() / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimeter(&self) -> f64 {
        self.a + self.b + self.c
    }
}

// ── Example 3: Trait as function parameter ───────────────────

// Using "impl Trait" syntax — the function accepts ANY type that implements Shape
fn print_shape_info(shape: &impl Shape) {
    println!("{}", shape.describe());
}

// ── Example 4: Implementing std traits (Display, Debug) ──────

use std::fmt;

struct Point {
    x: f64,
    y: f64,
}

// Implement the Display trait from std library
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Implement custom distance method
impl Point {
    fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

// ── Example 5: Multiple Traits ────────────────────────────────

trait Printable {
    fn print(&self);
}

trait Saveable {
    fn save(&self) -> String;
}

struct Document {
    title: String,
    content: String,
}

impl Printable for Document {
    fn print(&self) {
        println!("[PRINT] {}: {}", self.title, self.content);
    }
}

impl Saveable for Document {
    fn save(&self) -> String {
        format!("{{title: '{}', content: '{}'}}", self.title, self.content)
    }
}

// ─────────────────────────────────────────────────────────────

fn main() {
    println!("=== Day 31: Traits ===\n");

    // ── Trait examples ────────────────────────────────────────
    println!("--- Greet Trait ---");
    let person = Person { name: String::from("Krishna"), age: 25 };
    let robot  = Robot  { id: 42, model: String::from("R2D2") };

    println!("{}", person.greet());
    println!("{}", person.farewell());
    println!("{}", person.greet_loudly());
    println!();
    println!("{}", robot.greet());
    println!("{}", robot.farewell());         // uses default
    println!("{}", robot.greet_loudly());     // uses default

    println!("\n--- Shape Trait ---");
    let c = Circle    { radius: 5.0 };
    let r = Rectangle { width: 4.0, height: 6.0 };
    let t = Triangle  { a: 3.0, b: 4.0, c: 5.0 };

    print!("Circle    — "); print_shape_info(&c);
    print!("Rectangle — "); print_shape_info(&r);
    print!("Triangle  — "); print_shape_info(&t);

    println!("\n--- Display Trait ---");
    let p = Point { x: 3.0, y: 4.0 };
    println!("Point: {}", p);          // uses Display
    println!("Distance from origin: {:.2}", p.distance_from_origin());

    println!("\n--- Multiple Traits ---");
    let doc = Document {
        title:   String::from("My Rust Notes"),
        content: String::from("Traits are awesome!"),
    };
    doc.print();
    println!("Saved: {}", doc.save());

    // ── EXERCISES ─────────────────────────────────────────────
    println!("\n=== EXERCISES ===\n");

    // EXERCISE 1:
    // Define a trait called `Describable` with:
    //   - required method: fn name(&self) -> &str;
    //   - required method: fn color(&self) -> &str;
    //   - default method: fn describe(&self) -> String
    //     that returns "I am a {color} {name}"
    // Create structs: Apple, Car
    // Implement Describable for both
    // Call describe() on each
    // TODO:




    // EXERCISE 2:
    // Define a trait `Calculator` with methods:
    //   fn add(&self, a: f64, b: f64) -> f64
    //   fn subtract(&self, a: f64, b: f64) -> f64
    //   fn multiply(&self, a: f64, b: f64) -> f64
    //   fn divide(&self, a: f64, b: f64) -> Option<f64>  // None if b == 0
    // Create a struct `BasicCalc` and implement Calculator for it
    // Test all four operations
    // TODO:




    // EXERCISE 3:
    // Create a trait `Animal` with:
    //   fn name(&self) -> &str
    //   fn sound(&self) -> &str
    //   fn description(&self) -> String  (default: "{name} says {sound}")
    // Create: Dog, Cat, Cow structs (each with a name field)
    // Implement Animal for all three
    // Create a function: fn make_noise(animal: &impl Animal)
    //   that calls description() and prints it
    // Call make_noise() on each animal
    // TODO:




    // EXERCISE 4:
    // Implement the std::fmt::Display trait for Rectangle (from above)
    // so that println!("{}", rect) prints "Rectangle(4 x 6)"
    // Test it with println!
    // TODO:




    // EXERCISE 5 (CHALLENGE):
    // Create a trait `Summary` with:
    //   fn summarize_author(&self) -> String    (required)
    //   fn summarize(&self) -> String           (default: "Read more from {author}")
    // Create structs: Article { title, author, content }
    //               Tweet   { username, content }
    // Implement Summary for both
    //   - Article.summarize() should return "{title}, by {author}"
    //   - Tweet uses the default summarize() with their username
    // Create a function: fn notify(item: &impl Summary) that prints item.summarize()
    // TODO:




    println!("\n=== Day 31 Complete! Traits mastered! ===");
    println!("Tomorrow: Default trait implementations & Derive macros");
}
