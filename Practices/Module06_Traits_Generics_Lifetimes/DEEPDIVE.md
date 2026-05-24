# 🦀 Module 6 — Traits, Generics & Lifetimes (Deep Dive)

> **This module is where Rust's type system truly shines.**

---

## 1. Traits — Rust's Answer to Interfaces

A trait defines shared behaviour. Unlike Java interfaces, traits support **default implementations** and can be implemented for any type (including external types via the **orphan rule**).

```rust
trait Describable {
    fn describe(&self) -> String;

    // Default implementation
    fn print_description(&self) {
        println!("{}", self.describe());
    }
}

struct Dog { name: String, breed: String }
struct Cat { name: String, indoor: bool }

impl Describable for Dog {
    fn describe(&self) -> String {
        format!("{} is a {}", self.name, self.breed)
    }
}

impl Describable for Cat {
    fn describe(&self) -> String {
        let location = if self.indoor { "indoor" } else { "outdoor" };
        format!("{} is an {} cat", self.name, location)
    }
    // print_description() inherited from default!
}

fn main() {
    let d = Dog { name: "Rex".into(), breed: "German Shepherd".into() };
    let c = Cat { name: "Whiskers".into(), indoor: true };
    d.print_description();
    c.print_description();
}
```

### Operator Overloading via Traits

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Vector2D { x: f64, y: f64 }

impl Add for Vector2D {
    type Output = Vector2D;
    fn add(self, rhs: Self) -> Self {
        Vector2D { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl std::fmt::Display for Vector2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };
    println!("{} + {} = {}", v1, v2, v1 + v2);
}
```

---

## 2. Generics — Code Reuse With Type Safety

```rust
// Generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max { max = item; }
    }
    max
}

// Generic struct
#[derive(Debug)]
struct Pair<T> {
    first:  T,
    second: T,
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }

    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("Larger: {}", self.first);
        } else {
            println!("Larger: {}", self.second);
        }
    }
}

fn main() {
    let nums = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&nums));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest char: {}", largest(&chars));

    let p = Pair::new(5, 10);
    p.cmp_display();
}
```

### Trait Bounds — Multiple Ways to Write Them

```rust
// Inline bound
fn notify<T: std::fmt::Display>(item: T) { println!("{}", item); }

// Where clause (cleaner for complex bounds)
fn process<T, U>(t: T, u: U)
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug,
{
    println!("{:?}", t.clone());
    println!("{:?}", u);
}

// impl Trait — anonymous generic (return position)
fn make_greeter(name: String) -> impl Fn() {
    move || println!("Hello, {}!", name)
}

// dyn Trait — dynamic dispatch (trait object)
fn print_all(items: &[Box<dyn std::fmt::Display>]) {
    for item in items { println!("{}", item); }
}
```

---

## 3. Lifetimes — Explicit Borrow Duration

Lifetimes tell the compiler *how long references are valid*.

```rust
// The compiler needs to know: does output live as long as x, y, or both?
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
//  'a means: output lives at least as long as the SHORTER of x and y

fn main() {
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("xyz");
        result = longest(s1.as_str(), s2.as_str());
        println!("{}", result);  // ✅ result used within s2's scope
    }
    // println!("{}", result);  // ❌ s2 is dropped, result might point to it
}
```

### Lifetime in Structs

```rust
// This struct holds a reference — must annotate lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part   // lifetime elision: output = self's lifetime
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("no sentence");

    let excerpt = ImportantExcerpt { part: first_sentence };
    println!("{}", excerpt.part);
}
```

### Lifetime Elision Rules

The compiler can infer lifetimes in common patterns (you don't always write `'a`):

```rust
// Rule 1: each input reference gets its own lifetime
fn foo(x: &str) -> &str { x }
// Compiler sees: fn foo<'a>(x: &'a str) -> &'a str

// Rule 2: if exactly one input lifetime, output gets it
fn first(x: &str, _y: &str) -> &str { x }  // requires explicit annotation — not elided!
// ^ Actually ❌ — with multiple inputs you MUST annotate

// Rule 3: if &self is a parameter, output gets self's lifetime
impl MyStruct {
    fn get(&self) -> &str { &self.data }  // elided: output = 'self lifetime
}
```

### Static Lifetime

```rust
// 'static = lives for the entire program duration
let s: &'static str = "I live forever";  // string literals are 'static

// Common in trait objects
fn make_error() -> Box<dyn std::error::Error + 'static> {
    Box::new(std::io::Error::new(std::io::ErrorKind::Other, "oops"))
}
```

---

## 4. Advanced Trait Patterns

### Blanket Implementations

```rust
trait Summary {
    fn summarize(&self) -> String;
}

// Implement Display for ALL types that implement Summary
impl<T: Summary> std::fmt::Display for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summarize())
    }
}
```

### Associated Types vs Generics

```rust
// Associated type — one implementation per type (Iterator uses this)
trait Container {
    type Item;
    fn get(&self, index: usize) -> Option<&Self::Item>;
}

// Generic — multiple implementations possible per type
trait Converter<T> {
    fn convert(&self) -> T;
}
```

---

## 5. Tricky Questions

### Q1 — Lifetime Scope Trap
```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x;      // ❌?
    }
    println!("{}", r);
}
```
**Answer:** ❌ `x` dropped at `}`, but `r` outlives it. The borrow checker catches this.

---

### Q2 — Trait Object vs Generics
```rust
fn print_generic<T: std::fmt::Display>(val: T) { println!("{}", val); }
fn print_dynamic(val: &dyn std::fmt::Display) { println!("{}", val); }
```
**Question:** Which is faster and why?  
**Answer:** `print_generic` — uses **static dispatch** (monomorphized at compile time, inlineable). `print_dynamic` uses **vtable lookup** at runtime (virtual dispatch, like Java interfaces).

---

### Q3 — Where Does This Fail?
```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```
**Answer:** ❌ `dyn Fn` has unknown size. Fix: `Box<dyn Fn(i32) -> i32>` or `impl Fn(i32) -> i32`.

---

### Q4 — Trait Coherence (Orphan Rule)
```rust
use std::fmt;
impl fmt::Display for Vec<i32> { ... }  // ❌?
```
**Answer:** ❌ Orphan rule — you can't implement a foreign trait for a foreign type. Both `Display` (std) and `Vec` (std) are external. Workaround: use a **newtype** `struct MyVec(Vec<i32>)`.

---

### Q5 — Lifetime Elision Puzzle
```rust
fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}
```
**Question:** What lifetimes does the compiler infer?  
**Answer:** `fn first_word<'a>(s: &'a str) -> &'a str` — output borrows from `s` (one input, so output gets its lifetime). Correct and safe.

---

### Q6 — Generic Struct With Mixed Bounds
```rust
use std::fmt::{Debug, Display};

fn compare_and_print<T>(t: T, u: T)
where T: Display + Debug + PartialOrd
{
    if t > u {
        println!("{:?} is bigger", t);
    } else {
        println!("{} is not bigger", t);
    }
}
```
**Answer:** ✅ Compiles. `T` must be the same type for both parameters and satisfy all three bounds.

---

## 6. Key Takeaways

| Concept | Rule |
|---------|------|
| Traits | Define shared behaviour; default impls allowed |
| Generics | Compile-time monomorphization = zero-cost |
| `impl Trait` | Anonymous generic — good for return types |
| `dyn Trait` | Runtime dispatch — heap allocation, vtable |
| Lifetimes | Annotations help compiler verify borrow safety |
| Orphan rule | Can't implement foreign trait for foreign type |
| Associated types | Single impl per type (like Iterator::Item) |
