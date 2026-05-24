# MCQs — Module 5: Error Handling & Result-Oriented Design

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** What does the `?` operator do in a function returning `Result<T, E>`?

- A) Unwraps the value or panics
- B) Returns `Err(e)` from the function if the result is an error, otherwise unwraps the `Ok` value
- C) Converts any error to a string
- D) Suppresses errors silently

---

**Q2.** When should you use `panic!()` vs returning `Result`?

- A) `panic!()` for all errors; `Result` is deprecated
- B) `panic!()` for unrecoverable programming errors; `Result` for expected, recoverable failures
- C) `Result` for all errors; `panic!()` only in tests
- D) They are interchangeable

---

**Q3.** What crate provides the `#[derive(Error)]` macro for custom error types?

- A) `anyhow`
- B) `serde`
- C) `thiserror`
- D) `std::error`

---

**Q4.** What does `anyhow` provide over standard `Result`?

- A) Faster performance
- B) A single `anyhow::Error` that wraps any error type + `.context()` for messages
- C) Compile-time error checking
- D) Automatic retry logic

---

**Q5.** Which code is DANGEROUS in production?

- A) `result.unwrap_or_default()`
- B) `result?`
- C) `result.unwrap()`
- D) `result.map_err(|e| MyError::from(e))`

---

**Q6.** What does `#[from]` in `thiserror` do?

- A) Marks a variant for serialization
- B) Auto-implements `From<SourceError>` for your error type — enables `?` conversion
- C) Imports the error from another module
- D) Generates a `Display` impl

---

**Q7.** `Option<T>` and `Result<T, E>` differ in that:

- A) `Option` carries an error description; `Result` does not
- B) `Result` can describe WHY something failed; `Option` only signals presence/absence
- C) They are identical but with different names
- D) `Option` is for async code; `Result` is for sync

---

**Q8.** What is the purpose of `.context()` from `anyhow`?

- A) Unwraps the error
- B) Adds human-readable context to an error without changing its type
- C) Converts errors to strings for logging
- D) Catches panics

---

**Q9.** What does `unwrap_or_else(|e| fallback(e))` do?

- A) Panics with the error message
- B) Returns the `Ok` value or calls a closure with the error to produce a fallback
- C) Converts `Err` to `Option::None`
- D) Retries the operation on error

---

**Q10.** How do you convert between different error types without `anyhow`?

- A) Use `as` casting
- B) Implement `From<SourceError> for TargetError` — enables automatic `?` conversion
- C) Use `transmute`
- D) Wrap in `Box<dyn std::error::Error>`

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | `?` propagates `Err` upward, unwraps `Ok` — like `try` in other languages |
| 2 | **B** | `panic!` for bugs; `Result` for expected failures |
| 3 | **C** | `thiserror` derives Error trait implementations |
| 4 | **B** | `anyhow::Error` erases the type and allows `.context()` chaining |
| 5 | **C** | `.unwrap()` panics in production — use `?` or `unwrap_or` |
| 6 | **B** | `#[from]` generates `From` impl for seamless `?` error conversion |
| 7 | **B** | `Result<T,E>` carries error info; `Option<T>` only presence/absence |
| 8 | **B** | `.context("msg")` wraps error with additional description |
| 9 | **B** | Calls the closure with the error to produce a fallback value |
| 10 | **B** | `impl From<X> for Y` enables `?` to auto-convert error types |
