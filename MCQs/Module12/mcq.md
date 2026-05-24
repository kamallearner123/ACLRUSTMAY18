# MCQs — Module 12: Capstone Project & Review

> **Instructions:** These questions cover the entire 32-hour course. Answers at the bottom.

---

**Q1.** Which combination of types safely shares mutable state across async tasks?

- A) `Rc<RefCell<T>>`
- B) `Arc<Mutex<T>>`
- C) `Box<Cell<T>>`
- D) `&mut T`

---

**Q2.** What is zero-cost abstraction in Rust?

- A) Abstractions that only work in release mode
- B) High-level abstractions (iterators, generics) that compile to equally efficient code as hand-written low-level code
- C) Code that uses no heap memory
- D) Macros with no compile time cost

---

**Q3.** Given: `fn foo(v: Vec<i32>) -> Vec<i32>`. If you call `foo(data)`, what happens to `data`?

- A) `data` is cloned automatically
- B) `data` is moved into `foo` — the caller can no longer use it
- C) `data` is borrowed immutably
- D) `data` is passed by reference

---

**Q4.** In an enterprise Rust service, structured logging should use:

- A) `println!()` throughout
- B) `eprintln!()` for errors
- C) `tracing` crate with spans and fields for async-aware observability
- D) `log` crate macros only

---

**Q5.** What is the main advantage of Axum over writing a raw TCP server?

- A) Axum is faster than TCP
- B) Axum provides routing, middleware, extractors, and JSON handling built on Hyper/Tokio
- C) Axum doesn't require async
- D) Axum has no dependencies

---

**Q6.** Which Rust concept MOST directly replaces Java's garbage collector?

- A) `unsafe` blocks
- B) Ownership and the RAII pattern — memory freed deterministically when owner goes out of scope
- C) `Rc<T>` reference counting
- D) The `Drop` trait alone

---

**Q7.** You want to write a function that accepts both `String` and `&str`. The best parameter type is:

- A) `String`
- B) `&str`
- C) `impl AsRef<str>`
- D) `Box<str>`

---

**Q8.** `cargo build` vs `cargo build --release`: which should be used in CI for performance benchmarks?

- A) `cargo build` — faster CI runs
- B) `cargo build --release` — optimized binary for accurate benchmarks
- C) They produce identical binaries
- D) Neither — use `cargo bench` directly

---

**Q9.** In the producer-consumer pattern, which Rust type is most appropriate?

- A) `Arc<Vec<T>>`
- B) `std::sync::mpsc` channel (or `tokio::sync::mpsc` for async)
- C) `Mutex<VecDeque<T>>` only
- D) `RwLock<Queue<T>>`

---

**Q10.** What makes Rust suitable for embedded systems compared to C++?

- A) Rust has a built-in RTOS
- B) Rust provides memory safety guarantees, no GC pauses, predictable performance, and `no_std` support
- C) Rust has smaller binary sizes
- D) Rust supports more hardware architectures

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | `Arc<Mutex<T>>` = thread-safe shared mutable state for async |
| 2 | **B** | Zero-cost: you pay only for what you use, no runtime overhead |
| 3 | **B** | `Vec<i32>` doesn't implement `Copy` — passing by value moves ownership |
| 4 | **C** | `tracing` with spans is the enterprise standard for async services |
| 5 | **B** | Axum abstracts routing, JSON, middleware — production-ready |
| 6 | **B** | Ownership + RAII = deterministic deallocation, no GC needed |
| 7 | **C** | `impl AsRef<str>` accepts `String`, `&str`, `&String` — most flexible |
| 8 | **B** | Always benchmark release builds — debug has no optimizations |
| 9 | **B** | Channels are designed for producer-consumer message passing |
| 10 | **B** | `no_std` + ownership + no GC = ideal for embedded systems |

---

## 🎯 Final Review Checklist

- [ ] Understood ownership, borrowing, and lifetimes
- [ ] Can use `Arc<Mutex<T>>` for shared state
- [ ] Can write `async fn` and use `tokio::spawn`
- [ ] Can define and implement custom traits with generics
- [ ] Can handle errors with `Result<T, E>` and `?`
- [ ] Can build a REST API with Axum
- [ ] Can write unit and integration tests
- [ ] Completed at least one capstone project option
