# MCQs — Module 7: Concurrency & Thread Safety

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** What do the `Send` and `Sync` marker traits guarantee?

- A) `Send`: safe to send between threads; `Sync`: safe to share reference across threads
- B) `Send`: fast message passing; `Sync`: synchronous I/O
- C) Both mean the type can be copied between threads
- D) They are compiler hints with no enforcement

---

**Q2.** Why is `Arc<T>` used instead of `Rc<T>` in multi-threaded code?

- A) `Arc` is faster than `Rc`
- B) `Arc` uses atomic reference counting — safe across threads; `Rc` is not `Send`
- C) `Rc` doesn't support generics
- D) `Arc` allows mutable access without `Mutex`

---

**Q3.** What is a "data race"?

- A) Two threads reading the same data simultaneously
- B) Two or more threads accessing shared data without synchronization, where at least one writes
- C) A performance bottleneck in concurrent code
- D) A race condition only in databases

---

**Q4.** Can data races occur in safe Rust?

- A) Yes, they are possible with `Arc<T>`
- B) Yes, with multiple immutable references
- C) No — the borrow checker and `Send`/`Sync` prevent data races at compile time
- D) Only in debug builds

---

**Q5.** What does `Mutex::lock()` return?

- A) The inner value directly
- B) A `MutexGuard` that releases the lock when dropped
- C) A `bool` indicating if the lock was acquired
- D) A `Result` with the thread ID

---

**Q6.** What is `mpsc` in `std::sync::mpsc`?

- A) Multi-producer, single-consumer channel
- B) Multi-process, single-core
- C) Message-passing, synchronized channel
- D) Mutex-protected, single-copy

---

**Q7.** What happens to a `Mutex` if the thread holding the lock panics?

- A) The lock is automatically released safely
- B) The mutex becomes "poisoned" — other threads get `Err(PoisonError)` on lock
- C) The entire program terminates
- D) The next thread gets the lock normally

---

**Q8.** `RwLock<T>` differs from `Mutex<T>` in that:

- A) `RwLock` allows only one reader at a time
- B) `RwLock` allows multiple concurrent readers OR one exclusive writer — better for read-heavy workloads
- C) `RwLock` is always faster than `Mutex`
- D) `Mutex` allows multiple writers

---

**Q9.** Which is the correct way to share a `Mutex` across multiple threads?

- A) `Mutex<T>` directly
- B) `Arc<Mutex<T>>`
- C) `Rc<Mutex<T>>`
- D) `Box<Mutex<T>>`

---

**Q10.** What is a "deadlock"?

- A) A crash caused by accessing freed memory
- B) A situation where two or more threads wait for each other to release locks indefinitely
- C) A stack overflow from infinite recursion
- D) A timeout in async code

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **A** | `Send` = ownership can cross thread boundary; `Sync` = `&T` can be shared |
| 2 | **B** | `Arc` uses atomic ops for thread-safe ref counting |
| 3 | **B** | Data race = concurrent unsynchronized access with at least one write |
| 4 | **C** | Safe Rust's type system prevents data races entirely |
| 5 | **B** | `MutexGuard<T>` holds the lock and releases it when it goes out of scope |
| 6 | **A** | `mpsc` = Multiple Producer, Single Consumer |
| 7 | **B** | Panic poisons the mutex — callers must handle `PoisonError` |
| 8 | **B** | `RwLock` allows N readers simultaneously, but only 1 writer at a time |
| 9 | **B** | `Arc<Mutex<T>>` = thread-safe shared mutable state |
| 10 | **B** | Deadlock = circular wait for locks that never resolves |
