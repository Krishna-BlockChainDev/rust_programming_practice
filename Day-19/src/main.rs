// ============================================================
// DAY 19: Vectors — Vec<T>
// Topic: Dynamic, growable lists in Rust
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Creating Vectors (Vec<T>)
//   2. Adding and removing elements
//   3. Accessing elements (indexing vs .get())
//   4. Iterating over vectors
//   5. Common Vec methods
//   6. Vec of structs
//
// Vec vs Array:
//   Array:  fixed size, stored on stack
//   Vec:    dynamic size, stored on heap — can grow/shrink
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-19 folder)
//
// ============================================================

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Creating a Vector
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Creating Vectors ===");

    // Method 1: Empty Vec, then push values
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1: {:?}", v1);

    // Method 2: vec! macro — easiest way
    let v2 = vec![10, 20, 30, 40, 50];
    println!("v2: {:?}", v2);

    // Method 3: Create with a size and default value
    let v3: Vec<i32> = vec![0; 5];   // [0, 0, 0, 0, 0]
    println!("v3: {:?}", v3);

    // Vec of Strings
    let v4 = vec!["apple", "banana", "cherry"];
    println!("v4: {:?}", v4);

    println!("v2 length: {}", v2.len());
    println!("v2 is_empty: {}", v2.is_empty());

    // --------------------------------------------------------
    // EXERCISE 2: Adding and Removing Elements
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Adding/Removing ===");

    let mut fruits = Vec::new();
    fruits.push("Apple");
    fruits.push("Banana");
    fruits.push("Cherry");
    fruits.push("Mango");
    println!("After push: {:?}", fruits);

    // pop() removes and returns the last element (as Option)
    let last = fruits.pop();
    println!("Popped: {:?}", last);
    println!("After pop: {:?}", fruits);

    // insert(index, value) — insert at specific position
    fruits.insert(1, "Blueberry");
    println!("After insert at 1: {:?}", fruits);

    // remove(index) — remove at specific position
    let removed = fruits.remove(0);
    println!("Removed index 0: '{}'", removed);
    println!("After remove: {:?}", fruits);

    // --------------------------------------------------------
    // EXERCISE 3: Accessing Elements
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Accessing Elements ===");

    let numbers = vec![10, 20, 30, 40, 50];

    // Direct indexing — PANICS if out of bounds!
    println!("First:  {}", numbers[0]);
    println!("Second: {}", numbers[1]);
    println!("Last:   {}", numbers[numbers.len() - 1]);

    // .get(index) — returns Option<&T> — SAFE, never panics
    println!("get(2):  {:?}", numbers.get(2));   // Some(30)
    println!("get(10): {:?}", numbers.get(10));  // None (out of bounds)

    if let Some(val) = numbers.get(2) {
        println!("Got value: {}", val);
    }

    // First and last safely
    println!("first(): {:?}", numbers.first());
    println!("last():  {:?}", numbers.last());

    // --------------------------------------------------------
    // EXERCISE 4: Iterating over Vectors
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Iteration ===");

    let scores = vec![85, 92, 78, 95, 60, 88];

    // Method 1: for loop with reference
    print!("Scores: ");
    for score in &scores {
        print!("{} ", score);
    }
    println!();

    // Method 2: with index using enumerate
    for (i, score) in scores.iter().enumerate() {
        println!("  Student {}: {}", i + 1, score);
    }

    // Method 3: Mutable iteration to modify elements
    let mut values = vec![1, 2, 3, 4, 5];
    for v in &mut values {
        *v *= 10;  // Multiply each by 10
    }
    println!("After *=10: {:?}", values);

    // --------------------------------------------------------
    // EXERCISE 5: Common Vec Methods
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Vec Methods ===");

    let mut nums = vec![5, 3, 8, 1, 9, 2, 7, 4, 6];
    println!("Original:  {:?}", nums);
    println!("Length:    {}", nums.len());

    nums.sort();
    println!("Sorted:    {:?}", nums);

    nums.reverse();
    println!("Reversed:  {:?}", nums);

    nums.dedup();  // removes consecutive duplicates (only after sort!)
    println!("Deduped:   {:?}", nums);

    // contains
    println!("Contains 7: {}", nums.contains(&7));
    println!("Contains 10: {}", nums.contains(&10));

    // iter() methods
    let sum: i32 = nums.iter().sum();
    let max = nums.iter().max().unwrap();
    let min = nums.iter().min().unwrap();
    println!("Sum: {}, Max: {}, Min: {}", sum, max, min);

    // retain — keep only elements satisfying a condition
    let mut evens = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    evens.retain(|&x| x % 2 == 0);
    println!("Even numbers: {:?}", evens);

    // --------------------------------------------------------
    // EXERCISE 6: Vec of Structs
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Vec of Structs ===");

    #[derive(Debug)]
    struct Student {
        name: String,
        score: u32,
    }

    impl Student {
        fn new(name: &str, score: u32) -> Student {
            Student { name: String::from(name), score }
        }
    }

    let mut students = vec![
        Student::new("Alice",   88),
        Student::new("Bob",     72),
        Student::new("Carol",   95),
        Student::new("David",   65),
        Student::new("Eve",     91),
    ];

    println!("All students:");
    for s in &students {
        println!("  {}: {}", s.name, s.score);
    }

    // Sort by score (descending)
    students.sort_by(|a, b| b.score.cmp(&a.score));
    println!("\nRanked by score:");
    for (i, s) in students.iter().enumerate() {
        println!("  {}. {} — {}", i + 1, s.name, s.score);
    }

    // Average score
    let avg: f64 = students.iter().map(|s| s.score as f64).sum::<f64>()
                    / students.len() as f64;
    println!("Average score: {:.2}", avg);

    // Find top student
    if let Some(top) = students.first() {
        println!("Top student: {} with {}", top.name, top.score);
    }

    // --------------------------------------------------------
    // EXERCISE 7: Extending and merging Vecs
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Extending Vecs ===");

    let mut v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    // extend — add all elements of another vec
    v1.extend(v2.iter());
    println!("Extended: {:?}", v1);

    // extend with a range
    v1.extend(7..=9);
    println!("Extended with range: {:?}", v1);

    // append — moves all elements from another vec
    let mut extra = vec![10, 11, 12];
    v1.append(&mut extra);
    println!("After append: {:?}", v1);
    println!("extra is now empty: {:?}", extra);

    // --------------------------------------------------------
    // EXERCISE 8: Slices from Vec
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: Vec Slices ===");

    let data = vec![100, 200, 300, 400, 500];
    let slice: &[i32] = &data[1..4];  // [200, 300, 400]
    println!("Slice [1..4]: {:?}", slice);

    fn sum_slice(s: &[i32]) -> i32 {
        s.iter().sum()
    }
    println!("Sum of full vec:  {}", sum_slice(&data));
    println!("Sum of slice 1-3: {}", sum_slice(&data[1..4]));

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Create a Vec<String> of 5 city names.
    // Sort them alphabetically, then print each with its index.

    // Challenge 2: Given a Vec<i32>, write a function that
    // returns a NEW Vec containing only numbers greater than average.

    // Challenge 3: Given a Vec<i32> = [1,2,3,4,5,6,7,8,9,10],
    // use retain() to keep only multiples of 3.
    // Then double each element using a mutable for loop.

    // Challenge 4: Create a Vec<(String, u32)> of (name, age) pairs.
    // Find the oldest person. Sort by age. Print all.

    // --------------------------------------------------------
    // SUMMARY:
    //   Vec::new()             -> empty vector
    //   vec![1,2,3]            -> vector from literal
    //   vec![0; n]             -> n copies of value
    //   v.push(x)              -> append to end
    //   v.pop()                -> remove+return last (Option)
    //   v.insert(i, x)         -> insert at index i
    //   v.remove(i)            -> remove at index i (returns value)
    //   v[i]                   -> index (panics if OOB!)
    //   v.get(i)               -> safe access (returns Option)
    //   v.len()                -> number of elements
    //   v.is_empty()           -> check if empty
    //   v.contains(&x)         -> check membership
    //   v.sort()               -> sort in-place
    //   v.reverse()            -> reverse in-place
    //   v.retain(|&x| cond)   -> keep matching elements
    //   v.extend(iter)         -> add elements from iterator
    //   v.first(), v.last()    -> safe access to ends
    //   for x in &v { }        -> iterate (borrowed)
    //   for x in &mut v { }    -> iterate (mutable)
    // --------------------------------------------------------
}
