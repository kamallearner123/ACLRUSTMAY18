# 🦀 Module 12 — Capstone Project & Review (Deep Dive)

> **Applying production-grade architectural patterns, performance guidelines, and final review.**

---

## 1. Structuring a High-Performance Capstone Project

When building a high-performance system in Rust (like a log parser, custom reverse proxy, or metrics ingestion engine), developers should design for maximum memory efficiency, non-blocking asynchronous processing, and zero-allocation pipelines.

### Capstone Architecture Template: Log Analytics Engine

```
                             ┌───────────────────┐
                             │    TCP Stream     │
                             └─────────┬─────────┘
                                       │ Async Reader
                             ┌─────────▼─────────┐
                             │   Tokio Task      │ (Splits lines concurrently)
                             └─────────┬─────────┘
                                       │ MPSC Channel
                             ┌─────────▼─────────┐
                             │ Processing Engine │ (Zero-Allocation Parser via &str)
                             └─────────┬─────────┘
                                       │ Batch Write
                             ┌─────────▼─────────┐
                             │   Metrics Store   │ (Thread-safe Lockless Storage)
                             └───────────────────┘
```

---

## 2. Real-World Project Implementation: Zero-Allocation Log Parser

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

// Keep memory footprint close to zero by storing borrowed string slices (&str)
// instead of allocating new owned Strings per log entry.
#[derive(Debug, PartialEq)]
pub struct LogEntry<'a> {
    pub timestamp: &'a str,
    pub level: &'a str,
    pub message: &'a str,
}

impl<'a> LogEntry<'a> {
    // Zero allocations: parses standard "TIMESTAMP [LEVEL] MESSAGE" format
    pub fn parse(raw: &'a str) -> Option<Self> {
        let mut parts = raw.splitn(3, ' ');
        let timestamp = parts.next()?;
        let raw_level = parts.next()?;
        let message = parts.next()?;
        
        // Validate level bracket formatting
        if !raw_level.starts_with('[') || !raw_level.ends_with(']') {
            return None;
        }
        let level = &raw_level[1..raw_level.len() - 1];

        Some(Self { timestamp, level, message })
    }
}

// In-Memory concurrent analytics aggregator using lock-free architecture where possible
#[derive(Default)]
pub struct MetricsTracker {
    level_counts: RwLock<HashMap<String, usize>>,
}

impl MetricsTracker {
    pub async fn increment(&self, level: &str) {
        let mut write_guard = self.level_counts.write().await;
        *write_guard.entry(level.to_string()).or_insert(0) += 1;
    }

    pub async fn get_count(&self, level: &str) -> usize {
        let read_guard = self.level_counts.read().await;
        *read_guard.get(level).unwrap_or(&0)
    }
}

#[tokio::main]
async fn main() {
    let tracker = Arc::new(MetricsTracker::default());
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Spawn processing worker
    let worker_tracker = Arc::clone(&tracker);
    tokio::spawn(async move {
        while let Some(line) = rx.recv().await {
            // Lifetime bound prevents 'line' reference escape
            if let Some(entry) = LogEntry::parse(&line) {
                worker_tracker.increment(entry.level).await;
            }
        }
    });

    // Ingest data
    tx.send("2026-05-24T11:00:00 [ERROR] Connection lost".to_string()).await.unwrap();
    tx.send("2026-05-24T11:01:00 [INFO] Heartbeat OK".to_string()).await.unwrap();
    tx.send("2026-05-24T11:02:00 [ERROR] Database write failed".to_string()).await.unwrap();

    // Give asynchronous tasks time to process
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    println!("Total Errors: {}", tracker.get_count("ERROR").await); // 2
    println!("Total Info logs: {}", tracker.get_count("INFO").await); // 1
}
```

---

## 3. High-Performance Optimization Checklist

When transitioning standard Rust applications into high-throughput enterprise systems, apply this systemic performance roadmap:

1. **Memory Allocations**:
   * Avoid calling `.clone()` or `to_string()` in hot processing paths.
   * Prefer **borrowed structures** (`&str`, `&[u8]`) or cow structures (`Cow<'a, str>`).
   * Reuse buffers using `String::clear()` or `Vec::clear()` rather than reallocating.

2. **Concurrency**:
   * Switch heavy-weight `Mutex<T>` structures to lock-free atomic values (`AtomicUsize`, `AtomicBool`) where appropriate.
   * Use `dashmap` crate for highly concurrent parallel HashMap writes.

3. **Compiler Directives**:
   * Enable Link-Time Optimization (LTO) in `Cargo.toml` (`lto = true`).
   * Compile using target architecture-native CPU instructions: `RUSTFLAGS="-C target-cpu=native" cargo build --release`.

---

## 4. Tricky Review Questions

### Q1 — Memory Leaks in Rust
**Question:** Since Rust has no Garbage Collector and uses static lifetimes, is it impossible to leak memory?  
**Answer:** **No, memory leaks are possible.** Rust prevents unsafe memory corruption, but logical memory leaks can still occur.  
**How:** Creating circular references using reference counting pointers (`Rc` or `Arc` cycles) will cause structures to never drop. Alternatively, you can explicitly leak heap memory using `Box::leak()`, which converts a Box into a `'static` reference (often used for fast global allocations).

---

### Q2 — RefCell vs Mutex
**Question:** Both `RefCell` and `Mutex` implement "interior mutability". When should you choose one over the other?  
**Answer:**
* `RefCell<T>`: Single-threaded applications only. Borrows are checked at runtime rather than compile-time. It is not thread-safe and does not implement `Sync`.
* `Mutex<T>`: Multi-threaded applications. Borrows are guarded by thread synchronization blocks. It blocks threads or yields async runtime loops to ensure safety across cores.

---

### Q3 — Custom Allocators
**Question:** Why do high-throughput web applications in Rust (like Discord backend engines) override the default system memory allocator?  
**Answer:** The standard glibc memory allocator can suffer from severe heap fragmentation and lock contention when multi-threaded async programs make thousands of small allocations. Override it with `jemalloc` or `mimalloc` to improve allocation speed and memory utilization.
```rust
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
```

---

## 5. Course Wrap-up Map

```
                          🚀 RUST EXPERTISE PATHWAY
                          
      [ Ownership & Borrowing ]   ──►   [ Structs & Enums ]
                  │                               │
                  ▼                               ▼
      [ Collections & Iterators ] ──►   [ Robust Error Handling ]
                  │                               │
                  ▼                               ▼
      [ Traits, Generics & Lifetimes ] ──► [ Async Rust & Concurrency ]
                  │                               │
                  ▼                               ▼
      [ Enterprise Architectures ] ──►  [ Multi-Language Migration ]
```
