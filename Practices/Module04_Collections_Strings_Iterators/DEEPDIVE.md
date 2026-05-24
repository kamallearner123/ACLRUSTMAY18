# 🦀 Module 4 — Collections, Strings & Iterators (Deep Dive)

---

## 1. Strings — Two Types, One Confusion

```
&str  →  string slice — fixed size, usually embedded in binary or borrowed
String →  owned, heap-allocated, growable
```

```rust
fn main() {
    // &str — borrowed string slice
    let s1: &str = "hello";           // stored in binary (read-only)

    // String — heap allocated
    let mut s2: String = String::from("hello");
    s2.push(' ');                     // append char
    s2.push_str("world");             // append &str
    s2 += "!";                        // + operator (takes ownership of left side)

    println!("{}", s2);               // "hello world!"

    // Conversion
    let s3: String = s1.to_string();  // &str → String
    let s4: &str   = &s2;             // String → &str (deref coercion)

    // String formatting (returns new String, no ownership issues)
    let full = format!("{} {}", s1, "rust");
    println!("{}", full);
}
```

### String Indexing — Why `s[0]` Fails

```rust
fn main() {
    let s = String::from("Héllo");   // é is 2 bytes (UTF-8)
    // let c = s[0];                 // ❌ NO direct indexing — Rust prevents this
    // Why? s[0] would be ONE BYTE, not one character

    // Use .chars() for Unicode-correct iteration
    for c in s.chars() {
        print!("{} ", c);            // H é l l o
    }

    // Use .bytes() for raw byte iteration
    for b in s.bytes() {
        print!("{} ", b);
    }

    // Slicing by BYTE position (be careful with multibyte chars!)
    let hello = &s[0..1];            // "H" — 1 byte, safe
    println!("{}", hello);
}
```

---

## 2. Vec<T> — Dynamic Arrays

```rust
fn main() {
    // Creation
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 4, 5];    // macro shorthand

    // Mutation
    v.push(10);
    v.push(20);
    v.push(30);

    // Accessing — two ways
    let third = &v[2];                // panics if out of bounds
    println!("third: {}", third);

    match v.get(10) {                 // safe — returns Option<&T>
        Some(val) => println!("got {}", val),
        None      => println!("index out of range"),
    }

    // Iteration
    for n in &v {                     // immutable borrow
        print!("{} ", n);
    }

    for n in &mut v {                 // mutable borrow
        *n *= 2;                      // dereference to mutate
    }
    println!("{:?}", v);

    // Pop, remove, retain
    v.pop();                          // removes last
    v.retain(|&x| x > 10);           // keep only elements > 10
    println!("{:?}", v);

    // Sorting
    let mut nums = vec![3, 1, 4, 1, 5, 9, 2];
    nums.sort();
    nums.sort_by(|a, b| b.cmp(a));   // reverse sort
    println!("{:?}", nums);
}
```

### Vec of Enums — Storing Different Types

```rust
#[derive(Debug)]
enum Cell {
    Int(i64),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        Cell::Int(42),
        Cell::Float(3.14),
        Cell::Text(String::from("hello")),
    ];

    for cell in &row {
        match cell {
            Cell::Int(n)   => println!("int: {}", n),
            Cell::Float(f) => println!("float: {}", f),
            Cell::Text(s)  => println!("text: {}", s),
        }
    }
}
```

---

## 3. HashMap<K, V>

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();

    // Insert
    scores.insert(String::from("Alice"), 95);
    scores.insert(String::from("Bob"),   87);

    // Entry API — insert only if absent (very idiomatic Rust)
    scores.entry(String::from("Alice")).or_insert(0);   // Alice already there
    scores.entry(String::from("Carol")).or_insert(75);  // Carol inserted

    // Update based on existing value
    let count = scores.entry(String::from("Bob")).or_insert(0);
    *count += 10;    // Bob: 87 → 97

    // Lookup
    if let Some(score) = scores.get("Alice") {
        println!("Alice: {}", score);
    }

    // Iteration (order NOT guaranteed)
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }

    // Word frequency counter — classic example
    let text = "hello world hello rust world hello";
    let mut freq: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", freq);
}
```

---

## 4. Iterators — The Functional Heart of Rust

Iterators are **lazy** — they do nothing until consumed.

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map → filter → collect pipeline
    let result: Vec<i32> = v.iter()
        .map(|x| x * x)          // square each
        .filter(|x| x % 2 == 0)  // keep evens
        .collect();
    println!("{:?}", result);     // [4, 16, 36, 64, 100]

    // fold — like reduce
    let sum = v.iter().fold(0, |acc, x| acc + x);
    println!("sum: {}", sum);      // 55

    // any, all
    println!("{}", v.iter().any(|&x| x > 9));  // true
    println!("{}", v.iter().all(|&x| x > 0));  // true

    // zip — pair two iterators
    let names = vec!["Alice", "Bob", "Carol"];
    let scores = vec![95, 87, 92];
    let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("{:?}", pairs);

    // enumerate — index + value
    for (i, name) in names.iter().enumerate() {
        println!("{}: {}", i, name);
    }

    // chain — combine iterators
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let chained: Vec<_> = a.iter().chain(b.iter()).collect();
    println!("{:?}", chained);

    // flat_map — flatten nested
    let words = vec!["hello world", "foo bar"];
    let chars: Vec<&str> = words.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();
    println!("{:?}", chars);

    // take, skip
    let first_three: Vec<_> = v.iter().skip(2).take(3).collect();
    println!("{:?}", first_three);  // [3, 4, 5]
}
```

### Custom Iterator

```rust
struct Counter {
    count: u32,
    max:   u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // All iterator adapters work on custom iterators!
    let sum: u32 = Counter::new(5)
        .zip(Counter::new(5).skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum: {}", sum);   // 6
}
```

---

## 5. Tricky Questions

### Q1 — String + Operator
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + &s2;     // s1 is MOVED here
    println!("{}", s3);
    println!("{}", s1);    // ❌ or ✅?
}
```
**Answer:** ❌ `s1` is moved by the `+` operator. The signature is `fn add(self, s: &str) -> String` — takes ownership of `self`. Use `format!("{}{}", s1, s2)` to avoid moving.

---

### Q2 — Iterator Laziness
```rust
fn main() {
    let v = vec![1, 2, 3];
    let _iter = v.iter().map(|x| {
        println!("processing {}", x);
        x * 2
    });
    // Did "processing 1/2/3" print?
}
```
**Answer:** ❌ Nothing prints! Iterators are lazy — `map` doesn't execute until consumed. Add `.collect::<Vec<_>>()` or `.for_each(|_| {})` to drive it.

---

### Q3 — HashMap Ownership
```rust
use std::collections::HashMap;

fn main() {
    let key = String::from("name");
    let val = String::from("Alice");
    let mut map = HashMap::new();
    map.insert(key, val);
    println!("{}", key);   // ❌ or ✅?
}
```
**Answer:** ❌ `key` and `val` are moved into the HashMap. Use `key.clone()` or insert `&str` references.

---

### Q4 — Vec Borrow During Iteration
```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    for n in &v {
        if *n == 3 { v.push(6); }   // ❌?
    }
}
```
**Answer:** ❌ Compile error — can't mutate `v` while it's borrowed by the for loop. Collect indices first, then modify.

---

### Q5 — Collect Into Different Types
```rust
fn main() {
    let s: String = "hello".chars()
        .filter(|c| *c != 'l')
        .collect();     // chars → String!
    println!("{}", s);  // "heo"
}
```
**Answer:** ✅ `collect()` can collect `char` iterators into a `String`. The return type annotation drives what `collect` produces.

---

### Q6 — .iter() vs .into_iter() vs .iter_mut()
```rust
let v = vec![1, 2, 3];
v.iter()       // yields &i32    — borrows v
v.iter_mut()   // yields &mut i32 — mutably borrows v  
v.into_iter()  // yields i32     — CONSUMES v (v no longer usable)
```
**Question:** After `for x in v { ... }` — can you use `v` again?  
**Answer:** ❌ No — `for x in v` calls `v.into_iter()`, consuming `v`. Use `for x in &v` to borrow.

---

## 6. Key Takeaways

| Collection | Use When |
|------------|----------|
| `Vec<T>` | Ordered, growable list |
| `HashMap<K,V>` | Key-value lookup, O(1) average |
| `BTreeMap<K,V>` | Sorted key-value (use when order matters) |
| `HashSet<T>` | Unique values, fast membership check |
| `String` | Owned UTF-8 text |
| `&str` | Borrowed string slice, function params |

**Iterator Rule:** Build pipelines with `map/filter/flat_map`, consume with `collect/sum/for_each/fold`.
