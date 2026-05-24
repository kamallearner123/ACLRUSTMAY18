# 🦀 Module 9 — Enterprise Project Architecture (Deep Dive)

> **Building maintainable, robust, and highly observable applications in Rust.**

---

## 1. Architectural Styles: Layered vs Hexagonal (Ports & Adapters)

In enterprise projects, decoupling business logic from I/O frameworks (Web servers, Databases) is essential.

### Hexagonal Architecture Structure in Rust

```
                    ┌─────────────────────────┐
                    │       Infrastructure    │ (Database, HTTP API Client)
                    │  (implements SPI Ports) │
                    └────────────┬────────────┘
                                 │ implements
                    ┌────────────▼────────────┐
                    │      Domain / Core      │ (Pure structs, domain entities)
                    │  (defines API/SPI Traits)│ (Zero direct I/O dependencies)
                    └────────────▲────────────┘
                                 │ uses
                    ┌────────────┴────────────┐
                    │       Application       │ (Use cases, orchestrators)
                    └─────────────────────────┘
```

* **API (Inbound Ports)**: Defines how callers interact with our system (e.g. traits for Use Cases).
* **SPI (Outbound Ports)**: Defines how our system interacts with external systems (e.g. traits for repositories, mailing).

---

## 2. Cargo Workspaces Setup

A workspace organizes multiple crates together, letting them share build artifacts and cargo settings.

### Workspace Root `Cargo.toml`
```toml
[workspace]
resolver = "2"
members = [
    "crates/domain",
    "crates/infra",
    "crates/api"
]

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
tracing = "0.1"
```

---

## 3. Dependency Injection in Rust

Rust doesn't have runtime reflection-based dependency injection containers like Spring (Java). Instead, we use compile-time dependency injection using **Traits and Generics** or **Trait Objects**.

### Trait-Based Dependency Injection Example
```rust
// Domain Layer - Pure Traits and Structs
pub struct User {
    pub id: u64,
    pub name: String,
}

pub trait UserRepository: Send + Sync {
    fn get_user(&self, id: u64) -> Result<User, String>;
}

// Application Layer - Services using Generic Injection
pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub fn get_user_name(&self, id: u64) -> Result<String, String> {
        let user = self.repo.get_user(id)?;
        Ok(user.name)
    }
}

// Infrastructure Layer - Real Repository implementation
pub struct PostgresUserRepository {
    db_conn: String, // representing live connection pool
}

impl UserRepository for PostgresUserRepository {
    fn get_user(&self, id: u64) -> Result<User, String> {
        // actual SQL execution...
        Ok(User { id, name: "Alice".into() })
    }
}
```

---

## 4. Observability: Structured Logging with Tracing

The standard `log` crate is simple. Enterprise Rust projects use the `tracing` ecosystem, which introduces **Spans** (temporal contexts) and **Events** (structured logs).

```rust
use tracing::{info, instrument, span, Level};

#[instrument(skip(db_pool))] // skip serialization of heavy parameters
async fn fetch_user_data(user_id: u32, db_pool: &str) -> Result<String, String> {
    info!("Starting database lookup");
    // This event is automatically contextualized inside the function span
    if user_id == 0 {
        tracing::error!("Invalid user ID queried");
        return Err("Not Found".into());
    }
    Ok("Alice".to_string())
}

#[tokio::main]
async fn main() {
    // Setup subscriber to format and output log events
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("Application started");
    let _ = fetch_user_data(42, "postgres://...").await;
}
```

---

## 5. Tricky Questions

### Q1 — Circular Workspace Dependencies
**Question:** If `crates/infra` needs to use models from `crates/api`, and `crates/api` needs mock engines from `crates/infra`, how do you resolve this?  
**Answer:** Rust workspaces do not allow circular dependency graphs. If you have a loop, it will fail to compile.  
**Fix:** Extrapolate the shared domain models into `crates/domain`. Both `infra` and `api` depend *only* on `domain`, avoiding cycles entirely. Keep the core pure.

---

### Q2 — Dynamic vs Static Dependency Injection
**Question:** Should you inject dependencies via `Box<dyn UserRepository>` (dynamic dispatch) or generic `R: UserRepository` (static dispatch)?  
**Answer:**
* **Static dispatch (`R: UserRepository`)**: Better performance (no vtable runtime indirection, compiler inlines calls). However, it propagates generic parameters up through all wrapping structs (e.g. `UserService<R>`), making implementation signatures highly complex.
* **Dynamic dispatch (`Box<dyn UserRepository>`)**: Simpler code signatures and cleaner application instantiation. Since repository calls usually involve network I/O, the tiny overhead of dynamic dispatch is completely negligible. Use `Arc<dyn UserRepository>` for standard multi-threaded web application services.

---

### Q3 — Context Propagation in Tracing
**Question:** How does `tracing` preserve log metadata across asynchronous context switches (`.await` points)?  
**Answer:** The `tracing` crate uses thread-local storage to track the active span. In async Rust, when an executor yields a future, tracing automatically backs up the current span and restores the correct span when the scheduler resumes that task on any thread. This is why you must avoid using manual thread-local storage in async code and use `tracing` instead.

---

## 6. Key Takeaways

* **Hexagonal Architecture**: Protect your business logic by keeping `Domain` free of databases or web server frameworks.
* **Traits as Boundaries**: Use Rust traits to define SPI contracts. Mock them easily in tests.
* **Tracing**: Adopt `tracing` + `#[instrument]` to ensure distributed traces are captured cleanly, including correlation IDs across microservices.
* **Clean Workspaces**: Use a centralized `Cargo.toml` workspace manifest with `[workspace.dependencies]` to avoid dependency version mismatch across crates.
