# Module 5 — Error Handling & Result-Oriented Design (2 Hours)

## Objectives
- Use Result<T,E> and Option for production error handling
- Propagate errors with `?` operator
- Create custom error types with `thiserror`
- Integrate structured logging

## Practice Exercises

### 📁 `config_loader/` — Resilient Configuration Loader
- Load from file, env vars, defaults
- Custom error types for each failure mode

### 📁 `api_error_mapper/` — API Error Mapping
Map HTTP status codes to typed error variants

### 📁 `file_processor/` — File Processing with Recovery
Process files, skip bad records, log errors, continue

## Key Concepts
```rust
use thiserror::Error;
use std::fs;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },
    #[error("Parse error at line {line}: {msg}")]
    ParseError { line: usize, msg: String },
    #[error("Missing required key: {0}")]
    MissingKey(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

// ? operator for clean propagation
fn load_config(path: &str) -> Result<Config, ConfigError> {
    let content = fs::read_to_string(path)?;  // auto-converts io::Error
    let config = parse_config(&content)?;
    Ok(config)
}

// Handling multiple error types with anyhow
use anyhow::{Context, Result};

fn run() -> Result<()> {
    let data = fs::read_to_string("data.txt")
        .context("Failed to read data file")?;
    println!("Loaded {} bytes", data.len());
    Ok(())
}
```

## Dangerous Patterns to Avoid
```rust
// ❌ NEVER in production code
let value = result.unwrap();        // panics on Err
let value = result.expect("msg");   // panics with message

// ✅ Preferred
let value = result?;                // propagate
let value = result.unwrap_or(0);    // default
let value = result.unwrap_or_else(|e| { log_error(e); 0 }); // handle
```

## ✅ Copy your practice programs here:
`Practices/Module05_Error_Handling/`
