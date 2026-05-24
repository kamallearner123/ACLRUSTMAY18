# 🦀 Module 2 — Ownership & Borrowing (Deep Dive)

> **The most unique and important part of Rust. Master this = master Rust.**

---

## 1. The Three Ownership Rules

```
1. Every value has exactly ONE owner.
2. When the owner goes out of scope, the value is dropped (freed).
3. There can only ever be ONE owner at a time.
```

```rust
fn main() {
    let s1 = String::from("hello");   // s1 owns the heap string
    let s2 = s1;                      // MOVE: s1 is no longer valid!
    // println!("{}", s1);            // ❌ borrow of moved value
    println!("{}", s2);               // ✅
}  // s2 dropped here → memory freed automatically
```

**Analogy:** Think of ownership like handing over a car key. Once you give it away, you no longer have access.

---

## 2. Stack vs Heap — Why it Matters

| Data | Location | Copy or Move? |
|------|----------|---------------|
| `i32`, `f64`, `bool`, `char` | Stack | **Copy** (cheap, fixed size) |
| `String`, `Vec<T>`, `Box<T>` | Heap | **Move** (owns heap memory) |

```rust
fn main() {
    // Stack types implement Copy — no move
    let x = 5;
    let y = x;          // COPY, not move
    println!("{} {}", x, y);  // both valid ✅

    // Heap types move
    let s1 = String::from("world");
    let s2 = s1.clone();   // explicit deep copy
    println!("{} {}", s1, s2);  // both valid ✅
}
```

---

## 3. Borrowing — References (&)

Borrowing = accessing data WITHOUT taking ownership.

```rust
fn print_len(s: &String) {   // borrows, doesn't own
    println!("length: {}", s.len());
}   // s reference dropped, original not affected

fn main() {
    let s = String::from("Rust");
    print_len(&s);           // pass reference
    println!("{}", s);       // s still valid ✅
}
```

### The Two Borrowing Rules
```
1. You can have MANY immutable references (&T) at the same time.
2. OR you can have EXACTLY ONE mutable reference (&mut T).
3. You CANNOT have both simultaneously.
```

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;         // ✅ immutable borrow
    let r2 = &s;         // ✅ second immutable borrow
    println!("{} {}", r1, r2);  // r1, r2 used here — borrows end

    let r3 = &mut s;     // ✅ mutable borrow (r1, r2 are done)
    r3.push_str(" world");
    println!("{}", r3);
}
```

---

## 4. The Borrow Checker — Visual Model

```
Memory:   [  "hello"  ]
           ↑          ↑
           owner: s    &s: r1, r2 (read-only, many ok)

Mutable:  [  "hello"  ]
           ↑
           &mut s: r3  (exclusive write — no other borrows allowed)
```

---

## 5. Dangling References — Prevented at Compile Time

```rust
fn dangle() -> &String {        // ❌ won't compile
    let s = String::from("hi");
    &s                          // s dropped at }, reference dangles
}
```

```
error[E0106]: missing lifetime specifier
```

Fix: Return the `String` itself (transfer ownership):
```rust
fn no_dangle() -> String {
    let s = String::from("hi");
    s   // move ownership to caller
}
```

---

## 6. Slices — Borrowing Part of a Collection

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    // sentence.clear();    // ❌ can't mutate while immutably borrowed
    println!("First word: {}", word);
}
```

---

## 7. Move Semantics in Functions

```rust
fn takes_ownership(s: String) {    // s moves in
    println!("{}", s);
}   // s dropped here

fn makes_copy(x: i32) {           // x is copied in (i32 is Copy)
    println!("{}", x);
}

fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);  // ❌ s was moved

    let x = 5;
    makes_copy(x);
    println!("{}", x);     // ✅ x still valid (copied)
}
```

---

## 8. Tricky Questions

### Q1 — What happens?
```rust
fn main() {
    let s = String::from("hello");
    let t = s;
    println!("{}", s);    // ?
}
```
**Answer:** ❌ Compile error — `s` moved to `t`. Use `s.clone()` or `&s`.

---

### Q2 — Will this compile?
```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];       // immutable borrow
    v.push(4);               // ❌ mutable borrow while immutable exists
    println!("{}", first);
}
```
**Answer:** ❌ No — `v.push()` needs `&mut v`, but `first` holds `&v`.  
Fix: Drop `first` before mutating:
```rust
{
    let first = &v[0];
    println!("{}", first);
}  // first dropped here
v.push(4);
```

---

### Q3 — Tricky Copy Trait
```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = p1;             // Move or Copy?
    println!("{:?}", p1);   // ?
}
```
**Answer:** ❌ Compile error — `Point` does NOT implement `Copy` by default.  
Fix: Add `#[derive(Copy, Clone)]` to `Point`.  
**Key insight:** Custom structs must explicitly opt in to `Copy`.

---

### Q4 — Lifetime Puzzle
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```
**Answer:** ❌ Won't compile — compiler can't know which reference is returned.  
Fix with lifetime annotation:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

### Q5 — Mutable Ref Scope (NLL — Non-Lexical Lifetimes)
```rust
fn main() {
    let mut s = String::from("hello");
    let r = &mut s;
    r.push_str(" world");
    println!("{}", s);    // Does this compile?
}
```
**Answer:** ✅ Yes in Rust 2021 (NLL)! `r`'s mutable borrow ends after `r.push_str(...)` is last used. Then `s` is usable again.

---

### Q6 — Double Free Prevention
```rust
fn main() {
    let s1 = String::from("data");
    let s2 = s1;    // move
    // Both s1 and s2 go out of scope — does Rust free memory twice?
}
```
**Answer:** ❌ No double free! Only `s2` is valid at scope end, so memory is freed exactly once. Rust's move semantics prevent double-free by design.

---

## 9. Key Takeaways

| Concept | Rule |
|---------|------|
| Ownership | One owner; dropped when owner leaves scope |
| Copy types | `i32`, `bool`, `f64`, `char`, tuples of Copy types |
| Move types | `String`, `Vec`, `Box` — ownership transfers |
| Immutable borrow | Many `&T` at once — read-only |
| Mutable borrow | One `&mut T` — exclusive write |
| Slices | Borrowed view into a contiguous sequence |
| NLL (2021) | Borrows end at last use, not end of block |
