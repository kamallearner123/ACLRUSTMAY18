# Module 8 — Async Rust with Tokio (4 Hours)

## Objectives
- Understand Futures, async/await, cooperative scheduling
- Build scalable TCP servers and REST APIs
- Use Axum framework for production services

## Practice Exercises

### 📁 `async_tcp_server/` — Async TCP Echo Server
Accept concurrent connections without blocking threads

### 📁 `axum_rest_api/` — REST API with Axum
CRUD endpoints, JSON request/response, error handling

### 📁 `websocket_chat/` — WebSocket Chat Server
Real-time bidirectional messaging

### 📁 `async_file_pipeline/` — Async File Processing Pipeline
Read → parse → transform → write using async streams

## Key Concepts
```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Connection from {}", addr);
        tokio::spawn(handle_connection(socket)); // non-blocking task
    }
}

async fn handle_connection(mut socket: TcpStream) {
    let mut buf = vec![0u8; 1024];
    loop {
        match socket.read(&mut buf).await {
            Ok(0) => break,          // connection closed
            Ok(n) => { socket.write_all(&buf[..n]).await.ok(); }
            Err(_) => break,
        }
    }
}

// Axum REST API
use axum::{routing::get, Router, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Health { status: String }

async fn health_check() -> Json<Health> {
    Json(Health { status: "ok".into() })
}

// select! — racing futures
use tokio::time::{sleep, Duration};
tokio::select! {
    result = fetch_data() => println!("Got data: {:?}", result),
    _ = sleep(Duration::from_secs(5)) => println!("Timeout!"),
}
```

## Tokio Internals
- Work-stealing scheduler across CPU cores
- Tasks are lightweight (not OS threads)
- `await` suspends task — scheduler runs other tasks
- No blocking: use `tokio::task::spawn_blocking` for CPU work

## ✅ Copy your practice programs here:
`Practices/Module08_Async_Tokio/`
