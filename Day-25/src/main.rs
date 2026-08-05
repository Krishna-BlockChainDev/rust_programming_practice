// ============================================================
// DAY 25: HashSet & BTreeMap
// Topic: Sets and sorted maps in Rust
// Time: 30-45 minutes
// ============================================================
//
// WHAT YOU WILL LEARN TODAY:
//   1. HashSet<T> — unique collection of values (no duplicates)
//   2. Set operations: union, intersection, difference
//   3. BTreeMap<K,V> — sorted HashMap (keys always sorted)
//   4. BTreeSet<T> — sorted HashSet
//   5. When to use which collection
//
// HOW TO RUN:
//   $ cargo run   (inside the Day-25 folder)
//
// ============================================================

use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet};

fn main() {
    // --------------------------------------------------------
    // EXERCISE 1: HashSet — No Duplicates!
    // --------------------------------------------------------
    println!("=== EXERCISE 1: HashSet Basics ===");

    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(2);  // duplicate — silently ignored
    set.insert(1);  // duplicate — silently ignored
    println!("Set: {:?}", set);    // order is not guaranteed!
    println!("Length: {}", set.len());  // 3, not 5

    // Check membership
    println!("Contains 2: {}", set.contains(&2));
    println!("Contains 5: {}", set.contains(&5));

    // Remove element
    set.remove(&2);
    println!("After remove(2): {:?}", set);

    // --------------------------------------------------------
    // EXERCISE 2: HashSet from a Vec (deduplicate!)
    // --------------------------------------------------------
    println!("\n=== EXERCISE 2: Deduplication ===");

    let numbers_with_dupes = vec![1, 2, 3, 2, 4, 3, 5, 1, 6, 5];
    println!("Original: {:?}", numbers_with_dupes);

    let unique: HashSet<i32> = numbers_with_dupes.into_iter().collect();
    println!("Unique:   {:?}", unique);

    // Back to sorted Vec
    let mut unique_sorted: Vec<i32> = unique.into_iter().collect();
    unique_sorted.sort();
    println!("Sorted:   {:?}", unique_sorted);

    // --------------------------------------------------------
    // EXERCISE 3: Set Operations
    // --------------------------------------------------------
    println!("\n=== EXERCISE 3: Set Operations ===");

    let set_a: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    let set_b: HashSet<i32> = [4, 5, 6, 7, 8].iter().cloned().collect();

    println!("Set A: {:?}", {let mut v: Vec<_> = set_a.iter().collect(); v.sort(); v});
    println!("Set B: {:?}", {let mut v: Vec<_> = set_b.iter().collect(); v.sort(); v});

    // Union: all elements from both sets (no duplicates)
    let union: HashSet<_> = set_a.union(&set_b).collect();
    let mut union_vec: Vec<_> = union.into_iter().collect();
    union_vec.sort();
    println!("Union (A | B): {:?}", union_vec);

    // Intersection: elements in BOTH sets
    let intersection: HashSet<_> = set_a.intersection(&set_b).collect();
    let mut inter_vec: Vec<_> = intersection.into_iter().collect();
    inter_vec.sort();
    println!("Intersection (A & B): {:?}", inter_vec);

    // Difference: elements in A but NOT in B
    let difference: HashSet<_> = set_a.difference(&set_b).collect();
    let mut diff_vec: Vec<_> = difference.into_iter().collect();
    diff_vec.sort();
    println!("Difference (A - B): {:?}", diff_vec);

    // Symmetric difference: in A or B but not BOTH
    let sym_diff: HashSet<_> = set_a.symmetric_difference(&set_b).collect();
    let mut sym_vec: Vec<_> = sym_diff.into_iter().collect();
    sym_vec.sort();
    println!("Symmetric Diff: {:?}", sym_vec);

    // Subset and superset
    let sub: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
    println!("{{1,2,3}} is subset of A: {}", sub.is_subset(&set_a));
    println!("A is superset of {{1,2,3}}: {}", set_a.is_superset(&sub));

    // --------------------------------------------------------
    // EXERCISE 4: BTreeMap — Always Sorted by Key
    // --------------------------------------------------------
    println!("\n=== EXERCISE 4: BTreeMap (sorted) ===");

    let mut btree: BTreeMap<&str, i32> = BTreeMap::new();
    btree.insert("Zebra",   100);
    btree.insert("Apple",   50);
    btree.insert("Mango",   75);
    btree.insert("Banana",  30);
    btree.insert("Cherry",  90);

    // BTreeMap always iterates in sorted KEY order!
    println!("BTreeMap (always sorted by key):");
    for (k, v) in &btree {
        println!("  {}: {}", k, v);
    }

    // Range queries — unique to BTreeMap!
    println!("\nEntries from 'B' to 'M' (inclusive):");
    for (k, v) in btree.range("B"..="M") {
        println!("  {}: {}", k, v);
    }

    // --------------------------------------------------------
    // EXERCISE 5: BTreeMap for sorted statistics
    // --------------------------------------------------------
    println!("\n=== EXERCISE 5: Grade Distribution (BTreeMap) ===");

    let grades = vec!['B', 'A', 'C', 'A', 'B', 'D', 'A', 'B', 'C', 'F', 'A'];
    let mut grade_count: BTreeMap<char, u32> = BTreeMap::new();

    for grade in &grades {
        *grade_count.entry(*grade).or_insert(0) += 1;
    }

    println!("Grade Distribution (sorted):");
    for (grade, count) in &grade_count {
        println!("  Grade {}: {} students", grade, count);
    }

    // --------------------------------------------------------
    // EXERCISE 6: BTreeSet — Sorted Set
    // --------------------------------------------------------
    println!("\n=== EXERCISE 6: BTreeSet ===");

    let mut bset: BTreeSet<i32> = BTreeSet::new();
    for n in [5, 3, 8, 1, 9, 2, 7, 4, 6] {
        bset.insert(n);
    }
    println!("BTreeSet (always sorted): {:?}", bset);

    // Range in BTreeSet
    println!("Elements 3..=7: {:?}", bset.range(3..=7).collect::<Vec<_>>());

    // --------------------------------------------------------
    // EXERCISE 7: Real-world — Finding common friends
    // --------------------------------------------------------
    println!("\n=== EXERCISE 7: Common Friends ===");

    let alice_friends: HashSet<&str> = ["Bob", "Carol", "David", "Eve"].iter().cloned().collect();
    let bob_friends:   HashSet<&str> = ["Alice", "Carol", "Frank", "Eve"].iter().cloned().collect();

    let common: HashSet<_> = alice_friends.intersection(&bob_friends).collect();
    let mut common_sorted: Vec<_> = common.into_iter().collect();
    common_sorted.sort();
    println!("Alice & Bob common friends: {:?}", common_sorted);

    let all_friends: HashSet<_> = alice_friends.union(&bob_friends).collect();
    println!("All friends combined: {}", all_friends.len());

    // --------------------------------------------------------
    // EXERCISE 8: When to use which?
    // --------------------------------------------------------
    println!("\n=== EXERCISE 8: Collection Comparison ===");
    println!("HashMap  — key-value, fast lookup, unordered");
    println!("BTreeMap — key-value, fast lookup, SORTED by key");
    println!("HashSet  — unique values, fast membership, unordered");
    println!("BTreeSet — unique values, fast membership, SORTED");
    println!("Vec      — ordered list, fast index access, allows duplicates");

    // --------------------------------------------------------
    // YOUR CHALLENGES FOR TODAY:
    // --------------------------------------------------------

    // Challenge 1: Given a sentence, find all UNIQUE words using HashSet
    // let sentence = "the quick brown fox jumps over the lazy dog the";
    // Count unique words and print them sorted

    // Challenge 2: Use BTreeMap to create a sorted phone directory
    // Insert names in random order, iterate to show them alphabetically

    // Challenge 3: Given two lists of enrolled students in Course A and Course B,
    // find students in both courses, in A only, and in B only using set operations

    // Challenge 4: Use BTreeMap to group a list of words by their first letter
    // Then print: "A: [Apple, Avocado]", "B: [Banana]", etc.

    // --------------------------------------------------------
    // SUMMARY:
    //   use std::collections::{HashSet, BTreeMap, BTreeSet};
    //   HashSet::new()           -> unique, unordered set
    //   BTreeMap::new()          -> sorted key-value map
    //   BTreeSet::new()          -> sorted unique set
    //   set.insert(v)            -> add value
    //   set.contains(&v)         -> membership check
    //   set.remove(&v)           -> remove value
    //   a.union(&b)              -> all unique from both
    //   a.intersection(&b)       -> only in both
    //   a.difference(&b)         -> in a but not b
    //   a.is_subset(&b)          -> is a ⊆ b?
    //   btree.range(k1..=k2)    -> range query (BTreeMap only)
    //   BTreeMap iterates in sorted key order (HashMap does not)
    // --------------------------------------------------------
}
