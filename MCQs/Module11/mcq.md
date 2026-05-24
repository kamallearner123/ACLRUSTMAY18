# MCQs — Module 11: Tooling, Testing & Benchmarking

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** Which attribute marks a function as a unit test in Rust?

- A) `#[unit_test]`
- B) `#[test]`
- C) `#[cfg(test)]`
- D) `#[bench]`

---

**Q2.** What does `cargo clippy` do?

- A) Compiles and runs the project
- B) Runs the Rust linter — catches common mistakes and suggests improvements
- C) Formats code according to Rust style guidelines
- D) Generates documentation

---

**Q3.** What is `Miri`?

- A) A Rust web framework
- B) An interpreter for Rust's MIR that detects undefined behavior in unsafe code
- C) A benchmarking tool
- D) A code coverage tool

---

**Q4.** Which crate provides micro-benchmarking for Rust?

- A) `cargo-test`
- B) `bench`
- C) `criterion`
- D) `flamegraph`

---

**Q5.** What does `#[should_panic]` on a test do?

- A) Skips the test
- B) Marks the test as expected to panic — the test PASSES if it panics
- C) Catches panics and logs them
- D) Makes the test run in a separate process

---

**Q6.** Integration tests in Rust are placed in:

- A) `src/tests/`
- B) `tests/` directory at the project root (each file is a separate test binary)
- C) `src/main.rs` with `#[integration_test]`
- D) `benches/` directory

---

**Q7.** What does `black_box()` do in Criterion benchmarks?

- A) Makes the benchmark run in debug mode
- B) Prevents the compiler from optimizing away the benchmarked computation
- C) Measures memory usage
- D) Prints the benchmark output in black text

---

**Q8.** `cargo fmt` uses which style configuration file?

- A) `.clippy.toml`
- B) `rustfmt.toml` or `.rustfmt.toml`
- C) `Cargo.toml`
- D) `.cargo/config.toml`

---

**Q9.** What does `cargo-tarpaulin` measure?

- A) Binary size
- B) Code coverage — which lines are exercised by tests
- C) Compilation speed
- D) Memory leaks

---

**Q10.** Which GitHub Actions step is correct for running Rust tests?

- A) `run: rust test`
- B) `run: cargo test --all`
- C) `run: rustup test`
- D) `run: cargo check --tests`

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | `#[test]` marks a function to run with `cargo test` |
| 2 | **B** | Clippy is the official Rust linter with 600+ lints |
| 3 | **B** | Miri detects UB, memory errors, and invalid operations in unsafe Rust |
| 4 | **C** | `criterion` provides statistical micro-benchmarking |
| 5 | **B** | `#[should_panic]` inverts the test: it passes if a panic occurs |
| 6 | **B** | `tests/` contains integration tests compiled as separate crates |
| 7 | **B** | `black_box` prevents dead-code elimination that would skew benchmarks |
| 8 | **B** | `rustfmt.toml` configures formatting rules |
| 9 | **B** | `cargo-tarpaulin` measures line/branch code coverage |
| 10 | **B** | `cargo test --all` runs tests for all workspace members |
