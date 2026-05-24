# Module 3 — Structs, Enums & Pattern Matching (2 Hours)

## Objectives
- Model data with structs and enums
- Replace null with Option
- Design state machines with match

## Practice Exercises

### 📁 `state_machine/` — Automotive ECU State Machine
States: Off → Idle → Running → Fault

### 📁 `packet_parser/` — Network Packet Parser
Parse different packet types using enums

### 📁 `session_manager/` — Login/Session State Manager

## Key Concepts
```rust
// Struct with methods
struct Product {
    name: String,
    price: f64,
    quantity: u32,
}

impl Product {
    fn new(name: &str, price: f64, quantity: u32) -> Self {
        Product { name: name.to_string(), price, quantity }
    }
    fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

// Enum state machine
#[derive(Debug)]
enum EngineState {
    Off,
    Idle,
    Running { rpm: u32 },
    Fault(String),
}

impl EngineState {
    fn transition(&self, event: &str) -> EngineState {
        match (self, event) {
            (EngineState::Off, "start")  => EngineState::Idle,
            (EngineState::Idle, "accelerate") => EngineState::Running { rpm: 1000 },
            (EngineState::Running { .. }, "stop") => EngineState::Off,
            _ => EngineState::Fault("Invalid transition".to_string()),
        }
    }
}

// Option replaces null
fn find_user(id: u32) -> Option<String> {
    if id == 1 { Some("Alice".to_string()) } else { None }
}

if let Some(user) = find_user(1) {
    println!("Found: {}", user);
}
```

## Java vs Rust
```java
// Java — nullable reference (NPE risk)
String name = null;
System.out.println(name.length()); // NullPointerException!
```
```rust
// Rust — Option forces explicit handling
let name: Option<String> = None;
let len = name.as_ref().map(|s| s.len()).unwrap_or(0); // safe
```

## ✅ Copy your practice programs here:
`Practices/Module03_Structs_Enums_Patterns/`
