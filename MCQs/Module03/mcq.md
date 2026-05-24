# MCQs — Module 3: Structs, Enums & Pattern Matching

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** Which keyword defines methods on a struct in Rust?

- A) `fn`
- B) `impl`
- C) `trait`
- D) `method`

---

**Q2.** What is an "associated function" on a struct?

- A) A function that takes `&self` as first argument
- B) A function defined in a trait
- C) A function in an `impl` block that does NOT take `self` (often used as constructors)
- D) A function imported from another module

---

**Q3.** What does `Option<T>` represent?

- A) An error type
- B) A value that is either `Some(T)` or `None` — replacing null
- C) A thread-safe reference
- D) A generic collection

---

**Q4.** What is the output of this code?
```rust
let x = Some(5);
if let Some(n) = x {
    println!("{}", n);
}
```

- A) Nothing (no output)
- B) Compile error
- C) `5`
- D) `Some(5)`

---

**Q5.** Which pattern matches any value and binds it to a name in a `match` arm?

- A) `_`
- B) `..`
- C) `@` binding: `n @ 1..=10`
- D) A variable name like `x`

---

**Q6.** A `match` expression in Rust must be:

- A) Exhaustive (cover all possible cases)
- B) Limited to 10 arms
- C) Used only with enums
- D) Non-exhaustive by default

---

**Q7.** How do you define an enum variant that holds data?

- A) `enum Color { Red(u8, u8, u8) }`
- B) `enum Color { Red = (u8, u8, u8) }`
- C) `enum Color { Red: [u8; 3] }`
- D) `enum Color { Red { data: u8 } }` (only struct variants)

---

**Q8.** What does a "match guard" add to a match arm?

- A) A return type annotation
- B) An additional `if` condition that must be true for the arm to match
- C) A lifetime annotation
- D) A closure capture

---

**Q9.** Which Rust feature is the safest replacement for a C++/Java null reference?

- A) `Box<T>`
- B) `*mut T`
- C) `Option<T>`
- D) `Result<T, E>`

---

**Q10.** What is a tuple struct?

- A) A struct with named fields
- B) A struct with unnamed (positional) fields: `struct Point(f64, f64);`
- C) A tuple inside a Vec
- D) A struct that implements the `Tuple` trait

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | `impl TypeName { ... }` defines methods |
| 2 | **C** | Associated functions don't take `self`; `Type::new()` is common |
| 3 | **B** | `Option<T>` is Rust's null-safe alternative |
| 4 | **C** | `if let` destructures `Some(5)` and binds `n = 5` |
| 5 | **D** | A variable name in match binds the value; `_` discards it |
| 6 | **A** | Rust's `match` must cover all cases (exhaustiveness) |
| 7 | **A** | Tuple variant `Red(u8, u8, u8)` holds positional data |
| 8 | **B** | `arm if condition =>` adds a conditional guard |
| 9 | **C** | `Option<T>` forces explicit handling of absence |
| 10 | **B** | `struct Point(f64, f64)` is a tuple struct |
