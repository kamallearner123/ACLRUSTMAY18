# MCQs — Module 6: Traits, Generics & Lifetimes

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** What is "monomorphization" in Rust generics?

- A) Converting generics to dynamic dispatch at runtime
- B) The compiler generates separate concrete code for each type used with a generic function — no runtime overhead
- C) A form of reflection
- D) Converting trait objects to concrete types

---

**Q2.** What is the difference between `impl Trait` (static dispatch) and `dyn Trait` (dynamic dispatch)?

- A) `dyn Trait` is always faster
- B) `impl Trait` resolves at compile time (zero-cost); `dyn Trait` uses a vtable at runtime
- C) `impl Trait` requires `Box`; `dyn Trait` does not
- D) They are identical in performance

---

**Q3.** What does a lifetime annotation `'a` in `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` mean?

- A) The returned reference lives for exactly `'a` nanoseconds
- B) The returned reference's lifetime is at least as long as the shorter of `x` and `y`'s lifetimes
- C) Both inputs must have the same length
- D) The function runs for at most `'a` seconds

---

**Q4.** The "orphan rule" in Rust states:

- A) Traits cannot have default implementations
- B) You can only implement a trait for a type if either the trait OR the type is defined in your crate
- C) Generics cannot implement traits from other crates
- D) Trait objects cannot be stored in `Vec`

---

**Q5.** Which trait allows a type to be converted from another type?

- A) `Into`
- B) `From`
- C) `AsRef`
- D) `Deref`

---

**Q6.** What are "associated types" in a trait?

- A) Methods that are associated with each other
- B) Type placeholders in a trait definition that implementors must specify
- C) Private types within a struct
- D) Types that implement the `Copy` trait

---

**Q7.** What does `where T: Clone + Send` mean in a generic function?

- A) The type `T` must be in the `Clone` or `Send` module
- B) `T` must implement both the `Clone` trait AND the `Send` trait
- C) `T` clones itself before being sent
- D) Either `Clone` or `Send` is sufficient

---

**Q8.** Which type allows a dynamically-dispatched collection of different types sharing a trait?

- A) `Vec<T>`
- B) `Vec<Box<dyn MyTrait>>`
- C) `Vec<impl MyTrait>`
- D) `Arc<T>`

---

**Q9.** What does `Deref` coercion allow?

- A) Converts raw pointers to references
- B) Automatically dereferences smart pointers — e.g., `Box<String>` can be used as `&str`
- C) Makes all types cloneable
- D) Allows mutable references to become immutable

---

**Q10.** In Java, interfaces are similar to Rust traits. What key advantage do Rust traits have?

- A) Rust traits support multiple inheritance of implementation
- B) Rust traits are zero-cost with static dispatch (monomorphization) and have the orphan rule for safety
- C) Rust traits support checked exceptions
- D) Rust traits allow null implementations

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | Compiler creates separate code per type — no runtime cost |
| 2 | **B** | `impl Trait` = compile-time; `dyn Trait` = vtable at runtime |
| 3 | **B** | Lifetime annotation constrains the output to live no longer than either input |
| 4 | **B** | Orphan rule prevents conflicting implementations across crates |
| 5 | **B** | `From<T>` converts from `T`; `Into` is blanket-implemented via `From` |
| 6 | **B** | Associated types are type placeholders: `type Output;` |
| 7 | **B** | `+` requires all listed trait bounds simultaneously |
| 8 | **B** | `Box<dyn Trait>` allows heterogeneous collections |
| 9 | **B** | `Deref` enables `*` and automatic coercions through smart pointer layers |
| 10 | **B** | Zero-cost static dispatch + coherence through the orphan rule |
