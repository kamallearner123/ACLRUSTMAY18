# Module 6 — Traits, Generics & Lifetimes (3 Hours)

## Objectives
- Design reusable code with traits and generics
- Understand monomorphization vs dynamic dispatch
- Annotate lifetimes correctly

## Practice Exercises

### 📁 `plugin_architecture/` — Extensible Plugin System
Define a `Plugin` trait; register multiple plugin types dynamically

### 📁 `generic_parser/` — Generic Parser Framework
Parse different data formats using generic types and trait bounds

### 📁 `serializer_abstraction/` — Serializer Abstraction Layer
Abstract over JSON/TOML serialization via traits

## Key Concepts
```rust
// Trait definition
trait Summarize {
    fn summary(&self) -> String;
    fn author(&self) -> String { String::from("Anonymous") } // default
}

// Generic function with trait bound
fn print_summary<T: Summarize>(item: &T) {
    println!("{} — {}", item.author(), item.summary());
}

// Static dispatch (zero-cost, monomorphized)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

// Dynamic dispatch (runtime vtable)
fn notify(items: &[Box<dyn Summarize>]) {
    for item in items {
        println!("{}", item.summary()); // dispatched at runtime
    }
}

// Lifetimes — explicit annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Associated types
trait Parser {
    type Output;
    fn parse(&self, input: &str) -> Result<Self::Output, String>;
}
```

## Java Interfaces vs Rust Traits
```java
// Java: interfaces allow null implementors, checked exceptions
interface Printable { void print(); }
```
```rust
// Rust: traits are zero-cost, no null, coherence enforced
trait Printable { fn print(&self); }
// Orphan rule: can only impl for your type OR your trait
```

## ✅ Copy your practice programs here:
`Practices/Module06_Traits_Generics_Lifetimes/`
