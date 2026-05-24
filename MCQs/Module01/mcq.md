# MCQs — Module 1: Rust Mindset & Toolchain

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** Which command creates a new Rust binary project named `myapp`?

- A) `cargo init myapp`
- B) `cargo new myapp`
- C) `rustc new myapp`
- D) `cargo build myapp`

---

**Q2.** What does `cargo build --release` do differently from `cargo build`?

- A) It compiles faster with debug symbols included
- B) It applies optimizations and removes debug info for smaller, faster binaries
- C) It runs tests before building
- D) It uploads the binary to crates.io

---

**Q3.** Which tool manages Rust toolchain versions (stable, beta, nightly)?

- A) `cargo`
- B) `rustc`
- C) `rustup`
- D) `rustfmt`

---

**Q4.** What is `Cargo.toml` used for?

- A) Storing compiled artifacts
- B) Project metadata and dependency declarations
- C) Runtime configuration
- D) Compiler flags only

---

**Q5.** Which of these is a primary reason Rust was created?

- A) To replace Python for data science
- B) To provide memory safety without a garbage collector
- C) To simplify web frontend development
- D) To add dynamic typing to systems programming

---

**Q6.** What does `cargo doc --open` do?

- A) Opens the Rust Book in a browser
- B) Generates and opens HTML documentation for your project
- C) Opens the source code in your editor
- D) Downloads documentation from crates.io

---

**Q7.** In a Cargo workspace, where is the shared `Cargo.lock` located?

- A) In each member crate's directory
- B) In the workspace root directory
- C) In `~/.cargo/`
- D) There is no lock file in workspaces

---

**Q8.** Which problem does Rust's ownership system solve that C++ does NOT automatically prevent?

- A) Slow compilation speed
- B) Use-after-free bugs
- C) Missing semicolons
- D) Integer overflow

---

**Q9.** What is `crates.io`?

- A) Rust's official IDE
- B) Rust's package registry (like npm for JavaScript)
- C) Rust's CI/CD platform
- D) Rust's standard library documentation

---

**Q10.** Which command adds the `serde` crate to your project?

- A) `cargo install serde`
- B) `cargo add serde`
- C) `rustup add serde`
- D) `cargo fetch serde`

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | `cargo new myapp` creates a binary crate |
| 2 | **B** | `--release` enables `opt-level=3` and strips debug symbols |
| 3 | **C** | `rustup` manages toolchain versions |
| 4 | **B** | `Cargo.toml` is the project manifest |
| 5 | **B** | Memory safety without GC is Rust's core goal |
| 6 | **B** | Generates HTML docs from `///` comments and opens browser |
| 7 | **B** | Workspace root holds the single `Cargo.lock` |
| 8 | **B** | Ownership prevents use-after-free at compile time |
| 9 | **B** | crates.io is Rust's public package registry |
| 10 | **B** | `cargo add` edits `Cargo.toml` and fetches the crate |
