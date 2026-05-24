# Module 11 — Tooling, Testing & Benchmarking (2 Hours)

## Practice Exercises

### 📁 `bench_demo/` — Criterion Benchmark Suite
Compare naive vs optimized algorithms

### 📁 `ci_pipeline/` — GitHub Actions CI Setup
Lint → test → benchmark → release build

## Key Commands
```bash
cargo test                    # run all tests
cargo test -- --nocapture     # show println! output
cargo clippy                  # linter
cargo fmt                     # formatter
cargo bench                   # benchmarks (criterion)
cargo miri test               # detect undefined behavior
cargo doc --open              # generate docs

# Install extra tools
cargo install cargo-tarpaulin  # code coverage
cargo install flamegraph        # profiling
```

## Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_panic() {
        divide(10, 0);
    }
}
```

## Criterion Benchmark
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n { 0 => 0, 1 => 1, n => fibonacci(n-1) + fibonacci(n-2) }
}

fn bench_fib(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, bench_fib);
criterion_main!(benches);
```

## ✅ Copy your practice programs here:
`Practices/Module11_Tooling_Testing_Benchmarking/`
