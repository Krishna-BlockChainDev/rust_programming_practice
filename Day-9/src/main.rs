// ============================================================
// DAY 9: Tuples & Arrays
// Topic: Fixed-size compound data types
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Tuples — group different types together
//   2. Tuple destructuring
//   3. Arrays — fixed-size list of SAME type
//   4. Array indexing and slicing
//   5. Iterating over arrays
//   6. Differences between arrays and Vectors (preview)
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-9 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // PART 1: TUPLES
    // --------------------------------------------------------

    // EXERCISE 1: Creating a tuple
    // Tuples can hold DIFFERENT types
    // Fixed size — you can't add or remove elements
    // --------------------------------------------------------
    println!("=== TUPLES ===");

    let person: (&str, u32, f64) = ("Krishna", 25, 5.9);
    println!("Person tuple: {:?}", person);

    // Accessing tuple elements with dot notation (index starts at 0)
    println!("Name:   {}", person.0);
    println!("Age:    {}", person.1);
    println!("Height: {}", person.2);

    // --------------------------------------------------------
    // EXERCISE 2: Tuple Destructuring
    // Unpack a tuple into individual variables
    // --------------------------------------------------------
    println!("\n=== Tuple Destructuring ===");

    let point = (10, 20);
    let (x, y) = point;  // destructure into x and y
    println!("Point: ({}, {})", x, y);

    let rgb = (255u8, 128u8, 0u8);   // orange color
    let (red, green, blue) = rgb;
    println!("RGB: red={}, green={}, blue={}", red, green, blue);

    // Ignore some values with _
    let data = (42, "hello", true, 3.14);
    let (num, text, _, _) = data;
    println!("Extracted: {} and '{}'", num, text);

    // --------------------------------------------------------
    // EXERCISE 3: Tuples as function return values
    // --------------------------------------------------------
    println!("\n=== Tuple Returns ===");

    fn get_name_age() -> (&'static str, u32) {
        ("Krishna", 25)
    }

    let (name, age) = get_name_age();
    println!("Name: {}, Age: {}", name, age);

    // Circle: return both area and circumference
    fn circle_info(radius: f64) -> (f64, f64) {
        let pi = std::f64::consts::PI;
        let area = pi * radius * radius;
        let circumference = 2.0 * pi * radius;
        (area, circumference)
    }

    let (area, circ) = circle_info(5.0);
    println!("Circle r=5: Area={:.2}, Circumference={:.2}", area, circ);

    // --------------------------------------------------------
    // EXERCISE 4: Unit tuple ()
    // The empty tuple () is called "unit" — it means "nothing"
    // Functions that return nothing implicitly return ()
    // --------------------------------------------------------
    let empty: () = ();
    println!("\nUnit tuple: {:?}", empty);

    // --------------------------------------------------------
    // PART 2: ARRAYS
    // --------------------------------------------------------

    // EXERCISE 5: Creating Arrays
    // Arrays have FIXED size and all elements must be the SAME type
    // Syntax: [type; size] or just list values
    // --------------------------------------------------------
    println!("\n=== ARRAYS ===");

    let numbers = [1, 2, 3, 4, 5];           // array of 5 i32 values
    let zeros: [i32; 5] = [0; 5];             // [0, 0, 0, 0, 0] — filled with zeros
    let grades: [u8; 4] = [85, 92, 78, 95];  // explicit type

    println!("numbers: {:?}", numbers);
    println!("zeros:   {:?}", zeros);
    println!("grades:  {:?}", grades);

    // --------------------------------------------------------
    // EXERCISE 6: Array indexing
    // Indexing starts at 0
    // Rust will PANIC (crash) if you access out of bounds (at runtime)
    // --------------------------------------------------------
    println!("\n=== Array Indexing ===");

    let fruits = ["Apple", "Banana", "Cherry", "Mango", "Orange"];
    println!("First fruit:  {}", fruits[0]);
    println!("Second fruit: {}", fruits[1]);
    println!("Last fruit:   {}", fruits[4]);

    // Array length
    println!("Total fruits: {}", fruits.len());

    // --------------------------------------------------------
    // EXERCISE 7: Modifying an array (must be mut)
    // --------------------------------------------------------
    println!("\n=== Mutable Array ===");

    let mut scores = [70, 85, 90, 60, 75];
    println!("Before: {:?}", scores);

    scores[0] = 80;   // change first element
    scores[3] = 95;   // change fourth element
    println!("After:  {:?}", scores);

    // --------------------------------------------------------
    // EXERCISE 8: Iterating over arrays
    // --------------------------------------------------------
    println!("\n=== Iterating Arrays ===");

    let temperatures = [22.5, 25.0, 19.8, 30.1, 27.3];

    // Method 1: for...in with reference
    print!("Temperatures: ");
    for temp in &temperatures {
        print!("{:.1} ", temp);
    }
    println!();

    // Method 2: with index using enumerate
    for (i, temp) in temperatures.iter().enumerate() {
        println!("Day {}: {:.1}°C", i + 1, temp);
    }

    // --------------------------------------------------------
    // EXERCISE 9: Array operations
    // --------------------------------------------------------
    println!("\n=== Array Operations ===");

    let data = [10, 20, 30, 40, 50];

    // Sum all elements
    let mut total = 0;
    for num in &data {
        total += num;
    }
    println!("Sum: {}", total);

    // Find maximum
    let mut max = data[0];
    for &num in &data {
        if num > max {
            max = num;
        }
    }
    println!("Max: {}", max);

    // Average
    let average = total as f64 / data.len() as f64;
    println!("Average: {:.1}", average);

    // --------------------------------------------------------
    // EXERCISE 10: Array Slices
    // A slice is a reference to a part of an array
    // Syntax: &array[start..end]  (end is exclusive)
    // --------------------------------------------------------
    println!("\n=== Array Slices ===");

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let first_three = &numbers[0..3];    // [1, 2, 3]
    let middle = &numbers[3..7];          // [4, 5, 6, 7]
    let last_three = &numbers[7..];       // [8, 9, 10]
    let all = &numbers[..];               // entire array as slice

    println!("First three: {:?}", first_three);
    println!("Middle:      {:?}", middle);
    println!("Last three:  {:?}", last_three);
    println!("All:         {:?}", all);

    // --------------------------------------------------------
    // EXERCISE 11: 2D Array (array of arrays)
    // --------------------------------------------------------
    println!("\n=== 2D Array (Matrix) ===");

    let matrix: [[i32; 3]; 3] = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];

    println!("Matrix:");
    for row in &matrix {
        for col in row {
            print!("{:3}", col);
        }
        println!();
    }
    println!("Center element: {}", matrix[1][1]);

    // --------------------------------------------------------
    // EXERCISE 12: Useful array methods
    // --------------------------------------------------------
    println!("\n=== Array Methods ===");

    let mut arr = [64, 34, 25, 12, 22, 11, 90];
    println!("Before sort: {:?}", arr);
    arr.sort();
    println!("After sort:  {:?}", arr);

    println!("Contains 25: {}", arr.contains(&25));
    println!("Min: {}", arr[0]);              // after sorting
    println!("Max: {}", arr[arr.len() - 1]); // after sorting

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Create a tuple for a book (title, author, year, price)
    // and print all fields
    // let book = ("The Rust Book", "Steve Klabnik", 2019, 0.0);
    // let (title, author, year, price) = book;

    // Challenge 2: Create an array of 7 weekday names
    // Print each with its number (Mon=1, Tue=2, ...)
    // let days = ["Monday", ...];
    // for (i, day) in days.iter().enumerate() { ... }

    // Challenge 3: Find the second largest number in an array
    // let nums = [34, 7, 23, 32, 5, 62];
    // Sort it and print nums[nums.len()-2]

    // Challenge 4: Create a 3x3 identity matrix
    // [[1,0,0],[0,1,0],[0,0,1]]
    // Print it in a nice format

    // --------------------------------------------------------
    // SUMMARY:
    //   (1, "hi", 3.0)        -> tuple with mixed types
    //   tuple.0, tuple.1      -> access tuple elements
    //   let (a, b) = tuple;   -> destructure a tuple
    //   [1, 2, 3]             -> array
    //   [0; 5]                -> array of five 0s
    //   arr[0]                -> index (starts at 0)
    //   arr.len()             -> number of elements
    //   &arr[1..3]            -> slice [index 1 and 2]
    //   arr.sort()            -> sort in place
    //   arr.contains(&val)    -> check if value exists
    //   for x in &arr { }     -> iterate with references
    //   arr.iter().enumerate() -> iterate with (index, value)
    // --------------------------------------------------------
}
