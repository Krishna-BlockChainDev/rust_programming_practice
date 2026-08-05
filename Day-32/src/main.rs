// ============================================================
// DAY 32: DEFAULT TRAIT + DERIVE MACROS
// Topic: The Default trait, #[derive] macros, PartialEq, Clone, Copy
// Time: 30-45 minutes
// ============================================================
//
// THE DEFAULT TRAIT:
//   Provides a default value for a type.
//   You can either #[derive(Default)] or implement it manually.
//
//   trait Default {
//       fn default() -> Self;
//   }
//
// #[derive] MACROS — auto-implement common traits:
//   #[derive(Debug)]       — enables {:?} and {:#?} printing
//   #[derive(Clone)]       — enables .clone()
//   #[derive(Copy)]        — enables implicit copy (for simple types)
//   #[derive(PartialEq)]   — enables == and != operators
//   #[derive(Eq)]          — full equality (for types where PartialEq is total)
//   #[derive(PartialOrd)]  — enables <, >, <=, >=
//   #[derive(Ord)]         — full ordering
//   #[derive(Hash)]        — enables use as HashMap/HashSet key
//   #[derive(Default)]     — provides a default() constructor
//
// ============================================================

use std::fmt;

// ── Example 1: #[derive(Debug, Clone, PartialEq)] ────────────

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

// ── Example 2: #[derive(Default)] ────────────────────────────

// Default gives: Point2::default() => Point2 { x: 0.0, y: 0.0 }
#[derive(Debug, Default, Clone, PartialEq)]
struct Config {
    width:   u32,
    height:  u32,
    title:   String,
    debug:   bool,
    volume:  f32,
}

// ── Example 3: Manual Default implementation ─────────────────

#[derive(Debug, Clone, PartialEq)]
struct ServerConfig {
    host:     String,
    port:     u16,
    max_conn: u32,
    timeout:  u64,
}

// Manually implementing Default lets us choose sensible defaults
impl Default for ServerConfig {
    fn default() -> Self {
        ServerConfig {
            host:     String::from("localhost"),
            port:     8080,
            max_conn: 100,
            timeout:  30,
        }
    }
}

// ── Example 4: Builder pattern using Default ─────────────────

// This is a very common Rust pattern!
impl ServerConfig {
    // Start from defaults, then modify specific fields
    fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    fn with_host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }
    fn with_max_conn(mut self, max: u32) -> Self {
        self.max_conn = max;
        self
    }
}

// ── Example 5: PartialEq & Eq ────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

// ── Example 6: PartialOrd & Ord ──────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// ── Example 7: Copy trait ─────────────────────────────────────

// Copy means the value is copied on assignment (no move)
// Only valid for types that are entirely on the stack (no heap)
#[derive(Debug, Clone, Copy, PartialEq)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

// ── Example 8: Struct update syntax + Default ─────────────────

#[derive(Debug, Default)]
struct Player {
    name:   String,
    health: i32,
    score:  u64,
    level:  u32,
}

impl Player {
    fn new(name: &str) -> Self {
        Player {
            name: name.to_string(),
            health: 100,
            level: 1,
            ..Default::default()  // fill rest with defaults
        }
    }
}

// ─────────────────────────────────────────────────────────────

fn main() {
    println!("=== Day 32: Default Trait & Derive Macros ===\n");

    // ── Debug, Clone, PartialEq ───────────────────────────────
    println!("--- Debug, Clone, PartialEq ---");
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1.clone();          // Clone
    let p3 = Point { x: 5.0, y: 6.0 };

    println!("p1 = {:?}", p1);    // Debug
    println!("p2 = {:?}", p2);
    println!("p1 == p2: {}", p1 == p2);    // PartialEq
    println!("p1 == p3: {}", p1 == p3);

    // ── Default (derived) ─────────────────────────────────────
    println!("\n--- #[derive(Default)] ---");
    let config = Config::default();
    println!("Default config: {:#?}", config);

    // Partial override using struct update syntax
    let custom_config = Config {
        width:  1920,
        height: 1080,
        title:  String::from("My App"),
        ..Default::default()   // debug: false, volume: 0.0
    };
    println!("Custom config: {:#?}", custom_config);

    // ── Manual Default ────────────────────────────────────────
    println!("\n--- Manual Default Implementation ---");
    let server = ServerConfig::default();
    println!("Default server: {:#?}", server);

    // ── Builder pattern ───────────────────────────────────────
    println!("\n--- Builder Pattern ---");
    let custom_server = ServerConfig::default()
        .with_host("example.com")
        .with_port(443)
        .with_max_conn(500);
    println!("Custom server: {:#?}", custom_server);

    // ── Color enum with PartialEq ─────────────────────────────
    println!("\n--- Enum with PartialEq ---");
    let c1 = Color::Red;
    let c2 = Color::Red;
    let c3 = Color::Blue;
    let c4 = Color::Custom(255, 128, 0);
    println!("Red == Red: {}", c1 == c2);
    println!("Red == Blue: {}", c1 == c3);
    println!("Custom: {:?}", c4);

    // ── Version with Ord ──────────────────────────────────────
    println!("\n--- Version Ordering ---");
    let mut versions = vec![
        Version { major: 1, minor: 2, patch: 3 },
        Version { major: 2, minor: 0, patch: 0 },
        Version { major: 1, minor: 0, patch: 5 },
        Version { major: 1, minor: 2, patch: 0 },
    ];
    versions.sort();
    for v in &versions { println!("  {}", v); }
    println!("Latest: {}", versions.last().unwrap());

    // ── Copy trait ────────────────────────────────────────────
    println!("\n--- Copy Trait ---");
    let px1 = Pixel { r: 255, g: 0, b: 128 };
    let px2 = px1;           // Copy, px1 still usable!
    println!("px1: {:?}", px1);   // works because Pixel is Copy
    println!("px2: {:?}", px2);

    // ── Player with Default ───────────────────────────────────
    println!("\n--- Player with Default ---");
    let player = Player::new("Krishna");
    println!("{:#?}", player);

    // ── EXERCISES ─────────────────────────────────────────────
    println!("\n=== EXERCISES ===\n");

    // EXERCISE 1:
    // Create a struct `Rectangle` with fields: width: f64, height: f64
    // Derive: Debug, Clone, PartialEq, Default
    // Create two Rectangles using default(), then modify them
    // Check if they are equal using ==
    // TODO:




    // EXERCISE 2:
    // Create a struct `UserProfile` with fields:
    //   username: String, email: String, age: u32,
    //   is_active: bool, score: u64
    // Implement Default manually with these defaults:
    //   username: "anonymous", email: "none@none.com", age: 0,
    //   is_active: true, score: 0
    // Create a new profile using Default, then update only the username
    // using struct update syntax: UserProfile { username: "Krishna".to_string(), ..Default::default() }
    // TODO:




    // EXERCISE 3:
    // Create an enum `Priority` with variants: Low, Medium, High, Critical
    // Derive: Debug, Clone, PartialEq, Eq, PartialOrd, Ord
    // (The order of enum variants determines their ordering!)
    // Create a Vec of priorities, sort it, and print them
    // TODO:




    // EXERCISE 4:
    // Create a struct `Coordinate` with x: i32, y: i32
    // Derive: Debug, Clone, Copy, PartialEq
    // Demonstrate that Copy works by assigning coord1 to coord2,
    // then modifying coord2 (use a mutable variable),
    // and showing that coord1 is unchanged
    // TODO:




    // EXERCISE 5 (CHALLENGE — Builder Pattern):
    // Create a struct `EmailConfig` with fields:
    //   from:    String
    //   to:      String
    //   subject: String
    //   body:    String
    //   html:    bool
    //   cc:      Vec<String>
    // Implement Default manually
    // Add builder methods: .from(), .to(), .subject(), .body(), .html(), .cc()
    // Each method takes mut self and returns Self
    // Build and print a complete email config using the builder pattern
    // TODO:




    println!("\n=== Day 32 Complete! Default & Derive mastered! ===");
    println!("Tomorrow: Generics in Functions");
}
