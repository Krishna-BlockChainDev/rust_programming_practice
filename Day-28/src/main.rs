// ============================================================
// DAY 28: File I/O Basics
// Topic: Reading and writing files in Rust
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. Writing to a file with fs::write and File::create
//   2. Reading a file with fs::read_to_string
//   3. Appending to files
//   4. Working with file paths
//   5. Error handling with file operations
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-28 folder)
//   Files will be created in the Day-28/ directory
//
// ============================================================

use std::fs;
use std::fs::File;
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

fn main() {
    println!("=== DAY 28: File I/O ===\n");

    // --------------------------------------------------------
    // EXERCISE 1: Writing a file — simplest way
    // fs::write creates (or overwrites) a file
    // --------------------------------------------------------
    println!("=== EXERCISE 1: Writing a File ===");

    let content = "Hello, File!\nThis is line 2.\nThis is line 3.\n";

    match fs::write("hello.txt", content) {
        Ok(()) => println!("Written to hello.txt successfully!"),
        Err(e) => println!("Error writing file: {}", e),
    }

    // --------------------------------------------------------
    // EXERCISE 2: Reading a file — simplest way
    // fs::read_to_string reads entire file into a String
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Reading a File ===");

    match fs::read_to_string("hello.txt") {
        Ok(contents) => {
            println!("File contents:");
            println!("---");
            print!("{}", contents);
            println!("---");
            println!("Total bytes: {}", contents.len());
        },
        Err(e) => println!("Error reading file: {}", e),
    }

    // --------------------------------------------------------
    // EXERCISE 3: Writing with File::create (more control)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Writing with File::create ===");

    match File::create("numbers.txt") {
        Ok(mut file) => {
            for i in 1..=10 {
                writeln!(file, "Number: {}", i).expect("Failed to write");
            }
            println!("Written numbers.txt with 10 lines");
        },
        Err(e) => println!("Error creating file: {}", e),
    }

    // --------------------------------------------------------
    // EXERCISE 4: Appending to a file
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: Appending to File ===");

    use std::fs::OpenOptions;
    match OpenOptions::new().append(true).open("hello.txt") {
        Ok(mut file) => {
            writeln!(file, "This line was appended!").expect("Failed to append");
            println!("Appended to hello.txt");
        },
        Err(e) => println!("Error opening for append: {}", e),
    }

    // Read back to verify
    let updated = fs::read_to_string("hello.txt").unwrap_or_default();
    println!("Updated file:\n{}", updated);

    // --------------------------------------------------------
    // EXERCISE 5: Reading line by line (efficient for large files)
    // --------------------------------------------------------
    println!("=== EXERCISE 5: Line-by-Line Reading ===");

    // Write a multi-line file first
    let csv_data = "name,age,city\nAlice,25,Mumbai\nBob,30,Delhi\nCarol,28,Bangalore\n";
    fs::write("people.csv", csv_data).expect("Failed to write CSV");

    match File::open("people.csv") {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut line_num = 0;
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        if line_num == 0 {
                            println!("Header: {}", l);
                        } else {
                            let parts: Vec<&str> = l.split(',').collect();
                            if parts.len() == 3 {
                                println!("  Name: {}, Age: {}, City: {}", parts[0], parts[1], parts[2]);
                            }
                        }
                        line_num += 1;
                    },
                    Err(e) => println!("Error reading line: {}", e),
                }
            }
        },
        Err(e) => println!("Error opening file: {}", e),
    }

    // --------------------------------------------------------
    // EXERCISE 6: Checking if a file/directory exists
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: Path Operations ===");

    let files_to_check = ["hello.txt", "numbers.txt", "nonexistent.txt", "people.csv"];
    for filename in files_to_check {
        let exists = Path::new(filename).exists();
        println!("  '{}' exists: {}", filename, exists);
    }

    // --------------------------------------------------------
    // EXERCISE 7: Writing structured data (simple CSV)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Writing CSV Data ===");

    #[derive(Debug)]
    struct Student {
        name: String,
        age: u32,
        score: f64,
    }

    let students = vec![
        Student { name: String::from("Alice"), age: 20, score: 92.5 },
        Student { name: String::from("Bob"),   age: 22, score: 78.0 },
        Student { name: String::from("Carol"), age: 21, score: 95.5 },
    ];

    match File::create("students.csv") {
        Ok(mut file) => {
            writeln!(file, "name,age,score").expect("write failed");
            for s in &students {
                writeln!(file, "{},{},{}", s.name, s.age, s.score).expect("write failed");
            }
            println!("Written students.csv");
        },
        Err(e) => println!("Error: {}", e),
    }

    // Read it back
    let csv_content = fs::read_to_string("students.csv").unwrap_or_default();
    println!("students.csv content:\n{}", csv_content);

    // --------------------------------------------------------
    // EXERCISE 8: Error handling for file not found
    // --------------------------------------------------------
    println!("=== EXERCISE 8: Error Handling ===");

    fn read_file_safe(path: &str) -> Result<String, String> {
        fs::read_to_string(path).map_err(|e| format!("Failed to read '{}': {}", path, e))
    }

    match read_file_safe("hello.txt") {
        Ok(content) => println!("hello.txt: {} bytes", content.len()),
        Err(e)      => println!("Error: {}", e),
    }

    match read_file_safe("does_not_exist.txt") {
        Ok(content) => println!("Content: {}", content),
        Err(e)      => println!("Error: {}", e),
    }

    // --------------------------------------------------------
    // CLEANUP: Remove created files
    // --------------------------------------------------------
    println!("\n=== Cleanup ===");
    for file in ["hello.txt", "numbers.txt", "people.csv", "students.csv"] {
        match fs::remove_file(file) {
            Ok(()) => println!("Removed {}", file),
            Err(e) => println!("Could not remove {}: {}", file, e),
        }
    }

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Write a function that writes a "log" with timestamp
    // Each call appends a line: "[2024-01-01] Log message here\n"
    // (Use a counter or line number instead of real timestamp for now)

    // Challenge 2: Read a file, count the number of words and lines
    // Print a report: "File: xxx, Lines: n, Words: w, Chars: c"

    // Challenge 3: Write a function that copies a file from src to dst
    // Read the source with fs::read_to_string, write to dest with fs::write

    // Challenge 4: Write a simple note-taking program:
    // Write notes to a file, read them back and display numbered

    // --------------------------------------------------------
    // SUMMARY:
    //   use std::fs;
    //   use std::io::{Write, BufRead, BufReader};
    //   fs::write("file.txt", "content")     -> create/overwrite
    //   fs::read_to_string("file.txt")       -> read all into String
    //   fs::remove_file("file.txt")          -> delete file
    //   File::create("file.txt")             -> create, get File handle
    //   File::open("file.txt")               -> open for reading
    //   OpenOptions::new().append(true).open() -> open for append
    //   writeln!(file, "...")                 -> write a line
    //   BufReader::new(file)                 -> buffered reader
    //   reader.lines()                       -> iterator over lines
    //   Path::new("file.txt").exists()       -> check existence
    //   All file ops return Result — always handle errors!
    // --------------------------------------------------------
}
