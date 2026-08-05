// ============================================================
// DAY 24: HashMaps — HashMap<K, V>
// Topic: Key-value store for fast lookups
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Creating HashMaps
//   2. Inserting, getting, updating, removing
//   3. Iterating over HashMaps
//   4. Entry API (insert if not exists)
//   5. Common HashMap patterns
//
// KEY CONCEPT:
//   HashMap stores key-value pairs. Given a key, you can look
//   up its value in O(1) average time.
//   Like a dictionary in Python, or a Map in Java/JS.
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-24 folder)
//
// ============================================================

use std::collections::HashMap;

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: Creating a HashMap
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Creating HashMap ===");

    // Method 1: Empty HashMap, then insert
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"),   78);
    scores.insert(String::from("Carol"), 88);
    scores.insert(String::from("David"), 65);
    println!("scores: {:?}", scores);

    // Method 2: Collect from iterators
    let names = vec!["Apple", "Banana", "Cherry"];
    let prices = vec![50, 30, 80];
    let fruit_map: HashMap<&str, i32> = names.into_iter().zip(prices.into_iter()).collect();
    println!("fruit_map: {:?}", fruit_map);

    // --------------------------------------------------------
    // EXERCISE 2: Accessing values
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Accessing Values ===");

    // .get() returns Option<&V> — safe access
    match scores.get("Alice") {
        Some(score) => println!("Alice's score: {}", score),
        None        => println!("Alice not found"),
    }

    match scores.get("Eve") {
        Some(score) => println!("Eve's score: {}", score),
        None        => println!("Eve not found in map"),
    }

    // Direct indexing — PANICS if key not found (use .get() instead!)
    // let s = scores["Alice"];  // OK only if you know it exists

    // contains_key
    println!("Contains 'Bob':    {}", scores.contains_key("Bob"));
    println!("Contains 'Frank':  {}", scores.contains_key("Frank"));

    // --------------------------------------------------------
    // EXERCISE 3: Inserting and Updating
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Insert & Update ===");

    let mut map: HashMap<&str, i32> = HashMap::new();

    // insert() — adds new OR overwrites existing
    map.insert("x", 10);
    map.insert("y", 20);
    println!("After insert: {:?}", map);

    // Overwrite existing key
    map.insert("x", 99);
    println!("After overwrite x: {:?}", map);

    // Entry API — insert ONLY if key doesn't exist
    // .entry(key).or_insert(value)
    map.entry("z").or_insert(30);    // inserts z=30
    map.entry("x").or_insert(100);   // does NOT overwrite (x already exists)
    println!("After entry: {:?}", map);

    // --------------------------------------------------------
    // EXERCISE 4: The Entry API (very useful!)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Entry API ===");

    let text = "hello world hello rust world hello";
    let mut word_count: HashMap<&str, u32> = HashMap::new();

    for word in text.split_whitespace() {
        // Get the count for this word, or insert 0, then add 1
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word counts:");
    let mut sorted: Vec<(&&str, &u32)> = word_count.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));  // sort by count descending
    for (word, count) in sorted {
        println!("  '{}': {}", word, count);
    }

    // --------------------------------------------------------
    // EXERCISE 5: Iterating over HashMaps
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Iteration ===");

    let mut capitals: HashMap<&str, &str> = HashMap::new();
    capitals.insert("India",    "New Delhi");
    capitals.insert("USA",      "Washington D.C.");
    capitals.insert("UK",       "London");
    capitals.insert("France",   "Paris");
    capitals.insert("Japan",    "Tokyo");

    // Iterate key-value pairs
    println!("Capitals:");
    let mut sorted_caps: Vec<(&&str, &&str)> = capitals.iter().collect();
    sorted_caps.sort_by_key(|&(k, _)| *k);
    for (country, capital) in &sorted_caps {
        println!("  {} → {}", country, capital);
    }

    // Iterate only keys
    print!("\nCountries: ");
    let mut countries: Vec<&&str> = capitals.keys().collect();
    countries.sort();
    for c in countries { print!("{} ", c); }
    println!();

    // Iterate only values
    print!("Capitals: ");
    let mut cap_list: Vec<&&str> = capitals.values().collect();
    cap_list.sort();
    for v in cap_list { print!("{} ", v); }
    println!();

    // --------------------------------------------------------
    // EXERCISE 6: Removing entries
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Removing Entries ===");

    let mut inventory: HashMap<&str, u32> = HashMap::new();
    inventory.insert("Apple",  50);
    inventory.insert("Banana", 30);
    inventory.insert("Cherry", 80);
    inventory.insert("Mango",  20);

    println!("Before: {:?}", inventory);

    // remove() — removes and returns Option<V>
    let removed = inventory.remove("Banana");
    println!("Removed 'Banana': {:?}", removed);
    println!("After remove: {:?}", inventory);

    // --------------------------------------------------------
    // EXERCISE 7: HashMap with struct values
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: HashMap with Struct Values ===");

    #[derive(Debug)]
    struct Student {
        grade: char,
        score: f64,
    }

    let mut students: HashMap<&str, Student> = HashMap::new();
    students.insert("Alice",  Student { grade: 'A', score: 92.5 });
    students.insert("Bob",    Student { grade: 'B', score: 78.0 });
    students.insert("Carol",  Student { grade: 'A', score: 95.0 });
    students.insert("David",  Student { grade: 'C', score: 65.5 });

    println!("Student Records:");
    let mut student_list: Vec<(&&str, &Student)> = students.iter().collect();
    student_list.sort_by_key(|&(k, _)| *k);
    for (name, s) in &student_list {
        println!("  {}: Grade {}, Score {:.1}", name, s.grade, s.score);
    }

    // Find highest scorer
    if let Some((name, s)) = students.iter().max_by(|a, b| {
        a.1.score.partial_cmp(&b.1.score).unwrap()
    }) {
        println!("Highest scorer: {} with {:.1}", name, s.score);
    }

    // --------------------------------------------------------
    // EXERCISE 8: Practical — Phone book
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: Phone Book ===");

    let mut phonebook: HashMap<String, Vec<String>> = HashMap::new();

    fn add_number(book: &mut HashMap<String, Vec<String>>, name: &str, number: &str) {
        book.entry(String::from(name))
            .or_insert_with(Vec::new)
            .push(String::from(number));
    }

    add_number(&mut phonebook, "Alice", "+91-9876543210");
    add_number(&mut phonebook, "Alice", "+91-9123456789");  // second number
    add_number(&mut phonebook, "Bob",   "+91-9988776655");
    add_number(&mut phonebook, "Carol", "+91-7766554433");

    for (name, numbers) in &phonebook {
        println!("  {}:", name);
        for num in numbers {
            println!("    {}", num);
        }
    }

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Count character frequency in a string
    // let s = "programming";
    // Count how many times each character appears
    // Sort and print: 'a' appears 2 times, 'g' appears 2 times, ...

    // Challenge 2: Create a HashMap<String, Vec<String>> to group
    // fruits by their starting letter:
    // "A" → ["Apple", "Apricot"], "B" → ["Banana", "Blueberry"], etc.

    // Challenge 3: Create a "grade book" HashMap<String, Vec<u32>>
    // with students and their multiple test scores.
    // Print each student's average score.

    // Challenge 4: Invert a HashMap (swap keys and values)
    // Given: {"a": 1, "b": 2, "c": 3}
    // Result: {1: "a", 2: "b", 3: "c"}

    // --------------------------------------------------------
    // SUMMARY:
    //   use std::collections::HashMap;
    //   HashMap::new()              -> empty map
    //   map.insert(key, val)        -> insert (overwrites if exists)
    //   map.get(&key)               -> Option<&V>
    //   map.contains_key(&key)      -> bool
    //   map.remove(&key)            -> Option<V>
    //   map.entry(key).or_insert(v) -> insert only if not present
    //   let cnt = map.entry(k).or_insert(0); *cnt += 1; -> count pattern
    //   map.iter()                  -> (&K, &V) pairs
    //   map.keys()                  -> just keys
    //   map.values()                -> just values
    //   map.len()                   -> number of entries
    // --------------------------------------------------------
}
