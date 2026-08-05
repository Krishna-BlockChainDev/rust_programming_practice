# 🦀 60-Day Rust Programming Curriculum

> **Goal:** Learn Rust from absolute basics to intermediate level in 60 days  
> **Daily Time:** 30–45 minutes  
> **Start Level:** Complete beginner

---

## 📅 Phase 1: Foundations (Days 1–10)

> Learn Rust syntax, basic data types, and how to write simple programs.

| Day    | Topic                      | Key Concepts                                    |
| ------ | -------------------------- | ----------------------------------------------- |
| Day-1  | Hello World & Cargo Basics | `println!`, `cargo new`, `cargo run`, comments  |
| Day-2  | Variables & Mutability     | `let`, `mut`, `const`, shadowing                |
| Day-3  | Data Types                 | integers, floats, booleans, characters          |
| Day-4  | String Types               | `String` vs `&str`, basic string operations     |
| Day-5  | Operators & Expressions    | arithmetic, comparison, logical operators       |
| Day-6  | Control Flow – if/else     | `if`, `else if`, `else`, `if` as expression     |
| Day-7  | Loops                      | `loop`, `while`, `for`, `break`, `continue`     |
| Day-8  | Functions                  | defining functions, parameters, return values   |
| Day-9  | Tuples & Arrays            | fixed-size collections, indexing, destructuring |
| Day-10 | Mini Project: Calculator   | Apply Days 1–9 concepts                         |

---

## 📅 Phase 2: Core Concepts (Days 11–20)

> Dive into Rust's unique ownership model and compound types.

| Day    | Topic                         | Key Concepts                                       |
| ------ | ----------------------------- | -------------------------------------------------- |
| Day-11 | Ownership Basics              | ownership rules, move semantics                    |
| Day-12 | Borrowing & References        | `&`, `&mut`, immutable vs mutable references       |
| Day-13 | Slices                        | string slices `&str`, array slices                 |
| Day-14 | Structs                       | defining structs, creating instances, field access |
| Day-15 | Struct Methods & impl         | `impl` block, methods, associated functions        |
| Day-16 | Enums                         | defining enums, variants, `match` with enums       |
| Day-17 | Pattern Matching              | `match`, `if let`, `while let`, wildcards          |
| Day-18 | Option<T>                     | `Some`, `None`, handling optional values           |
| Day-19 | Vectors                       | `Vec<T>`, push, pop, iteration                     |
| Day-20 | Mini Project: Student Records | Structs + Vectors + Enums                          |

---

## 📅 Phase 3: Error Handling & Modules (Days 21–30)

> Handle errors gracefully and organize code into modules.

| Day    | Topic                           | Key Concepts                         |
| ------ | ------------------------------- | ------------------------------------ |
| Day-21 | Error Handling – Result<T,E>    | `Ok`, `Err`, `unwrap`, `expect`      |
| Day-22 | The `?` Operator                | propagating errors, chaining         |
| Day-23 | Custom Error Types              | defining error enums, `From` trait   |
| Day-24 | HashMaps                        | `HashMap<K,V>`, insert, get, iterate |
| Day-25 | HashSets & BTreeMap             | sets, sorted maps                    |
| Day-26 | Modules & `mod`                 | organizing code, `pub`, `use`        |
| Day-27 | Crates & Cargo.toml             | external crates, `rand`, `chrono`    |
| Day-28 | File I/O Basics                 | reading/writing files, `std::fs`     |
| Day-29 | Command Line Args               | `std::env::args`, parsing input      |
| Day-30 | Mini Project: File Word Counter | File I/O + HashMaps + Error Handling |

---

## 📅 Phase 4: Generics, Traits & Lifetimes (Days 31–40)

> Master Rust's powerful type system.

| Day    | Topic                            | Key Concepts                                 |
| ------ | -------------------------------- | -------------------------------------------- | --- | ------------------------------- |
| Day-31 | Generics Basics                  | generic functions, generic structs           |
| Day-32 | Traits                           | defining traits, implementing traits         |
| Day-33 | Default Trait Implementations    | `#[derive]`, `Display`, `Debug`              |
| Day-34 | Trait Bounds                     | `where` clause, multiple bounds              |
| Day-35 | Lifetimes Introduction           | lifetime annotations, `'a`                   |
| Day-36 | Lifetimes in Structs             | struct lifetime annotations                  |
| Day-37 | Closures                         | `                                            |     | ` syntax, capturing environment |
| Day-38 | Iterators                        | `Iterator` trait, `map`, `filter`, `collect` |
| Day-39 | Iterator Chaining                | complex iterator chains, `enumerate`, `zip`  |
| Day-40 | Mini Project: Generic Data Store | Generics + Traits + Iterators                |

---

## 📅 Phase 5: Smart Pointers & Concurrency (Days 41–50)

> Understand memory management and multi-threading.

| Day    | Topic                                | Key Concepts                         |
| ------ | ------------------------------------ | ------------------------------------ |
| Day-41 | Box<T>                               | heap allocation, recursive types     |
| Day-42 | Rc<T> & Arc<T>                       | reference counting, shared ownership |
| Day-43 | RefCell<T> & Interior Mutability     | runtime borrow checking              |
| Day-44 | Threads Basics                       | `std::thread::spawn`, `join`         |
| Day-45 | Message Passing                      | channels, `mpsc::channel`            |
| Day-46 | Mutex<T> & Shared State              | `Mutex`, `Arc<Mutex<T>>`             |
| Day-47 | Fearless Concurrency Review          | combining Arc + Mutex + Threads      |
| Day-48 | Async/Await Introduction             | `async fn`, `await`, `tokio` basics  |
| Day-49 | Working with `tokio`                 | async tasks, runtime                 |
| Day-50 | Mini Project: Multi-threaded Counter | Threads + Arc + Mutex                |

---

## 📅 Phase 6: Advanced Topics & Real Projects (Days 51–60)

> Apply everything learned in real-world mini-projects.

| Day    | Topic                           | Key Concepts                          |
| ------ | ------------------------------- | ------------------------------------- |
| Day-51 | Testing in Rust                 | `#[test]`, `assert!`, `cargo test`    |
| Day-52 | Documentation & Doc Tests       | `///`, `cargo doc`, doc test examples |
| Day-53 | Macros Basics                   | `macro_rules!`, declarative macros    |
| Day-54 | Enums Advanced – State Machines | using enums for state modeling        |
| Day-55 | Newtype Pattern & Type Aliases  | `type`, newtype idiom                 |
| Day-56 | Working with JSON               | `serde`, `serde_json` crate           |
| Day-57 | Building a CLI Tool             | `clap` crate, argument parsing        |
| Day-58 | Mini Project: Todo CLI App      | Full CLI CRUD application             |
| Day-59 | Code Review & Refactoring       | clippy, fmt, best practices           |
| Day-60 | Final Project: Contact Book     | All concepts combined                 |

---

## 🗺️ Learning Path Visualization

```
BASICS          OWNERSHIP       ERROR HANDLING    TYPE SYSTEM      CONCURRENCY     PROJECTS
Days 1-10  →   Days 11-13  →  Days 21-23    →  Days 31-36   →  Days 41-47   →  Days 51-60
               STRUCTS/ENUMS   COLLECTIONS      CLOSURES/ITER    ASYNC
               Days 14-18   →  Days 24-29    →  Days 37-39   →  Days 48-49
```

---

## 📌 Tips for Success

1. **Be consistent** — 30 mins every day beats 5 hours on weekends
2. **Type, don't copy** — muscle memory is real
3. **Read error messages** — Rust's compiler is your best teacher
4. **Use `cargo check`** — faster than `cargo build` during development
5. **Experiment** — break things intentionally to learn
6. **Revisit** — look back at earlier days when concepts seem fuzzy
7. **Take notes** — add comments in your code files

---

## 🏁 Milestones

- [ ] **Day 10** — First mini project complete ✅
- [ ] **Day 20** — Ownership & Structs mastered ✅
- [ ] **Day 30** — Error handling & File I/O done ✅
- [ ] **Day 40** — Generics & Traits understood ✅
- [ ] **Day 50** — Concurrency basics complete ✅
- [ ] **Day 60** — Real projects built! 🎉

---

_Run `cargo run` inside any Day folder to execute that day's exercises._  
_See `SETUP_GUIDE.md` for installation instructions._
