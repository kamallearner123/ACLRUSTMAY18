# Module 12 — Capstone Project & Review (3 Hours)

## Capstone Options

### Option 1: High-Performance Log Analytics Engine
- Ingest millions of log lines/sec
- Multi-threaded parsing pipeline
- Aggregate stats with HashMap
- Output JSON report

### Option 2: Async Backend Service with Axum
- REST API with CRUD operations
- JWT authentication
- SQLite or in-memory store
- Structured logging with tracing

### Option 3: Embedded Telemetry Collector
- Collect CPU/memory metrics
- Format as Prometheus metrics
- HTTP endpoint for scraping

### Option 4: Python Module Accelerated with Rust (PyO3)
- Profile Python bottleneck
- Rewrite in Rust
- Expose via PyO3
- Benchmark comparison

### Option 5: Legacy C++ Parser Migration
- Take a C++ string parser
- Rewrite safely in Rust
- Add comprehensive tests
- Benchmark to prove parity

## Recommended Project Structure
```
capstone/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── config.rs
│   ├── error.rs
│   └── handlers/
├── tests/
│   └── integration_test.rs
├── benches/
│   └── perf_bench.rs
└── .github/
    └── workflows/
        └── ci.yml
```

## Interview Topics to Prepare
- Explain ownership vs garbage collection
- When would you use Arc vs Rc?
- What is the borrow checker? Give an example.
- Explain Send and Sync traits.
- async vs threads — when to use which?
- How does Rust eliminate data races?
- What is monomorphization?

## ✅ Copy your capstone project here:
`Practices/Module12_Capstone/`
