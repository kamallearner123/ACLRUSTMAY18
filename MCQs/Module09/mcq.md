# MCQs — Module 9: Enterprise Project Architecture

> **Instructions:** Choose the best answer. Answers at the bottom.

---

**Q1.** What is a Cargo workspace?

- A) A single binary project
- B) A collection of related crates sharing a `Cargo.lock` and build output
- C) A Docker container for Rust projects
- D) A cloud deployment configuration

---

**Q2.** Which crate provides derive macros for serialization/deserialization?

- A) `tracing`
- B) `clap`
- C) `serde`
- D) `config`

---

**Q3.** In hexagonal architecture applied to Rust, where does business logic live?

- A) In the HTTP handler functions
- B) In the `domain` crate with no I/O dependencies
- C) In the database layer
- D) In `main.rs`

---

**Q4.** What does the `tracing` crate provide?

- A) HTTP request tracing only
- B) Structured, async-aware logging with spans and events
- C) Network packet inspection
- D) Memory allocation tracing

---

**Q5.** Which crate is used to parse command-line arguments with derive macros?

- A) `serde`
- B) `config`
- C) `clap`
- D) `dotenvy`

---

**Q6.** What does `dotenvy` do?

- A) Provides DNS lookup
- B) Loads environment variables from a `.env` file
- C) Manages Rust editions
- D) Generates API documentation

---

**Q7.** In a layered architecture, which layer should NOT import from infrastructure?

- A) Application layer
- B) API layer
- C) Domain layer
- D) Infrastructure layer

---

**Q8.** What does `#[derive(Serialize, Deserialize)]` require?

- A) The `tracing` crate
- B) The `serde` crate with the `derive` feature enabled
- C) The `clap` crate
- D) No additional dependencies

---

**Q9.** Feature flags in `Cargo.toml` allow:

- A) Runtime feature toggling
- B) Conditional compilation of code based on enabled features
- C) A/B testing in production
- D) Versioning of APIs

---

**Q10.** What is the purpose of `tracing::instrument`?

- A) Measures network latency
- B) Automatically creates a span for a function, recording its name and arguments
- C) Profiles memory allocations
- D) Adds retry logic to functions

---

## Answers

| Q | Answer | Explanation |
|---|--------|-------------|
| 1 | **B** | Workspace = multiple crates, one `Cargo.lock`, shared `target/` |
| 2 | **C** | `serde` provides `Serialize` and `Deserialize` derive macros |
| 3 | **B** | Domain layer has pure logic, no I/O — hexagonal architecture |
| 4 | **B** | `tracing` = structured, async-aware observability |
| 5 | **C** | `clap` parses CLI args with `#[derive(Parser)]` |
| 6 | **B** | `dotenvy` loads `.env` into environment variables |
| 7 | **C** | Domain layer must be pure — no I/O or framework dependencies |
| 8 | **B** | `serde = { features = ["derive"] }` enables derive macros |
| 9 | **B** | Feature flags control what code is compiled |
| 10 | **B** | `#[instrument]` auto-spans the function for distributed tracing |
