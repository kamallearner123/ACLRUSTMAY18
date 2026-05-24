# 🦀 ACLRUSTMAY18 — Enterprise Rust Training (32 Hours)

> **Migrating C / C++ / Java to Rust** — Memory Safety for High-End Applications

---

## 📋 Course Overview

| Module | Topic | Hours |
|--------|-------|-------|
| Module 1  | Rust Mindset & Toolchain              | 2 |
| Module 2  | Ownership & Borrowing Deep Dive       | 4 |
| Module 3  | Structs, Enums, Pattern Matching      | 2 |
| Module 4  | Collections, Strings, Iterators       | 2 |
| Module 5  | Error Handling & Result-Oriented Design | 2 |
| Module 6  | Traits, Generics, Lifetimes           | 3 |
| Module 7  | Concurrency & Thread Safety           | 3 |
| Module 8  | Async Rust with Tokio                 | 4 |
| Module 9  | Enterprise Project Architecture       | 2 |
| Module 10 | Migration from Python / Java / C++   | 3 |
| Module 11 | Tooling, Testing, Benchmarking        | 2 |
| Module 12 | Capstone Project & Review             | 3 |
| **Total** |                                       | **32** |

---

## 📁 Repository Structure

```
ACLRUSTMAY18/
├── README.md                   ← You are here
├── Cargo.toml                  ← Workspace manifest
│
├── Practices/                  ← 🏋️ Hands-on practice programs (copy here!)
│   ├── Module01_Mindset_Toolchain/
│   ├── Module02_Ownership_Borrowing/
│   ├── Module03_Structs_Enums_Patterns/
│   ├── Module04_Collections_Strings_Iterators/
│   ├── Module05_Error_Handling/
│   ├── Module06_Traits_Generics_Lifetimes/
│   ├── Module07_Concurrency/
│   ├── Module08_Async_Tokio/
│   ├── Module09_Enterprise_Architecture/
│   ├── Module10_Migration/
│   ├── Module11_Tooling_Testing_Benchmarking/
│   └── Module12_Capstone/
│
└── MCQs/                       ← 📝 Multiple Choice Questions per module
    ├── Module01/mcq.md
    ├── Module02/mcq.md
    ├── ...
    └── Module12/mcq.md
```

> **📂 Where to copy your practice programs:**
> Place all your practice `.rs` files and Cargo projects inside the corresponding
> `Practices/ModuleXX_<name>/` folder. Each module folder has a `README.md`
> explaining the exercises.

---

## 🚀 Quick Start

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone the repo
git clone <repo-url>
cd ACLRUSTMAY18

# Verify toolchain
rustc --version
cargo --version

# Run any practice binary
cd Practices/Module01_Mindset_Toolchain/hello_workspace
cargo run
```

---

## 🎯 Why Rust?

| Language | Problem | Rust Solution |
|----------|---------|---------------|
| **C/C++** | Dangling pointers, double-free, data races | Ownership system eliminates at compile time |
| **Java**  | GC pauses, thread contention, JVM overhead | Zero-cost abstractions, fearless concurrency |
| **Python**| GIL limits parallelism, slow native code | Native speed + PyO3 FFI bridge |

---

## 📚 Learning Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Crates.io](https://crates.io)

---

## 🏆 Capstone Options

1. High-performance log analytics engine
2. Async backend service with Axum
3. Embedded telemetry collector
4. Python module accelerated with Rust (PyO3)
5. Legacy C++ parser migration

---

*Trainer: Kamal Kumar Mukiri | ACL Training*
