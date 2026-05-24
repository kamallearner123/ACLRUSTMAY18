# Module 7 — Concurrency & Thread Safety (3 Hours)

## Objectives
- Write fearless concurrent code
- Use Arc<Mutex<T>>, channels, RwLock
- Prevent data races at compile time

## Practice Exercises

### 📁 `thread_pool_demo/` — Multi-threaded Log Processor
Spawn N threads, distribute log lines, aggregate results

### 📁 `producer_consumer/` — Producer-Consumer with Channels
Use `std::sync::mpsc` for message passing

### 📁 `parallel_compute/` — Parallel Computation Engine
Compute sum of large array using thread partitioning

## Key Concepts
```rust
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

// Arc<Mutex<T>> — shared mutable state across threads
let counter = Arc::new(Mutex::new(0u32));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}
for h in handles { h.join().unwrap(); }
println!("Final count: {}", *counter.lock().unwrap());

// Channel — message passing
use std::sync::mpsc;
let (tx, rx) = mpsc::channel::<String>();
let tx2 = tx.clone();

thread::spawn(move || tx.send("Hello from thread 1".to_string()).unwrap());
thread::spawn(move || tx2.send("Hello from thread 2".to_string()).unwrap());

for msg in rx.iter().take(2) {
    println!("{}", msg);
}

// RwLock — multiple readers, one writer
let data = Arc::new(RwLock::new(vec![1, 2, 3]));
{
    let r = data.read().unwrap();  // multiple readers OK
    println!("{:?}", *r);
}
{
    let mut w = data.write().unwrap(); // exclusive write
    w.push(4);
}
```

## Python GIL vs Rust Fearless Concurrency
```python
# Python: GIL prevents true CPU parallelism for threads
import threading  # only concurrent, not parallel for CPU work
```
```rust
// Rust: Send + Sync enforced at compile time — true parallelism
// The compiler REJECTS data races — they cannot exist at runtime
```

## ✅ Copy your practice programs here:
`Practices/Module07_Concurrency/`
