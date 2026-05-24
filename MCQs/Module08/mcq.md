# MCQs — Module 8: Async Rust with Tokio

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** What is a `Future` in Rust?

- A) A spawned OS thread
- B) A value representing a computation that will complete at some point — must be `.await`ed
- C) A timer callback
- D) A channel receiver

---

**Q2.** What does `#[tokio::main]` do?

- A) Creates a new OS process
- B) Sets up the Tokio async runtime and allows `main` to be `async`
- C) Spawns a background thread
- D) Enables multi-threading automatically

---

**Q3.** What is the difference between `tokio::spawn` and `std::thread::spawn`?

- A) `tokio::spawn` creates OS threads; `std::thread::spawn` creates async tasks
- B) `tokio::spawn` creates lightweight async tasks on the Tokio runtime; `std::thread::spawn` creates OS threads
- C) They are identical
- D) `tokio::spawn` is blocking; `std::thread::spawn` is non-blocking

---

**Q4.** What does `tokio::select!` do?

- A) Picks the fastest of multiple futures and cancels the rest
- B) Runs futures sequentially
- C) Combines futures into a single result
- D) Creates a timeout

---

**Q5.** Why should you NOT use blocking operations inside `async fn`?

- A) They cause compile errors
- B) Blocking stalls the Tokio thread, preventing other tasks from running on that thread
- C) They don't work in async context
- D) They are deprecated

---

**Q6.** What is "cooperative scheduling" in async Rust?

- A) The OS kernel schedules tasks
- B) Tasks explicitly yield control at `await` points; the runtime switches to other tasks
- C) Tasks compete for CPU time preemptively
- D) Tasks run on dedicated threads each

---

**Q7.** Which Axum type is used to return JSON responses?

- A) `HttpResponse`
- B) `Json<T>`
- C) `Response<Body>`
- D) `SerdeJson`

---

**Q8.** What does "pinning" a Future mean?

- A) Making it run on a specific CPU core
- B) Ensuring it doesn't move in memory after being polled — required for self-referential futures
- C) Copying it to the stack
- D) Making it thread-safe

---

**Q9.** What is the correct way to run a blocking operation inside Tokio without blocking the runtime?

- A) `tokio::spawn(blocking_fn())`
- B) `tokio::task::spawn_blocking(|| blocking_fn())`
- C) `async { blocking_fn() }.await`
- D) Use `std::thread::spawn` inside async

---

**Q10.** What does `async` before a function definition do?

- A) Makes the function run in a background thread
- B) Makes the function return a `Future` — it doesn't execute until `.await`ed
- C) Enables the function to panic safely
- D) Allows the function to have multiple return types

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | `Future` is a lazy computation — runs when polled/awaited |
| 2 | **B** | Sets up Tokio's async runtime for the `async main` function |
| 3 | **B** | Tokio tasks are green threads; `std::thread` are OS threads |
| 4 | **A** | `select!` races futures and cancels others when one completes |
| 5 | **B** | Blocking a Tokio thread prevents other tasks from progressing |
| 6 | **B** | Cooperative: tasks yield at `await` points voluntarily |
| 7 | **B** | `Json<T>` from Axum serializes `T` to JSON automatically |
| 8 | **B** | Pinning prevents memory moves required for self-referential structs |
| 9 | **B** | `spawn_blocking` runs blocking code on a dedicated thread pool |
| 10 | **B** | `async fn` returns a `Future` — deferred execution |
