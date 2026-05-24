# MCQs — Module 10: Migration from Python / Java / C++

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** What is PyO3?

- A) A Python testing framework
- B) A Rust crate that allows creating native Python extension modules in Rust
- C) A Python-to-Rust transpiler
- D) An async Python runtime

---

**Q2.** The Python GIL (Global Interpreter Lock) limits:

- A) Memory usage
- B) True CPU parallelism across threads for Python code
- C) File I/O speed
- D) Import speed

---

**Q3.** Which `crate-type` is needed for a Rust library callable from Python?

- A) `rlib`
- B) `staticlib`
- C) `cdylib`
- D) `dylib`

---

**Q4.** In Java, `synchronized` is most closely equivalent to which Rust construct?

- A) `Arc<T>`
- B) `Mutex<T>` (specifically `Arc<Mutex<T>>` for shared state)
- C) `RwLock<T>`
- D) `Send` trait

---

**Q5.** What is `unsafe` in Rust?

- A) Code that is always buggy
- B) A block where the programmer takes responsibility for memory safety invariants the compiler cannot verify
- C) Code that runs slower
- D) A deprecated feature

---

**Q6.** FFI stands for:

- A) Fast Function Interface
- B) Foreign Function Interface — allows Rust to call C/C++ functions (and vice versa)
- C) File Format Interface
- D) Functional Framework Infrastructure

---

**Q7.** When migrating C++ to Rust, which C++ problem does Rust's ownership system DIRECTLY prevent?

- A) Integer overflow
- B) Use-after-free and double-free memory errors
- C) Division by zero
- D) Slow compilation

---

**Q8.** What does `extern "C"` in Rust declare?

- A) An external Rust crate
- B) A function using the C ABI — enables calling C library functions
- C) A C++-compatible class
- D) An unsafe block

---

**Q9.** Which Java concept maps most directly to Rust's `Result<T, E>`?

- A) `Optional<T>`
- B) `synchronized`
- C) Checked exceptions (`throws Exception`)
- D) Interfaces

---

**Q10.** Why is Python → Rust migration beneficial for CPU-intensive tasks?

- A) Rust has simpler syntax
- B) Rust compiles to native machine code without GC pauses or GIL overhead, giving 10–100× speedups
- C) Rust has better library support
- D) Python cannot do CPU-intensive tasks at all

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | PyO3 compiles Rust into `.so`/`.pyd` files callable from Python |
| 2 | **B** | GIL prevents Python threads from running CPU code in parallel |
| 3 | **C** | `cdylib` produces a dynamic library compatible with C/Python ABI |
| 4 | **B** | `Arc<Mutex<T>>` provides synchronized shared mutable access across threads |
| 5 | **B** | `unsafe` = programmer asserts safety guarantees the compiler can't verify |
| 6 | **B** | FFI = Foreign Function Interface for cross-language calls |
| 7 | **B** | Ownership eliminates use-after-free and double-free entirely |
| 8 | **B** | `extern "C"` uses the C calling convention for interop |
| 9 | **C** | Checked exceptions = must handle; `Result` = must handle |
| 10 | **B** | Native code + no GC = massive performance gains for CPU work |
