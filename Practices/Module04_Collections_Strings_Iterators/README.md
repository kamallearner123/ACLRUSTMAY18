# Module 4 — Collections, Strings & Iterators (2 Hours)

## Objectives
- Work with Vec, HashMap, BTreeMap, HashSet
- Understand UTF-8 and String internals
- Master functional iterator patterns

## Practice Exercises

### 📁 `log_parser/` — Log Line Analyzer
Parse log lines: filter ERRORs, count by level, sort by timestamp

### 📁 `csv_analyzer/` — CSV Data Processor
Use iterators to compute statistics from CSV data

### 📁 `sensor_processor/` — Sensor Data Aggregator
Process streams of sensor readings using map/filter/fold

## Key Concepts
```rust
use std::collections::HashMap;

// Vec operations
let mut scores: Vec<i32> = vec![85, 92, 78, 95, 88];
scores.push(100);
let passing: Vec<i32> = scores.iter().filter(|&&s| s >= 80).cloned().collect();

// HashMap
let mut word_count: HashMap<String, usize> = HashMap::new();
for word in text.split_whitespace() {
    *word_count.entry(word.to_string()).or_insert(0) += 1;
}

// Iterator chain (zero allocation)
let result: i32 = (1..=100)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .take(5)
    .sum();

// String vs &str
let owned: String = String::from("hello");  // heap allocated
let borrowed: &str = "world";               // string literal
let combined = format!("{} {}", owned, borrowed);

// UTF-8 iteration
let emoji = "Hello 🦀";
for c in emoji.chars() {
    print!("{} ", c);
}
```

## Python vs Rust Iterators
```python
# Python list comprehension
result = [x*x for x in range(1, 101) if x % 2 == 0]
```
```rust
// Rust iterator — lazy, zero-cost
let result: Vec<i32> = (1..=100)
    .filter(|x| x % 2 == 0)
    .map(|x| x * x)
    .collect();
```

## ✅ Copy your practice programs here:
`Practices/Module04_Collections_Strings_Iterators/`
