# Module 9 — Enterprise Project Architecture (2 Hours)

## Objectives
- Design Cargo workspaces with layered architecture
- Implement hexagonal/clean architecture in Rust
- Configure serde, tracing, clap, config

## Practice Exercises

### 📁 `enterprise_project/` — Full Project Scaffold
```
enterprise_project/
├── Cargo.toml          (workspace)
├── crates/
│   ├── domain/         (pure domain logic, no I/O)
│   ├── infrastructure/ (DB, HTTP, filesystem)
│   ├── application/    (use cases / services)
│   └── api/            (CLI or HTTP entry point)
```

## Key Crates
```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
clap = { version = "4", features = ["derive"] }
config = "0.14"
dotenvy = "0.15"
```

## Key Concepts
```rust
// Structured logging with tracing
use tracing::{info, warn, error, instrument};

#[instrument]
async fn process_request(id: u64) -> Result<(), Error> {
    info!(request_id = id, "Processing started");
    // ... work ...
    info!("Processing complete");
    Ok(())
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("info,my_app=debug")
        .init();
}

// CLI with clap derive
use clap::Parser;

#[derive(Parser)]
#[command(name = "myapp", about = "Enterprise Rust CLI")]
struct Cli {
    #[arg(short, long, default_value = "config.toml")]
    config: String,

    #[arg(short, long)]
    verbose: bool,
}

// Config management
use config::{Config, Environment, File};

let settings = Config::builder()
    .add_source(File::with_name("config"))
    .add_source(Environment::with_prefix("APP"))
    .build()?;
```

## ✅ Copy your practice programs here:
`Practices/Module09_Enterprise_Architecture/`
