# 🦀 Module 3 — Structs, Enums & Pattern Matching (Deep Dive)

---

## 1. Structs — Custom Data Types

### Three Kinds of Structs

```rust
// 1. Named-field struct (most common)
struct Employee {
    name:   String,
    salary: f64,
    active: bool,
}

// 2. Tuple struct (lightweight wrapper)
struct Meters(f64);
struct Kilograms(f64);

// 3. Unit struct (marker type — no data)
struct Placeholder;
```

### Creating & Using Structs

```rust
fn main() {
    let emp = Employee {
        name:   String::from("Alice"),
        salary: 95_000.0,
        active: true,
    };
    println!("{} earns {:.2}", emp.name, emp.salary);

    // Struct update syntax (like spread in JS)
    let emp2 = Employee {
        name: String::from("Bob"),
        ..emp      // NOTE: emp.name is moved? No — only String fields move
                   // emp.salary (f64) and emp.active (bool) are Copy
    };
    // emp.name is still valid (we gave Bob a new name)
    // emp.salary and emp.active were COPIED, not moved
    println!("{} earns {:.2}", emp.name, emp.salary);  // ✅
}
```

### Methods & Associated Functions

```rust
struct Rectangle {
    width:  f64,
    height: f64,
}

impl Rectangle {
    // Associated function (like static method) — no `self`
    fn new(width: f64, height: f64) -> Self {
        Rectangle { width, height }   // field init shorthand
    }

    // Method — takes &self (immutable borrow)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Method — takes &mut self
    fn scale(&mut self, factor: f64) {
        self.width  *= factor;
        self.height *= factor;
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

fn main() {
    let mut r = Rectangle::new(4.0, 6.0);
    println!("Area: {}", r.area());
    r.scale(2.0);
    println!("After scale: {} x {}", r.width, r.height);
    println!("Square? {}", r.is_square());
}
```

---

## 2. Enums — Algebraic Data Types (ADTs)

Rust enums are *far* more powerful than C/Java enums — each variant can carry data.

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),                         // tuple variant
    Rectangle { width: f64, height: f64 }, // struct variant
    Triangle(f64, f64, f64),             // three sides
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r)              => std::f64::consts::PI * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(a, b, c) => {
                // Heron's formula
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle { width: 4.0, height: 3.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for s in &shapes {
        println!("{:?} → area = {:.2}", s, s.area());
    }
}
```

---

## 3. Option<T> — Replacing NULL

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

fn main() {
    // Pattern matching
    match divide(10.0, 2.0) {
        Some(v) => println!("Result: {}", v),
        None    => println!("Division by zero"),
    }

    // if let — ergonomic single-variant match
    if let Some(v) = divide(10.0, 0.0) {
        println!("Got {}", v);
    } else {
        println!("No result");
    }

    // unwrap_or, unwrap_or_else
    let result = divide(10.0, 3.0).unwrap_or(0.0);
    println!("unwrap_or: {:.3}", result);

    // map — transform the inner value
    let doubled = divide(10.0, 2.0).map(|v| v * 2.0);
    println!("doubled: {:?}", doubled);
}
```

---

## 4. Pattern Matching — Deep

```rust
fn classify(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "negative",
        0             => "zero",
        1..=9         => "single digit positive",
        10..=99       => "double digit",
        _             => "large",
    }
}

fn main() {
    // Destructuring tuples
    let point = (3, -5);
    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("on x-axis at {}", x),
        (0, y) => println!("on y-axis at {}", y),
        (x, y) => println!("at ({}, {})", x, y),
    }

    // Guards in match
    let num = 7;
    match num {
        n if n < 0  => println!("negative: {}", n),
        n if n == 0 => println!("zero"),
        n if n % 2 == 0 => println!("positive even: {}", n),
        n           => println!("positive odd: {}", n),
    }

    // @ bindings
    match 15u32 {
        n @ 1..=12 => println!("month {}", n),
        n @ 13..=31 => println!("possible day {}", n),
        _ => println!("out of range"),
    }
}
```

### Destructuring Structs & Enums

```rust
struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 3, y: 7 };
    let Point { x, y } = p;         // destructure into x, y
    println!("x={} y={}", x, y);

    // Ignore fields with ..
    let Point { x, .. } = Point { x: 5, y: 9 };
    println!("x only: {}", x);
}
```

---

## 5. Tricky Questions

### Q1 — Struct Update Trap
```rust
struct Config {
    host:    String,
    port:    u16,
    debug:   bool,
}

fn main() {
    let base = Config {
        host:  String::from("localhost"),
        port:  8080,
        debug: false,
    };
    let prod = Config {
        debug: true,
        ..base       // base.host is MOVED here!
    };
    println!("{}", base.host);   // ❌ or ✅?
}
```
**Answer:** ❌ Compile error — `base.host` (String) is moved into `prod`. `base.port` and `base.debug` are Copy so still accessible, but `base.host` is gone.

---

### Q2 — Exhaustive Match
```rust
enum Direction { North, South, East, West }

fn describe(d: Direction) -> &'static str {
    match d {
        Direction::North => "up",
        Direction::South => "down",
        Direction::East  => "right",
        // West missing!
    }
}
```
**Answer:** ❌ Compile error — `match` must be exhaustive. Rust forces you to handle all variants (unlike Java switches).

---

### Q3 — What prints?
```rust
fn main() {
    let x: Option<i32> = Some(5);
    let y = match x {
        Some(n) if n > 10 => "big",
        Some(_)           => "small",
        None              => "nothing",
    };
    println!("{}", y);
}
```
**Answer:** `small` — `5` matches `Some(_)` because the guard `n > 10` is false.

---

### Q4 — Nested Option
```rust
fn main() {
    let opt: Option<Option<i32>> = Some(Some(42));
    if let Some(Some(n)) = opt {
        println!("{}", n);
    }
}
```
**Answer:** `42` — nested `if let` destructuring in one pattern. This is valid.

---

### Q5 — Enum Memory Size
```rust
enum Msg {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}
```
**Question:** Is `Msg::Quit` the same size in memory as `Msg::Write(...)`?  
**Answer:** No — but ALL variants share the same enum size (the size of the *largest* variant + discriminant). Rust allocates space for the biggest possible variant for any enum value.

---

### Q6 — if let vs match
```rust
// Are these equivalent?
let opt = Some(42);

// Version A
match opt {
    Some(v) => println!("{}", v),
    _       => {}
}

// Version B
if let Some(v) = opt {
    println!("{}", v);
}
```
**Answer:** ✅ Yes — `if let` is syntactic sugar for a single-arm match with a wildcard fallthrough.

---

## 6. Key Takeaways

| Feature | Rust | C++ / Java |
|---------|------|------------|
| Enum variants | Can carry any data | Just integer constants |
| Pattern matching | Exhaustive, compiler-enforced | Switch falls through by default |
| `Option<T>` | Replaces nullable references | Null pointer exists (danger) |
| Struct methods | In `impl` blocks | In class definitions |
| Struct update `..` | Moves non-Copy fields | No equivalent |
