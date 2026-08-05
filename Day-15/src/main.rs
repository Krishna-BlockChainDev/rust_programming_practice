// ============================================================
// DAY 15: Struct Methods & impl
// Topic: Adding behavior to your structs
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. impl blocks — where you add methods to a struct
//   2. Methods (&self, &mut self, self)
//   3. Associated functions (like ::new())
//   4. Multiple impl blocks
//   5. Method chaining
//
// KEY CONCEPT:
//   Methods are like functions BUT they belong to a struct.
//   The first parameter is always `self` (the instance itself).
//   &self   = read-only access to the struct
//   &mut self = read-write access to the struct
//   self    = takes ownership (consumes the struct)
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-15 folder)
//
// ============================================================

// ── Struct Definitions ────────────────────────────────────────

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// impl block — where all methods for Rectangle live
impl Rectangle {
    // ── Associated Function (no `self`) ──────────────────────
    // Called with Rectangle::new(w, h) — like a constructor
    // Does NOT take `self` as first parameter
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // Create a square (width == height)
    fn square(size: f64) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    // ── Methods (&self) ───────────────────────────────────────
    // &self means: borrow the struct (read-only access)

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn diagonal(&self) -> f64 {
        (self.width * self.width + self.height * self.height).sqrt()
    }

    // ── Method (&mut self) ────────────────────────────────────
    // &mut self means: borrow mutably (can modify fields)

    fn scale(&mut self, factor: f64) {
        self.width  *= factor;
        self.height *= factor;
    }

    fn set_width(&mut self, w: f64) {
        self.width = w;
    }
}

// ─────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }

    fn diameter(&self) -> f64 {
        2.0 * self.radius
    }

    fn grow(&mut self, amount: f64) {
        self.radius += amount;
    }
}

// ─────────────────────────────────────────────────────────────

#[derive(Debug)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, initial_balance: f64) -> BankAccount {
        BankAccount {
            owner: String::from(owner),
            balance: initial_balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited ₹{:.2}. New balance: ₹{:.2}", amount, self.balance);
        }
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if amount > self.balance {
            println!("Insufficient funds! Balance: ₹{:.2}", self.balance);
            return false;
        }
        self.balance -= amount;
        println!("Withdrew ₹{:.2}. New balance: ₹{:.2}", amount, self.balance);
        true
    }

    fn balance(&self) -> f64 {
        self.balance
    }

    fn print_statement(&self) {
        println!("Account: {}, Balance: ₹{:.2}", self.owner, self.balance);
    }
}

// ─────────────────────────────────────────────────────────────

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Associated functions (constructors)
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Associated Functions ===");

    // Call with :: syntax (no instance needed!)
    let r1 = Rectangle::new(10.0, 5.0);
    let r2 = Rectangle::new(8.0, 12.0);
    let sq = Rectangle::square(7.0);

    println!("r1: {:?}", r1);
    println!("r2: {:?}", r2);
    println!("sq: {:?}", sq);

    // --------------------------------------------------------
    // EXERCISE 2: Calling methods with dot notation
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Methods on Rectangle ===");

    println!("r1 area:      {:.2}", r1.area());
    println!("r1 perimeter: {:.2}", r1.perimeter());
    println!("r1 diagonal:  {:.2}", r1.diagonal());
    println!("r1 is_square: {}", r1.is_square());
    println!("sq is_square: {}", sq.is_square());

    println!("r1 can_hold r2: {}", r1.can_hold(&r2));
    println!("r2 can_hold r1: {}", r2.can_hold(&r1));

    // --------------------------------------------------------
    // EXERCISE 3: Mutable methods (&mut self)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Mutable Methods ===");

    let mut rect = Rectangle::new(4.0, 3.0);
    println!("Before scale: {:?}, area={:.2}", rect, rect.area());

    rect.scale(2.0);
    println!("After scale(2): {:?}, area={:.2}", rect, rect.area());

    rect.set_width(10.0);
    println!("After set_width(10): {:?}", rect);

    // --------------------------------------------------------
    // EXERCISE 4: Circle methods
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Circle Methods ===");

    let mut c = Circle::new(5.0);
    println!("Circle: {:?}", c);
    println!("  Area:          {:.4}", c.area());
    println!("  Circumference: {:.4}", c.circumference());
    println!("  Diameter:      {:.2}", c.diameter());

    c.grow(2.5);
    println!("After grow(2.5): {:?}", c);
    println!("  New Area: {:.4}", c.area());

    // --------------------------------------------------------
    // EXERCISE 5: BankAccount — real-world example
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: BankAccount Methods ===");

    let mut account = BankAccount::new("Krishna", 1000.0);
    account.print_statement();

    account.deposit(500.0);
    account.deposit(250.0);
    account.withdraw(200.0);
    account.withdraw(2000.0);  // should fail

    println!("Final balance: ₹{:.2}", account.balance());

    // --------------------------------------------------------
    // EXERCISE 6: Multiple structs working together
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Multiple Structs ===");

    let rectangles = vec![
        Rectangle::new(3.0, 4.0),
        Rectangle::new(10.0, 2.0),
        Rectangle::new(6.0, 7.0),
        Rectangle::new(1.0, 15.0),
    ];

    println!("Rectangles with areas:");
    for (i, r) in rectangles.iter().enumerate() {
        println!("  {}: {:?} → area = {:.2}", i + 1, r, r.area());
    }

    // Find rectangle with max area
    let largest = rectangles.iter().max_by(|a, b| {
        a.area().partial_cmp(&b.area()).unwrap()
    });
    if let Some(r) = largest {
        println!("Largest area: {:.2} → {:?}", r.area(), r);
    }

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Add a method `is_larger_than(&self, other: &Rectangle) -> bool`
    // to Rectangle, comparing areas

    // Challenge 2: Create a struct `Triangle` with sides a, b, c
    // Add methods: perimeter(), area() [use Heron's formula],
    // is_valid() [sum of any two sides > third]

    // Challenge 3: Create a struct `Counter` with a value field.
    // Add methods: increment(), decrement(), reset(), value() -> i32

    // Challenge 4: Add a method to BankAccount called `transfer(&mut self, other: &mut BankAccount, amount: f64)`
    // that moves money from self to other

    // --------------------------------------------------------
    // SUMMARY:
    //   impl StructName { ... }     -> impl block for methods
    //   fn method(&self) { }        -> immutable method
    //   fn method(&mut self) { }    -> mutable method
    //   fn method(self) { }         -> consuming method
    //   fn new(...) -> Self { }     -> associated constructor
    //   StructName::new(...)        -> call associated function
    //   instance.method()           -> call method
    //   self.field                  -> access own field in method
    //   Multiple impl blocks allowed for same struct
    // --------------------------------------------------------
}
