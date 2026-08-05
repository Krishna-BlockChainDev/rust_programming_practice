// ============================================================
// DAY 30: MINI PROJECT — File Word Counter & Text Analyzer
// Topic: Apply File I/O + HashMaps + Error Handling + Modules
// Time: 30-45 minutes
// ============================================================
//
// PROJECT: A text analysis tool that:
//   1. Creates sample text files
//   2. Reads and analyzes text files
//   3. Counts words, lines, characters
//   4. Finds most frequent words
//   5. Detects unique words
//   6. Generates a full analysis report
//
// CONCEPTS USED:
//   ✅ Day 21 — Result<T,E>
//   ✅ Day 22 — ? operator
//   ✅ Day 23 — Custom errors
//   ✅ Day 24 — HashMap
//   ✅ Day 25 — HashSet
//   ✅ Day 26 — Modules (inline)
//   ✅ Day 28 — File I/O
//   ✅ Day 29 — String processing
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-30 folder)
//
// ============================================================

use std::collections::{HashMap, HashSet};
use std::fs;
use std::fmt;

// ── Custom Error ──────────────────────────────────────────────

#[derive(Debug)]
enum AnalyzerError {
    FileNotFound(String),
    ReadError(String),
    WriteError(String),
    EmptyFile(String),
}

impl fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnalyzerError::FileNotFound(p) => write!(f, "File not found: '{}'", p),
            AnalyzerError::ReadError(e)    => write!(f, "Error reading file: {}", e),
            AnalyzerError::WriteError(e)   => write!(f, "Error writing file: {}", e),
            AnalyzerError::EmptyFile(p)    => write!(f, "File is empty: '{}'", p),
        }
    }
}

impl std::error::Error for AnalyzerError {}

// ── Text Analysis Module ──────────────────────────────────────

mod analyzer {
    use std::collections::{HashMap, HashSet};

    #[derive(Debug)]
    pub struct TextStats {
        pub filename:     String,
        pub total_chars:  usize,
        pub total_lines:  usize,
        pub total_words:  usize,
        pub unique_words: usize,
        pub avg_word_len: f64,
        pub longest_word: String,
        pub shortest_word: String,
    }

    pub fn count_lines(text: &str) -> usize {
        text.lines().count()
    }

    pub fn count_words(text: &str) -> usize {
        text.split_whitespace().count()
    }

    pub fn count_unique_words(text: &str) -> usize {
        let unique: HashSet<String> = text
            .split_whitespace()
            .map(|w| w.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string())
            .filter(|w| !w.is_empty())
            .collect();
        unique.len()
    }

    pub fn word_frequency(text: &str) -> HashMap<String, usize> {
        let mut freq: HashMap<String, usize> = HashMap::new();
        for word in text.split_whitespace() {
            // Clean the word: lowercase, remove punctuation
            let clean: String = word
                .to_lowercase()
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect();
            if !clean.is_empty() {
                *freq.entry(clean).or_insert(0) += 1;
            }
        }
        freq
    }

    pub fn top_words(freq: &HashMap<String, usize>, n: usize) -> Vec<(String, usize)> {
        let mut pairs: Vec<(String, usize)> = freq
            .iter()
            .map(|(k, &v)| (k.clone(), v))
            .collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        pairs.truncate(n);
        pairs
    }

    pub fn avg_word_length(text: &str) -> f64 {
        let words: Vec<&str> = text.split_whitespace().collect();
        if words.is_empty() { return 0.0; }
        let total_len: usize = words.iter().map(|w| w.len()).sum();
        total_len as f64 / words.len() as f64
    }

    pub fn longest_word(text: &str) -> String {
        text.split_whitespace()
            .max_by_key(|w| w.len())
            .unwrap_or("")
            .to_string()
    }

    pub fn shortest_word(text: &str) -> String {
        text.split_whitespace()
            .filter(|w| !w.is_empty())
            .min_by_key(|w| w.len())
            .unwrap_or("")
            .to_string()
    }

    pub fn analyze(filename: &str, text: &str) -> TextStats {
        TextStats {
            filename:      filename.to_string(),
            total_chars:   text.len(),
            total_lines:   count_lines(text),
            total_words:   count_words(text),
            unique_words:  count_unique_words(text),
            avg_word_len:  avg_word_length(text),
            longest_word:  longest_word(text),
            shortest_word: shortest_word(text),
        }
    }
}

// ── File Operations ───────────────────────────────────────────

fn read_file(path: &str) -> Result<String, AnalyzerError> {
    fs::read_to_string(path)
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                AnalyzerError::FileNotFound(path.to_string())
            } else {
                AnalyzerError::ReadError(e.to_string())
            }
        })
}

fn write_file(path: &str, content: &str) -> Result<(), AnalyzerError> {
    fs::write(path, content)
        .map_err(|e| AnalyzerError::WriteError(e.to_string()))
}

fn print_separator() { println!("{}", "═".repeat(50)); }
fn print_thin()       { println!("{}", "─".repeat(50)); }

fn print_stats(stats: &analyzer::TextStats) {
    print_separator();
    println!("  📄 FILE: {}", stats.filename);
    print_thin();
    println!("  Lines:        {}", stats.total_lines);
    println!("  Total Words:  {}", stats.total_words);
    println!("  Unique Words: {}", stats.unique_words);
    println!("  Total Chars:  {}", stats.total_chars);
    println!("  Avg Word Len: {:.2}", stats.avg_word_len);
    println!("  Longest Word: '{}'", stats.longest_word);
    println!("  Shortest Wrd: '{}'", stats.shortest_word);
    print_separator();
}

// ─────────────────────────────────────────────────────────────

fn main() {
    print_separator();
    println!("  📊 RUST TEXT ANALYZER — Day 30 Project");
    print_separator();

    // ── Step 1: Create sample files ──────────────────────────

    let sample1 = "The quick brown fox jumps over the lazy dog.
The dog barked and the fox ran away quickly.
Rust programming is fast, safe, and productive.
Learning Rust every day makes you a better programmer.
The Rust language has ownership, borrowing, and lifetimes.";

    let sample2 = "To be or not to be that is the question.
Whether tis nobler in the mind to suffer.
The slings and arrows of outrageous fortune.
Or to take arms against a sea of troubles.
And by opposing end them to die to sleep.
No more and by a sleep to say we end.
The heartache and the thousand natural shocks.
That flesh is heir to tis a consummation.
Devoutly to be wished to die to sleep.
To sleep perchance to dream ay there is the rub.";

    println!("\n📝 Creating sample files...");
    write_file("sample1.txt", sample1).expect("Failed to write sample1");
    write_file("sample2.txt", sample2).expect("Failed to write sample2");
    println!("  ✓ sample1.txt created");
    println!("  ✓ sample2.txt created");

    // ── Step 2: Analyze each file ─────────────────────────────

    let files = ["sample1.txt", "sample2.txt"];

    for filename in &files {
        println!();
        match read_file(filename) {
            Err(e) => {
                println!("Error: {}", e);
                continue;
            },
            Ok(content) if content.trim().is_empty() => {
                println!("Error: {}", AnalyzerError::EmptyFile(filename.to_string()));
                continue;
            },
            Ok(content) => {
                let stats = analyzer::analyze(filename, &content);
                print_stats(&stats);

                // Word frequency analysis
                let freq = analyzer::word_frequency(&content);
                let top = analyzer::top_words(&freq, 10);

                println!("\n  🔤 TOP {} MOST FREQUENT WORDS:", top.len());
                print_thin();
                for (i, (word, count)) in top.iter().enumerate() {
                    let bar = "█".repeat(*count);
                    println!("  {:2}. {:15} {:3}  {}", i + 1, word, count, bar);
                }
                print_separator();
            }
        }
    }

    // ── Step 3: Compare both files ────────────────────────────
    println!("\n🔍 COMPARATIVE ANALYSIS");
    print_separator();

    let text1 = read_file("sample1.txt").unwrap_or_default();
    let text2 = read_file("sample2.txt").unwrap_or_default();

    let words1: std::collections::HashSet<String> = text1
        .split_whitespace()
        .map(|w| w.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>())
        .filter(|w| !w.is_empty())
        .collect();

    let words2: std::collections::HashSet<String> = text2
        .split_whitespace()
        .map(|w| w.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>())
        .filter(|w| !w.is_empty())
        .collect();

    let common: HashSet<_> = words1.intersection(&words2).collect();
    let only1:  HashSet<_> = words1.difference(&words2).collect();
    let only2:  HashSet<_> = words2.difference(&words1).collect();

    println!("  Words in sample1: {}", words1.len());
    println!("  Words in sample2: {}", words2.len());
    println!("  Common words:     {}", common.len());
    println!("  Only in sample1:  {}", only1.len());
    println!("  Only in sample2:  {}", only2.len());

    let mut common_sorted: Vec<_> = common.into_iter().collect();
    common_sorted.sort();
    println!("\n  Common words: {:?}", &common_sorted[..common_sorted.len().min(10)]);

    // ── Step 4: Write analysis report ────────────────────────
    let stats1 = analyzer::analyze("sample1.txt", &text1);
    let stats2 = analyzer::analyze("sample2.txt", &text2);

    let report = format!(
        "TEXT ANALYSIS REPORT\n\
         ====================\n\
         \n\
         FILE: {}\n\
         Lines: {}, Words: {}, Unique: {}, Chars: {}\n\
         Longest: '{}', Shortest: '{}'\n\
         \n\
         FILE: {}\n\
         Lines: {}, Words: {}, Unique: {}, Chars: {}\n\
         Longest: '{}', Shortest: '{}'\n\
         \n\
         COMPARISON\n\
         Common Words: {}\n\
         Only in file1: {}\n\
         Only in file2: {}\n",
        stats1.filename, stats1.total_lines, stats1.total_words, stats1.unique_words, stats1.total_chars,
        stats1.longest_word, stats1.shortest_word,
        stats2.filename, stats2.total_lines, stats2.total_words, stats2.unique_words, stats2.total_chars,
        stats2.longest_word, stats2.shortest_word,
        common_sorted.len(), only1.len(), only2.len()
    );

    write_file("analysis_report.txt", &report).expect("Failed to write report");
    println!("\n  ✓ Report written to analysis_report.txt");

    // ── Step 5: Error handling demo ───────────────────────────
    println!("\n🛡️  ERROR HANDLING DEMO");
    print_thin();
    match read_file("nonexistent.txt") {
        Ok(_)  => println!("  Found it!"),
        Err(e) => println!("  Handled: {}", e),
    }

    // ── Cleanup ──────────────────────────────────────────────
    println!("\n🧹 Cleanup...");
    for f in ["sample1.txt", "sample2.txt", "analysis_report.txt"] {
        if fs::remove_file(f).is_ok() {
            println!("  Removed {}", f);
        }
    }

    print_separator();
    println!("  Phase 3 COMPLETE! Error Handling & Collections ✅");
    print_separator();

    // ── Extension Challenges ─────────────────────────────────
    // 1. Add a --file <path> command line argument to analyze any file
    //    Use std::env::args() from Day 29

    // 2. Add sentence counting (split by '.', '!', '?')

    // 3. Calculate the Flesch-Kincaid readability score

    // 4. Find all lines containing a specific keyword (grep-like)
    //    fn grep(text: &str, keyword: &str) -> Vec<(usize, &str)>

    // 5. Add support for reading multiple files at once and combining stats
}
