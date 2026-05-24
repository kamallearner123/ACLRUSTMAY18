# MCQs — Module 2: Ownership & Borrowing Deep Dive

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** What happens when you assign a `String` from one variable to another?

- A) The value is copied (both variables remain valid)
- B) The value is moved (original variable is invalidated)
- C) A deep clone is automatically made
- D) Both variables share a reference count

---

**Q2.** How many immutable references to a value can exist simultaneously?

- A) Only one
- B) None — references are always mutable
- C) Unlimited (any number)
- D) Two maximum

---

**Q3.** What does the Rust borrow checker enforce?

- A) Runtime memory bounds checking
- B) Compile-time validation of reference lifetimes and aliasing rules
- C) Stack overflow prevention
- D) Integer type promotion rules

---

**Q4.** Which of the following types implements the `Copy` trait?

- A) `String`
- B) `Vec<i32>`
- C) `i32`
- D) `Box<i32>`

---

**Q5.** What is a "dangling pointer"?

- A) A pointer that points to the heap
- B) A pointer referencing memory that has already been freed
- C) A null pointer
- D) A pointer inside a struct

---

**Q6.** What is the difference between `String` and `&str` in Rust?

- A) `String` is immutable; `&str` is mutable
- B) `String` is heap-owned and growable; `&str` is a borrowed string slice
- C) They are completely interchangeable
- D) `&str` is for Unicode; `String` is for ASCII

---

**Q7.** What will the following code produce?
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}", s1);
```

- A) Prints "hello"
- B) Compile error: value used after move
- C) Runtime panic
- D) Prints "hello hello"

---

**Q8.** What does RAII stand for in Rust?

- A) Resource Allocation Is Immediate
- B) Runtime Allocation and Index Integrity
- C) Resource Acquisition Is Initialization
- D) Reference Allocation Is Immutable

---

**Q9.** You can have at most _____ mutable reference(s) to a value in a given scope.

- A) 0
- B) 1
- C) 2
- D) Unlimited

---

**Q10.** Which smart pointer provides shared ownership with reference counting?

- A) `Box<T>`
- B) `Cell<T>`
- C) `Rc<T>`
- D) `&T`

---

**Q11.** What does the `Drop` trait allow?

- A) Marks a value as cloneable
- B) Runs custom cleanup code when a value goes out of scope
- C) Enables garbage collection
- D) Prevents a value from being moved

---

**Q12.** Which of the following correctly creates a mutable reference?

- A) `let r = &value;`
- B) `let r = &mut value;`
- C) `let mut r = &value;`
- D) `let r = mut &value;`

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | `String` does not implement `Copy`; assignment moves ownership |
| 2 | **C** | Multiple immutable (`&T`) references are allowed simultaneously |
| 3 | **B** | Borrow checker validates at compile time, not runtime |
| 4 | **C** | Scalar types like `i32`, `f64`, `bool`, `char` implement `Copy` |
| 5 | **B** | Dangling pointer = reference to freed/invalid memory |
| 6 | **B** | `String` owns heap data; `&str` borrows a string slice |
| 7 | **B** | Compile error — `s1` was moved into `s2` |
| 8 | **C** | RAII = Resource Acquisition Is Initialization |
| 9 | **B** | Only ONE mutable reference at a time in a scope |
| 10 | **C** | `Rc<T>` = Reference Counted, single-threaded shared ownership |
| 11 | **B** | `Drop::drop()` runs custom cleanup when value is destroyed |
| 12 | **B** | `&mut value` creates a mutable reference |
