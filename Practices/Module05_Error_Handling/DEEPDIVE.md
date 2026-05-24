# 🦀 Module 5 — Error Handling & Result-Oriented Design (Deep Dive)

---

## 1. Two Categories of Errors in Rust

| Category | Type | Mechanism |
|----------|------|-----------|
| **Recoverable** | File not found, network timeout, parse failure | `Result<T, E>` |
| **Unrecoverable** | Array out of bounds, assertion failure, logic bug | `panic!()` |

Rust has **no exceptions** — errors are values you handle in the type system.

---

## 2. Result<T, E> — The Workhorse

```rust
use std::fs;
use std::io;

fn read_config(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)   // returns Result<String, io::Error>
}

fn main() {
    // Explicit match
    match read_config("config.toml") {
        Ok(content) => println!("Config loaded: {} bytes", content.len()),
        Err(e)      => println!("Error: {}", e),
    }

    // unwrap — panics if Err (use ONLY in tests/prototypes)
    let _data = read_config("exists.toml").unwrap();

    // expect — panic with custom message
    let _data = read_config("exists.toml").expect("Failed to read config");

    // unwrap_or — default on error
    let content = read_config("missing.toml").unwrap_or_default();

    // unwrap_or_else — compute default lazily
    let content = read_config("missing.toml")
        .unwrap_or_else(|e| format!("default # error: {}", e));
}
```

---

## 3. The `?` Operator — Error Propagation

The `?` operator is Rust's answer to exception propagation — it short-circuits on `Err`.

```rust
use std::fs;
use std::io;
use std::num::ParseIntError;

// Without ? — verbose
fn parse_number_verbose(s: &str) -> Result<i32, ParseIntError> {
    let trimmed = s.trim();
    match trimmed.parse::<i32>() {
        Ok(n)  => Ok(n * 2),
        Err(e) => Err(e),
    }
}

// With ? — concise
fn parse_number(s: &str) -> Result<i32, ParseIntError> {
    let n: i32 = s.trim().parse()?;  // ? returns Err early if parse fails
    Ok(n * 2)
}

// Chaining ? across multiple error types
fn read_and_parse(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;   // io::Error
    let number: i32 = content.trim().parse()?; // ParseIntError
    Ok(number)
}

fn main() {
    match parse_number("  42  ") {
        Ok(v)  => println!("Got: {}", v),   // 84
        Err(e) => println!("Error: {}", e),
    }
}
```

**Key:** `?` can only be used in functions returning `Result` or `Option`.

---

## 4. Custom Error Types

### Basic Custom Error

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    IoError(std::io::Error),
    ParseError(String),
    NotFound { resource: String },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::IoError(e)           => write!(f, "I/O error: {}", e),
            AppError::ParseError(msg)      => write!(f, "Parse error: {}", msg),
            AppError::NotFound { resource } => write!(f, "Not found: {}", resource),
        }
    }
}

impl std::error::Error for AppError {}

// Auto-convert io::Error into AppError using From trait
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IoError(e)
    }
}

fn load(path: &str) -> Result<String, AppError> {
    let content = std::fs::read_to_string(path)?;  // io::Error auto-converted via From
    if content.is_empty() {
        return Err(AppError::ParseError("file is empty".to_string()));
    }
    Ok(content)
}
```

---

## 5. `thiserror` Crate — Idiomatic Custom Errors

```toml
[dependencies]
thiserror = "1"
```

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum ServiceError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("not found: {resource}")]
    NotFound { resource: String },

    #[error("validation failed: {0}")]
    Validation(String),
}

// Now ? auto-converts sqlx::Error → ServiceError::Database
async fn get_user(id: u64) -> Result<User, ServiceError> {
    let user = db.find_user(id).await?;  // sqlx::Error → ServiceError
    Ok(user)
}
```

---

## 6. `anyhow` Crate — Application-Level Error Handling

```toml
[dependencies]
anyhow = "1"
```

```rust
use anyhow::{Context, Result, bail, ensure};

fn process_file(path: &str) -> Result<i32> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read '{}'", path))?;

    let n: i32 = content.trim().parse()
        .context("File must contain a valid integer")?;

    ensure!(n > 0, "Number must be positive, got {}", n);

    if n > 1000 {
        bail!("Number {} exceeds maximum of 1000", n);
    }

    Ok(n * 2)
}

fn main() -> Result<()> {
    let result = process_file("input.txt")?;
    println!("Result: {}", result);
    Ok(())
}
```

---

## 7. panic! — When to Use

```rust
fn main() {
    // Appropriate panics: invariant violations that should NEVER happen
    let v = vec![1, 2, 3];
    let _ = v[10];           // panics: index out of bounds

    // assert! for invariants
    assert!(2 + 2 == 4, "Math is broken");
    assert_eq!(4, 2 + 2, "Expected 4");

    // unreachable! for impossible code paths
    let direction = "north";
    let _ = match direction {
        "north" | "south" => true,
        "east"  | "west"  => false,
        _ => unreachable!("Invalid direction: {}", direction),
    };

    // todo! and unimplemented! for stubs
    fn not_done_yet() -> i32 { todo!() }
}
```

**Rule:** Use `panic!` for *programmer errors* (bugs). Use `Result` for *user/environment errors*.

---

## 8. Option<T> as Error Handling

```rust
fn find_first_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x % 2 == 0).copied()
}

fn main() {
    // ? works with Option too (in functions returning Option)
    fn double_first(v: &[i32]) -> Option<i32> {
        let first = v.first()?;   // None if empty
        Some(first * 2)
    }

    // Convert Option → Result
    let result: Result<i32, &str> = find_first_even(&[1, 3, 5])
        .ok_or("no even numbers");

    println!("{:?}", result);  // Err("no even numbers")
}
```

---

## 9. Tricky Questions

### Q1 — What does `?` desugar to?
```rust
fn foo() -> Result<i32, String> {
    let x = "42".parse::<i32>()?;
    Ok(x)
}
```
**Answer:** Roughly equivalent to:
```rust
let x = match "42".parse::<i32>() {
    Ok(val) => val,
    Err(e)  => return Err(From::from(e)),  // From trait converts error type
};
```

---

### Q2 — Compile error?
```rust
fn main() {
    let n = "abc".parse::<i32>()?;
    println!("{}", n);
}
```
**Answer:** ❌ Compile error — `?` can only be used in functions returning `Result` or `Option`. `main()` returns `()`. Fix: change to `fn main() -> Result<(), Box<dyn std::error::Error>>`.

---

### Q3 — What prints?
```rust
fn might_fail(fail: bool) -> Result<i32, String> {
    if fail { Err("oops".to_string()) } else { Ok(42) }
}

fn main() {
    let v = might_fail(true).unwrap_or_else(|e| {
        println!("handling: {}", e);
        -1
    });
    println!("v = {}", v);
}
```
**Answer:**
```
handling: oops
v = -1
```

---

### Q4 — `Box<dyn Error>` vs custom enum
**Question:** When should you use `Box<dyn Error>` vs a custom error enum?  
**Answer:**
- `Box<dyn Error>` → application binaries, prototyping (use `anyhow`)
- Custom enum → libraries (callers need to match on error variants)
- `thiserror` generates custom enum boilerplate automatically

---

### Q5 — Tricky unwrap cascade
```rust
fn main() {
    let s = Some("42");
    let n: i32 = s.unwrap().parse().unwrap();
    println!("{}", n);
}
```
**Answer:** ✅ `42` — but this is fragile. If `s` is `None` or parse fails, it panics. Prefer:
```rust
let n: Option<i32> = s.and_then(|s| s.parse().ok());
```

---

## 10. Key Takeaways

| Tool | Use For |
|------|---------|
| `Result<T, E>` | All recoverable errors — function return types |
| `?` operator | Propagate errors up the call stack cleanly |
| `panic!` | Programmer bugs, invariant violations |
| `thiserror` | Library crates with typed error enums |
| `anyhow` | Application binaries with rich context |
| `Option::ok_or` | Convert Option → Result |
| `From` trait | Automatic error type conversion for `?` |
