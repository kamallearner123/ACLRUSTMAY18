# MCQs — Module 4: Collections, Strings & Iterators

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** Which collection type preserves insertion order AND allows O(1) average lookups?

- A) `BTreeMap`
- B) `HashMap`
- C) `Vec` with binary search
- D) `HashSet`

---

**Q2.** What does `.collect()` do on an iterator?

- A) Applies a function to each element
- B) Consumes the iterator and builds a collection (e.g., `Vec`, `HashMap`)
- C) Filters elements
- D) Counts elements

---

**Q3.** What is the character encoding used by Rust's `String` type?

- A) ASCII
- B) UTF-16
- C) Latin-1
- D) UTF-8

---

**Q4.** What does `.map(|x| x * 2)` do on an iterator?

- A) Filters elements where `x * 2` is true
- B) Transforms each element by applying the closure
- C) Sums all elements multiplied by 2
- D) Sorts elements by `x * 2`

---

**Q5.** Rust iterators are:

- A) Eagerly evaluated — all elements computed immediately
- B) Lazily evaluated — elements computed only when consumed
- C) Always parallel
- D) Always sorted

---

**Q6.** What is the difference between `HashMap` and `BTreeMap`?

- A) `BTreeMap` is faster for insertions
- B) `HashMap` has no ordering; `BTreeMap` keeps keys sorted
- C) `HashMap` allows duplicate keys; `BTreeMap` does not
- D) They are identical

---

**Q7.** Which iterator method accumulates values into a single result?

- A) `.map()`
- B) `.filter()`
- C) `.fold()` / `.reduce()`
- D) `.zip()`

---

**Q8.** How do you get an iterator over a `Vec<T>` without consuming it?

- A) `vec.into_iter()`
- B) `vec.iter()`
- C) `vec.iter_mut()`
- D) `Iterator::from(vec)`

---

**Q9.** What does `.filter(|x| x > &5)` do?

- A) Keeps only elements greater than 5
- B) Removes elements greater than 5
- C) Replaces elements with their comparison result
- D) Panics if any element equals 5

---

**Q10.** In Rust, indexing into a `String` with `s[0]` is:

- A) Allowed and returns the first byte
- B) Not allowed — use `.chars().nth(0)` instead due to UTF-8
- C) Allowed and returns the first `char`
- D) Allowed only for ASCII strings

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | `HashMap` gives O(1) average lookup; order not guaranteed |
| 2 | **B** | `.collect()` materializes an iterator into a concrete collection |
| 3 | **D** | Rust `String` is always valid UTF-8 |
| 4 | **B** | `.map()` applies a transformation to each element |
| 5 | **B** | Rust iterators are lazy — adapters don't run until consumed |
| 6 | **B** | `BTreeMap` is sorted by key; `HashMap` is unordered |
| 7 | **C** | `.fold(init, |acc, x| ...)` folds elements into one value |
| 8 | **B** | `.iter()` yields `&T` references without consuming the Vec |
| 9 | **A** | `.filter()` keeps elements for which the predicate returns `true` |
| 10 | **B** | Cannot index String by byte — chars can be multi-byte in UTF-8 |
