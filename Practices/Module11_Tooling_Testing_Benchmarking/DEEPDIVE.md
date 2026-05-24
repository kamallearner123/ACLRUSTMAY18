# 🦀 Module 11 — Tooling, Testing & Benchmarking (Deep Dive)

> **Maximizing product reliability and micro-optimizing performance.**

---

## 1. Unit & Integration Testing in Rust

Rust provides a built-in testing framework that compiles tests concurrently by default.

### Unit Tests (Inside the same file)
Placed at the bottom of the source file inside a `tests` module. They can test private functions.
```rust
fn add_positive(a: i32, b: i32) -> Option<i32> {
    if a < 0 || b < 0 { return None; }
    Some(a + b)
}

#[cfg(test)] // Compiled only when running "cargo test"
mod tests {
    use super::*; // Import parent module functions

    #[test]
    fn test_add_positive() {
        assert_eq!(add_positive(2, 2), Some(4));
    }

    #[test]
    #[should_panic(expected = "division by zero")]
    fn test_panic_expected() {
        let _ = 1 / 0;
    }
}
```

### Integration Tests (Outside the crate)
Placed in the `tests/` directory at the project root. They can only test the public API of the library.
```rust
// tests/integration_test.rs
use my_library_crate::add_positive;

#[test]
fn test_public_api() {
    assert_eq!(add_positive(5, 5), Some(10));
}
```

---

## 2. Mocking with `mockall`

Rust does not support runtime reflection, so traditional Java-style mocking (like Mockito) is not possible. We use code generation with `mockall`.

```toml
[dev-dependencies]
mockall = "0.11"
```

```rust
use mockall::{automock, predicate::*};

#[automock] // Generates MockDatabase interface
pub trait Database {
    fn get_user_score(&self, username: &str) -> u32;
}

fn double_score(db: &dyn Database, user: &str) -> u32 {
    db.get_user_score(user) * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mocked_db() {
        let mut mock_db = MockDatabase::new();
        mock_db.expect_get_user_score()
            .with(eq("Alice"))
            .times(1)
            .returning(|_| 50);

        assert_eq!(double_score(&mock_db, "Alice"), 100);
    }
}
```

---

## 3. High-Precision Benchmarking with `Criterion`

To measure performance down to nanoseconds and prevent statistical anomalies, Rust uses `Criterion`.

```toml
# Cargo.toml
[[bench]]
name = "math_benchmarks"
harness = false

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

```rust
// benches/math_benchmarks.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn expensive_calculation(n: u64) -> u64 {
    (0..n).fold(0, |acc, x| acc.wrapping_add(x))
}

fn bench_calculation(c: &mut Criterion) {
    c.bench_function("expensive_calculation_1000", |b| {
        // black_box prevents compiler from optimizing out the function call
        b.iter(|| expensive_calculation(black_box(1000)))
    });
}

criterion_group!(benches, bench_calculation);
criterion_main!(benches);
```

Run benchmarks: `cargo bench`

---

## 4. Key Developer Tooling Checklist

### Cargo Tarpaulin (Code Coverage)
Measures what percentage of your code is executed during tests.
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests --out Html
```

### Cargo Flamegraph (CPU Profiling)
Generates visual SVG graphs to see where your code spends most of its CPU time.
```bash
cargo install flamegraph
cargo flamegraph --bin myapp
```

### Security Audits
```bash
cargo install cargo-audit
cargo audit # Scans dependency tree for known CVE database hits
```

---

## 5. Tricky Questions

### Q1 — The `black_box` Mystery
**Question:** Why do we wrap arguments in `black_box` inside Criterion benchmarks?  
**Answer:** Without `black_box`, if the compiler sees `expensive_calculation(1000)` has constant inputs, it will evaluate the output at compile-time and replace the function call with a static constant value. Your benchmark will falsely report 0 nanoseconds execution time. `black_box` prevents compiler optimization tricks from bypassing runtime evaluation.

---

### Q2 — Testing Private Struct Fields
**Question:** Can integration tests in the `tests/` directory verify private struct members or private methods?  
**Answer:** No. Integration tests are compiled as distinct binary modules outside the crate structure. They can only access the public API. If you need to test private operations, use internal unit tests with `#[cfg(test)]`.

---

### Q3 — Concurrent Test Failures
**Question:** Why do test suites occasionally fail randomly in CI, but succeed when run one by one locally?  
**Answer:** Rust runs all test functions concurrently in parallel threads. If multiple tests access shared state (e.g. database records, global variables, environment values), they will conflict.  
**Fix:** Run tests sequentially using `cargo test -- --test-threads=1` or guarantee test isolation by creating distinct databases/files per test runner.

---

## 6. Key Takeaways

* **Cfg Flag**: Use `#[cfg(test)]` to ensure test framework code is entirely stripped out of production release builds.
* **Mocks**: Limit mocking to complex structural boundaries (e.g. network services). Avoid mocking clean domain components.
* **Criterion**: Always run benchmarks with `--release`. Debug mode contains heavy debug hooks, making timings inaccurate.
* **Flamegraph**: When profiling, keep symbols enabled in your release builds (`debug = true` in Cargo.toml [profile.release] profile).
