# 🦀 Module 1 — Rust Mindset & Toolchain (Deep Dive)

> **Target Audience:** C / C++ / Java / Python developers  
> **Duration:** 2 Hours

---

## 1. Why Rust Exists — The Mental Shift

Rust's core promise: **Memory Safety + Zero-Cost Abstractions + No GC**.

| Problem in Old Languages | Rust Solution |
|--------------------------|---------------|
| `C`: `malloc` / `free` mismatch → use-after-free | Ownership drops memory automatically |
| `C++`: shared pointers + manual threading → data races | Borrow checker enforced at compile time |
| `Java`: GC pauses of 10–500ms in latency-critical code | No GC — deterministic drop |
| `Python`: GIL blocks true parallelism | True OS threads, fearless concurrency |

**Key Mental Model:** Rust moves *runtime* errors to *compile time*. If it compiles, it is safe.

---

## 2. Toolchain Components

```
rustup   →  manages toolchain versions (stable / beta / nightly)
rustc    →  the compiler (you rarely call this directly)
cargo    →  build system + package manager (like Maven/pip/npm)
clippy   →  lint tool (cargo clippy)
rustfmt  →  formatter  (cargo fmt)
rust-analyzer → LSP for IDEs
```

### Installing & Switching Toolchains

```bash
# Install stable Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Show installed toolchains
rustup show

# Add nightly for experimental features
rustup install nightly

# Pin a project to nightly
echo "nightly" > rust-toolchain.toml

# Update everything
rustup update
```

---

## 3. Cargo — The Build System

```bash
cargo new myapp           # binary crate (has src/main.rs)
cargo new mylib --lib     # library crate (has src/lib.rs)
cargo build               # debug build  → target/debug/myapp
cargo build --release     # release build → target/release/myapp (opt-level=3)
cargo run                 # build + run
cargo test                # run all tests
cargo clippy              # lint
cargo fmt                 # format code
cargo doc --open          # generate + open HTML docs
cargo add serde           # add dependency to Cargo.toml
cargo tree                # show dependency graph
```

### Cargo.toml Structure

```toml
[package]
name    = "myapp"
version = "0.1.0"
edition = "2021"          # Rust editions: 2015, 2018, 2021

[dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1",  features = ["full"] }

[dev-dependencies]
pretty_assertions = "1"   # Only used in tests

[profile.release]
opt-level = 3
lto       = true          # Link-time optimization
strip     = true          # Strip debug symbols
```

---

## 4. Workspace Architecture

Used in enterprise — multiple crates sharing one `Cargo.lock`:

```toml
# workspace Cargo.toml
[workspace]
members = [
    "crates/domain",
    "crates/api",
    "crates/infra",
]
resolver = "2"
```

```bash
cargo build -p domain     # build only one member
cargo test  -p api        # test only one member
```

---

## 5. Sample Program — Hello Workspace

```rust
// src/main.rs
fn main() {
    let name = "Rust";
    let year = 2015_u32;          // _ separator for readability
    println!("Hello, {}! Born in {}.", name, year);

    // Rust infers types — no need to annotate obvious cases
    let pi = 3.14_f64;
    let is_safe = true;

    println!("π ≈ {:.2}, safe = {}", pi, is_safe);
}
```

**Run it:**
```bash
cargo new hello_workspace
cd hello_workspace
cargo run
```

---

## 6. Understanding `rustc` Errors — A Feature, Not a Bug

```rust
fn main() {
    let x = 5;
    x = 6;          // ❌ ERROR: cannot assign twice to immutable variable
}
```

Rust's compiler error messages are famous for being *actionable*:
```
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:3:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

Fix: `let mut x = 5;`

---

## 7. Tricky Questions

### Q1 — What prints?
```rust
fn main() {
    let x = 5;
    let x = x + 1;      // shadowing!
    let x = x * 2;
    println!("{}", x);  // ?
}
```
**Answer:** `12`  
`5 + 1 = 6`, `6 * 2 = 12`. Shadowing creates a new binding — not mutation.

---

### Q2 — Will this compile?
```rust
fn main() {
    let x = 2.0;        // f64 by default
    let y: f32 = 3.0;
    println!("{}", x + y);
}
```
**Answer:** ❌ **No!** Rust does NOT do implicit numeric coercion.  
Fix: `println!("{}", x + y as f64);`

---

### Q3 — What is `cargo build --release` flag for?
**Answer:** Enables `opt-level=3` (full optimization), disables debug info, enables inlining → binary is 10–100× faster for CPU-bound tasks but compile time is longer.

---

### Q4 — Spot the Bug
```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}
fn multiply(a: i32, b: i32) -> i32 {
    a * b;              // ❌ semicolon makes this a statement, not expression
}
```
**Answer:** `multiply` returns `()` (unit), not `i32`. Remove the `;`:  
`a * b` — tail expressions without `;` are the return value.

---

### Q5 — Edition Trap
```rust
// Rust 2015 vs 2021 difference
use std::collections::HashMap;

fn process(map: &mut HashMap<&str, i32>) {
    // In 2015 this sometimes needed NLL workaround
    // In 2021 Non-Lexical Lifetimes are default + improved
    if let Some(v) = map.get("key") {
        println!("{}", v);
    }
    map.insert("key2", 99);  // OK in 2021, was sometimes rejected in 2015
}
```
**Key Point:** Always use `edition = "2021"` in new projects.

---

## 8. Key Takeaways

- `rustup` manages toolchains; `cargo` manages projects and deps
- `cargo build --release` ≠ debug build — always benchmark with `--release`
- Shadowing (`let x = ...`) is different from mutation (`mut x`)
- Rust has no implicit type coercion — be explicit
- Compiler errors are your friend — read them, they suggest fixes
