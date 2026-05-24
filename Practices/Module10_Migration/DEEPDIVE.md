# 🦀 Module 10 — Migration from Python / Java / C++ (Deep Dive)

> **Interoperability, FFI boundaries, and porting strategies for legacy systems.**

---

## 1. FFI Boundaries & Rust's C ABI

To talk to other systems, Rust must expose functions that conform to the C Application Binary Interface (ABI), which is the standard lingua franca of computing.

### Exporting Rust Code to C / C++

```rust
// Exposing a Rust function to C
#[no_mangle] // Disable name mangling (keeps name readable in binary symbol table)
pub extern "C" fn calculate_hash(data: *const u8, len: usize) -> u32 {
    if data.is_null() {
        return 0;
    }
    // Safe conversion of raw pointer to Rust slice
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    
    // Perform computation safely in Rust...
    slice.iter().fold(5381, |acc, &x| acc.wrapping_mul(33).wrapping_add(x as u32))
}
```

### Importing C Functions into Rust
```rust
extern "C" {
    // Declaring external function from C library
    fn abs(input: i32) -> i32;
}

fn main() {
    // Calling foreign functions is ALWAYS unsafe
    unsafe {
        let result = abs(-42);
        println!("Absolute value from C: {}", result);
    }
}
```

---

## 2. Porting from C++ (System Architecture)

| C++ Feature | Rust Equivalent |
|-------------|-----------------|
| `std::unique_ptr<T>` | `Box<T>` |
| `std::shared_ptr<T>` | `Arc<T>` (or `Rc<T>` if single-threaded) |
| `std::vector<T>` | `Vec<T>` |
| RAII | `Drop` Trait |
| Templates | Generics + Trait Bounds |
| `std::move` | Native ownership transfer (default in Rust) |

---

## 3. Accelerating Python with Rust (PyO3)

`PyO3` allows you to write native Rust extension modules for Python or run Python code inside Rust.

### Native Rust Module for Python

```toml
# Cargo.toml
[lib]
name = "rust_core"
crate-type = ["cdylib"] # Compiled dynamic library

[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }
```

```rust
use pyo3::prelude::*;

/// Formats a greeting. Exposed to Python!
#[pyfunction]
fn greet(name: &str) -> PyResult<String> {
    Ok(format!("Hello from Rust, {}!", name))
}

/// Our Python module definition
#[pymodule]
fn rust_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    Ok(())
}
```

In Python:
```python
import rust_core
print(rust_core.greet("Kamal")) # "Hello from Rust, Kamal!"
```

---

## 4. Porting from Java (JNI Integration)

`jni` crate enables developers to call Rust code from the JVM and vice versa, avoiding the overhead of heavy microservices.

```rust
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

// Function mapping to a Java native method:
// Native definition: private native String stringMethod(String input);
#[no_mangle]
pub extern "system" fn Java_com_example_NativeBridge_stringMethod(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input_str: String = env.get_string(input).unwrap().into();
    let response = format!("Processed by Rust: {}", input_str);
    let output = env.new_string(response).unwrap();
    output.into_raw()
}
```

---

## 5. Tricky Questions

### Q1 — The GIL (Global Interpreter Lock) in PyO3
**Question:** Does running PyO3 functions automatically bypass Python's GIL?  
**Answer:** No. By default, PyO3 methods run on Python threads and keep the GIL locked. If you execute a CPU-heavy, long-running loop in Rust via Python FFI, it will block other Python threads.  
**Fix:** Release the GIL explicitly inside your Rust function so that python threads can execute in parallel while Rust is working:
```rust
Python::with_gil(|py| {
    py.allow_threads(|| {
        // Run massive computation without holding the Python GIL
    });
});
```

---

### Q2 — Memory Layout / Alignment Issues
**Question:** If you pass a C struct with varying field types to Rust FFI, what happens?  
**Answer:** It can cause memory corruption. Rust has no fixed memory layout guarantees by default (compiler reorders fields for optimization).  
**Fix:** Annotate your Rust structs with `#[repr(C)]` when passing them across FFI boundaries. This forces Rust to align memory layout precisely according to the platform's C standards.

---

### Q3 — FFI Reference Lifetime Trap
**Question:** What happens to the memory when passing a pointer from Java (JVM Heap) to Rust?  
**Answer:** Java Garbage Collector moves objects in memory. If a reference pointer to JVM memory is retained in Rust beyond the JNI function execution boundary, the garbage collector might move or deallocate it, causing a segmentation fault.  
**Fix:** Never store raw FFI pointers in global or static variables. Copy JVM/C++ memory into owned Rust structures (`String`, `Vec`) if you need it to outlive the function invocation.

---

## 6. Key Takeaways

* **Safety First**: Every FFI call in Rust is considered `unsafe`. Wrap raw FFI calls in safe, idiomatic Rust API wrappers.
* **Layout Rule**: Always use `#[repr(C)]` on structs shared across FFI.
* **FFI Types**: Use the `std::os::raw` module or `libc` crate for exact standard FFI integer mappings (`c_int`, `c_char`).
* **Tooling**: Use `bindgen` to automatically generate Rust binding interfaces from C/C++ headers, reducing human error.
