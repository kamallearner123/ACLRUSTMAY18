# 🦀 Module 7 — Concurrency & Thread Safety (Deep Dive)

> **Rust's mantra: "Fearless Concurrency" — data races are compile-time errors.**

---

## 1. Threads — OS-Level Parallelism

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Spawn a thread
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..=3 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();  // wait for spawned thread to finish
}
```

### Moving Data Into Threads — `move` Closure

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // ❌ Without move — compiler rejects: can't borrow across thread boundary
    // let h = thread::spawn(|| println!("{:?}", v));

    // ✅ With move — v is moved INTO the thread's closure
    let h = thread::spawn(move || {
        println!("Thread owns: {:?}", v);
    });

    // println!("{:?}", v);  // ❌ v is moved

    h.join().unwrap();
}
```

---

## 2. Message Passing — Channels (`mpsc`)

`mpsc` = **m**ultiple **p**roducer, **s**ingle **c**onsumer.

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Clone tx for multiple producers
    let tx1 = tx.clone();
    thread::spawn(move || {
        let msgs = vec!["hello", "from", "thread 1"];
        for msg in msgs {
            tx1.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    thread::spawn(move || {
        let msgs = vec!["more", "messages", "here"];
        for msg in msgs {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    // rx.recv() blocks; rx as iterator receives until all senders drop
    for received in rx {
        println!("Got: {}", received);
    }
}
```

---

## 3. Shared State — `Mutex<T>` and `Arc<T>`

```
Mutex<T> — mutual exclusion lock (single-threaded: use RefCell<T>)
Arc<T>   — Atomically Reference Counted smart pointer (thread-safe Rc<T>)

Pattern: Arc<Mutex<T>> for shared mutable state across threads
```

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0_u32));
    let mut handles = vec![];

    for _ in 0..8 {
        let c = Arc::clone(&counter);
        let h = thread::spawn(move || {
            let mut num = c.lock().unwrap();   // blocks until lock acquired
            *num += 1;
            // Lock released automatically when `num` drops at end of block
        });
        handles.push(h);
    }

    for h in handles { h.join().unwrap(); }

    println!("Counter: {}", *counter.lock().unwrap());   // 8
}
```

### Deadlock — How It Happens and How to Avoid

```rust
// ⚠️ DEADLOCK EXAMPLE (don't run this!)
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(1));
    let b = Arc::new(Mutex::new(2));

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);

    let t1 = thread::spawn(move || {
        let _la = a2.lock().unwrap();   // locks A
        thread::sleep(std::time::Duration::from_millis(10));
        let _lb = b2.lock().unwrap();   // tries to lock B — DEADLOCK if t2 holds B
    });

    let t2 = thread::spawn(move || {
        let _lb = b.lock().unwrap();    // locks B
        thread::sleep(std::time::Duration::from_millis(10));
        let _la = a.lock().unwrap();    // tries to lock A — DEADLOCK if t1 holds A
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

// AVOID by: always acquiring locks in the SAME ORDER across all threads
```

---

## 4. `RwLock<T>` — Reader-Writer Lock

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Multiple readers simultaneously
    let mut handles = vec![];
    for i in 0..3 {
        let d = Arc::clone(&data);
        handles.push(thread::spawn(move || {
            let r = d.read().unwrap();    // multiple read locks OK
            println!("Reader {}: {:?}", i, *r);
        }));
    }

    for h in handles { h.join().unwrap(); }

    // Exclusive writer
    {
        let mut w = data.write().unwrap();
        w.push(4);
    }

    println!("After write: {:?}", *data.read().unwrap());
}
```

---

## 5. `Send` and `Sync` — The Safety Traits

```
Send  — type can be transferred (moved) to another thread
Sync  — type can be shared (&T reference) between threads
```

| Type | Send | Sync | Notes |
|------|------|------|-------|
| `i32`, `f64`, `bool` | ✅ | ✅ | Primitive types |
| `String`, `Vec<T>` | ✅ | ✅ | Owned types |
| `Rc<T>` | ❌ | ❌ | Not thread-safe; use `Arc<T>` |
| `RefCell<T>` | ✅ | ❌ | Interior mutability, not Sync |
| `Mutex<T>` | ✅ | ✅ | Send+Sync when T: Send |
| `*mut T` (raw pointer) | ❌ | ❌ | Unsafe |

---

## 6. Atomic Types — Lock-Free Primitives

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            c.fetch_add(1, Ordering::Relaxed);
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Atomic counter: {}", counter.load(Ordering::Relaxed));
}
```

### Memory Ordering (Simplified)

| Ordering | Meaning |
|----------|---------|
| `Relaxed` | No synchronization guarantee — just atomicity |
| `Acquire` | Reads before this can't be reordered after |
| `Release` | Writes after this can't be reordered before |
| `SeqCst` | Total ordering across all threads (safest, slowest) |

---

## 7. Rayon — Data Parallelism

```toml
[dependencies]
rayon = "1"
```

```rust
use rayon::prelude::*;

fn main() {
    let v: Vec<i64> = (1..=1_000_000).collect();

    // Sequential
    let sum_seq: i64 = v.iter().map(|&x| x * x).sum();

    // Parallel — just change iter() to par_iter()!
    let sum_par: i64 = v.par_iter().map(|&x| x * x).sum();

    assert_eq!(sum_seq, sum_par);
    println!("Sum of squares: {}", sum_par);

    // Parallel sort
    let mut data: Vec<i32> = (0..1_000_000).rev().collect();
    data.par_sort();
    println!("Sorted first: {}", data[0]);
}
```

---

## 8. Tricky Questions

### Q1 — Will this compile?
```rust
use std::thread;
fn main() {
    let x = 5;
    thread::spawn(|| println!("{}", x));
}
```
**Answer:** ❌ The closure borrows `x` from main, but the thread might outlive main. Use `move || println!("{}", x)` — copies `x` (it's `i32`, Copy).

---

### Q2 — Mutex Poisoning
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Arc::new(Mutex::new(0));
    let m2 = Arc::clone(&m);

    let h = thread::spawn(move || {
        let _guard = m2.lock().unwrap();
        panic!("thread panics while holding lock!");
    });

    let _ = h.join();  // join returns Err (thread panicked)

    match m.lock() {
        Ok(g)  => println!("got lock: {}", *g),
        Err(e) => println!("poisoned! recovered: {}", *e.into_inner()),
    }
}
```
**Answer:** When a thread panics while holding a `Mutex`, the mutex is **poisoned**. `lock()` returns `Err(PoisonError)`. You can still recover the data with `.into_inner()`.

---

### Q3 — Arc vs Rc
**Question:** Why can't you use `Rc<Mutex<T>>` for shared state across threads?  
**Answer:** `Rc<T>` is not `Send` — its reference count is not atomic. Moving `Rc` to another thread could cause race conditions on the count. Use `Arc<T>` (atomic ref count).

---

### Q4 — Channel Direction
```rust
use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel::<i32>();
    let tx2 = tx.clone();
    // Can rx be cloned too?
    let rx2 = rx.clone();   // ❌?
}
```
**Answer:** ❌ `Receiver` cannot be cloned — it's **single consumer**. For multiple consumers use `Arc<Mutex<Receiver<T>>>` or switch to `tokio::sync::broadcast`.

---

### Q5 — Data Race Prevention
```rust
fn main() {
    let mut data = vec![1, 2, 3];
    let r1 = &data;
    let r2 = &data;
    data.push(4);             // ❌?
    println!("{:?} {:?}", r1, r2);
}
```
**Answer:** ❌ This is the **same borrow checker rule** that prevents data races across threads — you can't mutate while immutably borrowed. Rust's single-ownership prevents data races both within and across threads.

---

## 9. Key Takeaways

| Tool | Use Case |
|------|----------|
| `thread::spawn` | CPU-bound parallel work |
| `mpsc::channel` | Send data between threads (message passing) |
| `Arc<Mutex<T>>` | Shared mutable state across threads |
| `RwLock<T>` | Many readers, occasional writer |
| `AtomicUsize` etc. | Simple lock-free counters/flags |
| `rayon::par_iter()` | Trivially parallelize iterator pipelines |
| `Send` / `Sync` | Compiler-enforced thread-safety contracts |
