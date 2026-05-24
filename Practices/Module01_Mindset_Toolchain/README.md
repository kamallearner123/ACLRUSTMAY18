# Module 1 — Rust Mindset & Toolchain (2 Hours)

## Objectives
- Understand why Rust exists and its enterprise position
- Setup professional development environment
- Create a Cargo workspace

## Practice Exercises

### 📁 `hello_workspace/` — Your First Cargo Project
```bash
cd hello_workspace
cargo run
cargo build --release
cargo doc --open
```

### 📁 Add a dependency
Edit `Cargo.toml` and add `rand = "0.8"`, then use it.

### 📁 Migration Discussion Code
- `python_comparison.rs` — GIL problem demonstration concept
- `cpp_comparison.rs` — Dangling pointer concept in C++ vs Rust

## Key Commands
```bash
rustup update
rustup show
cargo new my_project
cargo new --lib my_library
cargo add serde
cargo build
cargo run
cargo test
cargo doc
```

## ✅ Copy your practice programs here:
`Practices/Module01_Mindset_Toolchain/`
