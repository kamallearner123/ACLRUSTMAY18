# Module 2 — Ownership & Borrowing Deep Dive (4 Hours)

## Objectives
- Master Rust ownership model
- Understand move semantics, Copy, Clone
- Debug borrow checker errors
- Understand RAII and the Drop trait

## Practice Exercises

### 📁 `ownership_demo/` — Core ownership traces
Covers: move semantics, Copy vs Clone, borrow checker exercises

### 📁 `inventory_system/` — Assignment: Inventory Management
Build a system that:
- Manages a list of products (name, quantity, price)
- Uses ownership to pass products between functions
- Demonstrates borrow to print without consuming

### 📁 `file_parser/` — Assignment: File Parser
Parse a text file using efficient ownership:
- Read lines using `BufReader`
- Use `&str` slices where possible

## Key Concepts
```rust
// Move semantics
let s1 = String::from("hello");
let s2 = s1;            // s1 moved, no longer valid
// println!("{}", s1);  // ERROR: value used after move

// Immutable borrow
let s = String::from("world");
let len = calculate_length(&s);  // s still valid

// Mutable borrow
let mut s = String::from("hello");
change(&mut s);

// Slices
let s = String::from("hello world");
let word: &str = &s[0..5];
```

## C++ vs Rust Comparison
```cpp
// C++ — UNSAFE: dangling pointer
int* create_val() {
    int x = 5;
    return &x;  // UB: returning pointer to local variable!
}
```
```rust
// Rust — SAFE: borrow checker prevents this at compile time
fn create_val() -> &i32 {  // ERROR: missing lifetime specifier
    let x = 5;
    &x  // compiler rejects this
}
```

## ✅ Copy your practice programs here:
`Practices/Module02_Ownership_Borrowing/`
