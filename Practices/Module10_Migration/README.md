# Module 10 — Migration from Python / Java / C++ (3 Hours)

## Practice Exercises

### 🐍 Python → Rust
- `python_parser_port/` — Port CPU-intensive Python parser to Rust
- `pyo3_module/` — Expose Rust to Python via PyO3

### ☕ Java → Rust
- `java_service_rewrite/` — Migrate Spring Boot REST service to Axum

### ⚙️ C++ → Rust
- `cpp_port_safe/` — Port unsafe C++ module to safe Rust
- `ffi_demo/` — Call C library from Rust using `extern "C"`
- `benchmark_comparison/` — Criterion benchmarks: C++ vs Rust

## Key Migration Notes
| C++/Java Pattern | Rust Equivalent |
|---|---|
| Raw pointer `int*` | Slice `&[i32]` or `Vec<i32>` |
| `new` / `delete` | Ownership + Drop |
| `synchronized` | `Arc<Mutex<T>>` |
| `Optional<T>` | `Option<T>` |
| `throws Exception` | `Result<T, E>` |
| Virtual methods | `dyn Trait` |
| Templates | Generics `<T: Trait>` |

## ✅ Copy your practice programs here:
`Practices/Module10_Migration/`
