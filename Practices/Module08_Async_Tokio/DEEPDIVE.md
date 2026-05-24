# 🦀 Module 8 — Async Rust with Tokio (Deep Dive)

> **Understanding non-blocking I/O, cooperative scheduling, and the Tokio runtime.**

---

## 1. What is Async Rust?

Async Rust allows you to write non-blocking code that runs concurrently on a small number of OS threads. 

### OS Threads vs Async Tasks
* **OS Threads**: Heavy (1-2 MB stack), slow context switches (managed by OS kernel).
* **Async Tasks (Green Threads/Coroutines)**: Lightweight (hundreds of bytes), fast user-space context switches (managed by Tokio executor).

### The Core: `Future` Trait
A `Future` is a state machine that represents a computation that might not be finished yet.
```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
* Futures are **lazy** — they do nothing until `.await`ed or spawned.
* **Poll Model**: Rust uses a *pull* model rather than a *push* model. The runtime polls the future to drive it to completion.

---

## 2. Tokio Runtime

Tokio is the industry-standard multi-threaded async runtime.

```rust
#[tokio::main] // Macro sets up the multithreaded scheduler
async fn main() {
    println!("Hello from async main!");
}
```

### Spawning Tasks
`tokio::spawn` submits a task to the multi-threaded executor. Tasks must be `'static` and `Send`.

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "Task finished"
    });

    // Do other work concurrently...
    println!("Doing other work...");

    let result = handle.await.unwrap();
    println!("Result: {}", result);
}
```

---

## 3. Async Synchronization Primitives

Tokio provides async-aware alternatives to `std::sync` primitives.

```
tokio::sync::mpsc       -> Multi-producer, single-consumer channel
tokio::sync::oneshot    -> Single-producer, single-consumer channel (one message)
tokio::sync::watch      -> Single-producer, multi-consumer (retains last value)
tokio::sync::broadcast  -> Multi-producer, multi-consumer (pub-sub)
tokio::sync::Mutex      -> Async-aware Mutex (yields thread instead of blocking)
```

### Async Channel Example (`mpsc`)
```rust
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        for i in 0..5 {
            tx.send(format!("Message {}", i)).await.unwrap();
            sleep(Duration::from_millis(100)).await;
        }
    });

    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}
```

---

## 4. Concurrent Control Flow: `join!` and `select!`

### `join!` — Run all concurrently and wait for all
```rust
use tokio::time::{sleep, Duration};

async fn task_one() { sleep(Duration::from_millis(100)).await; println!("One done"); }
async fn task_two() { sleep(Duration::from_millis(200)).await; println!("Two done"); }

#[tokio::main]
async fn main() {
    tokio::join!(task_one(), task_two());
}
```

### `select!` — Race multiple async operations (First to finish wins)
```rust
use tokio::time::{sleep, Duration};

async fn fast() -> &'static str { sleep(Duration::from_millis(50)).await; "fast" }
async fn slow() -> &'static str { sleep(Duration::from_millis(200)).await; "slow" }

#[tokio::main]
async fn main() {
    tokio::select! {
        res = fast() => println!("Winner: {}", res),
        res = slow() => println!("Winner: {}", res),
    }
} // The slow() future is automatically cancelled/dropped!
```

---

## 5. Tricky Questions

### Q1 — The Blocking Trap
```rust
#[tokio::main]
async fn main() {
    tokio::spawn(async {
        // CPU-heavy mathematical calculation (takes 5 seconds)
        let mut sum = 0;
        for i in 0..1_000_000_000 { sum += i; }
        println!("Math done: {}", sum);
    });

    tokio::spawn(async {
        // Will this be blocked?
        println!("Fast Task running");
    });
}
```
**Answer:** Yes! CPU-bound tasks or synchronous blocking operations (`std::thread::sleep`, synchronous database drivers, `std::fs`) will starve the Tokio thread pool.  
**Fix:** Use `tokio::task::spawn_blocking` to offload blocking tasks to a dedicated blocking pool.
```rust
let res = tokio::task::spawn_blocking(|| {
    // CPU-heavy or blocking synchronous calls here
}).await.unwrap();
```

---

### Q2 — Which Mutex to use: `std` or `tokio`?
**Question:** Should you always use `tokio::sync::Mutex` in async functions?  
**Answer:** **No!** Actually, `std::sync::Mutex` is faster and should be preferred if you only lock across non-async computation. Use `tokio::sync::Mutex` **only** if you need to hold the lock guard open *across an `.await` boundary*. Holding a `std::sync::MutexGuard` across an `.await` boundary will cause a compiler error because it is not `Send`.

---

### Q3 — Cancellation Safety
```rust
tokio::select! {
    val = rx.recv() => { println!("Received {:?}", val); }
    _ = tokio::time::sleep(Duration::from_secs(1)) => { println!("Timeout"); }
}
```
**Question:** Is `rx.recv()` cancellation safe here?  
**Answer:** Yes. `mpsc::Receiver::recv` is cancellation-safe because if it is cancelled before returning a value, the value remains in the channel.  
*Caution:* Some futures are NOT cancellation safe (e.g., reading bytes from a TCP stream into a buffer via `read_exact`). If cancelled, the bytes already read are lost.

---

### Q4 — Pinning and Unpin
**Question:** Why do we need `Pin` in Rust's Async?  
**Answer:** Futures are self-referential structures (they store references to their own local fields across `.await` points). If a self-referential struct is moved in memory, its pointers become invalid (dangling). `Pin` guarantees that a value will not be moved in memory, ensuring safety for self-referential types.

---

## 6. Key Takeaways

| API | Rule of Thumb |
|-----|---------------|
| `async fn` / `async {}` | Lazy. Does nothing until polled (via `.await` or `spawn`) |
| `tokio::spawn` | Creates an independent task, runs concurrently on the pool |
| `spawn_blocking` | Mandatory for synchronous I/O or long CPU calculations |
| `select!` | Ideal for timeouts, cancellation, or multiplexing |
| `join!` | Ideal for executing independent queries or requests concurrently |
