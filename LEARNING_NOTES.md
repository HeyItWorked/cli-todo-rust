# Rust Learning Notes

## Project Setup and Structure

### Q: What is the usual dev way to set up a Rust project folder?

**Answer:**
The standard way is using Cargo, Rust's package manager:

```bash
cargo new project-name        # Creates a binary (executable) project
cargo new project-name --lib  # Creates a library project
cargo init                    # Converts existing directory to Rust project
```

This automatically initializes:
- Git repository (with .gitignore)
- Cargo.toml (project manifest)
- src/ directory with main.rs (binary) or lib.rs (library)

### Q: What is the checklist of one template Rust folder?

**Answer:**
Standard Rust project structure created by `cargo new`:

```
project-name/
├── Cargo.toml              # Project manifest (dependencies, metadata)
├── Cargo.lock              # Dependency lock file (auto-generated)
├── .gitignore              # Git ignore file (includes target/)
├── src/                    # Source code directory
│   └── main.rs             # Entry point for binary projects
│       OR
│   └── lib.rs              # Entry point for library projects
└── target/                 # Build artifacts (auto-generated, gitignored)
    ├── debug/              # Debug builds
    └── release/            # Release builds
```

Optional but common additions:
- tests/        # Integration tests
- benches/      # Benchmarks
- examples/     # Example programs
- README.md     # Documentation
- LICENSE       # License file

### Q: What is bare minimum for any `cargo new [project-name]` to run?

**Answer:**
Bare minimum files needed to compile and run:

✓ Cargo.toml with [package] section:
```toml
[package]
name = "project-name"
version = "0.1.0"
edition = "2021"
```

✓ src/main.rs (for binary) with main function:
```rust
fn main() {
    println!("Hello, world!");
}
```

OR src/lib.rs (for library) with any valid Rust code

Commands to verify:
- `cargo check`    # Check if code compiles (fast)
- `cargo build`    # Compile the project
- `cargo run`      # Build and run (for binaries)
- `cargo test`     # Run tests

Note: Cargo.lock and target/ are auto-generated on first build

---

## Result Type and Error Handling

### What is Result?

`Result` is an enum with two variants - it returns **either** success **or** failure, not both:

```rust
enum Result<T, E> {
    Ok(T),      // Success - contains your data
    Err(E),     // Failure - contains error info
}
```

Think of it like a box that can hold one of two things:

```
┌─────────────────────┐
│  Result             │
│  ┌───────────────┐  │
│  │ Ok(Vec<Todo>) │  │  ← Contains your data
│  └───────────────┘  │
└─────────────────────┘

OR

┌─────────────────────┐
│  Result             │
│  ┌───────────────┐  │
│  │ Err("error")  │  │  ← Contains error message
│  └───────────────┘  │
└─────────────────────┘
```

### Evolution of Error Handling in Rust

```rust
// Ancient Rust (pre-1.0): try! macro
let data = try!(fs::read_to_string(&path));

// Classic Rust: manual match (still valid and common in older code)
let data = match fs::read_to_string(&path) {
    Ok(d) => d,
    Err(e) => return Err(e.into()),
};

// Modern Rust: ? operator (since Rust 1.22, 2017)
let data = fs::read_to_string(&path)?;
```

**All three do the exact same thing** - `?` is just syntactic sugar for the match pattern.

### The `?` Operator Explained

The `?` operator is shorthand for error propagation:

```rust
// This:
let data = fs::read_to_string(&path)?;

// Expands to this:
let data = match fs::read_to_string(&path) {
    Ok(value) => value,              // Extract value, continue execution
    Err(e) => return Err(e.into()),  // Return error to caller immediately
};
```

**Key Point:** `?` doesn't crash - it returns the error to the calling function.

### Extracting Data from Result

```rust
fn load_todos() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(&file_path)?;  // Could fail
    let list = serde_json::from_str(&data)?;     // Could fail
    Ok(list)  // Success - wrap Vec in Ok
}

// In main - various ways to handle the Result:

// Option 1: Unwrap (panics/crashes if Err)
let list: Vec<Todo> = load_todos().unwrap();

// Option 2: Match (handle both cases explicitly)
let list = match load_todos() {
    Ok(todos) => todos,
    Err(e) => {
        eprintln!("Error loading: {}", e);
        std::process::exit(1);
    }
};

// Option 3: Use ? in main (requires main to return Result)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let list = load_todos()?;  // If Err, main returns early
    Ok(())
}

// Option 4: Provide fallback value
let list = load_todos().unwrap_or_default();  // Empty Vec if error

// Option 5: Custom fallback logic
let list = load_todos().unwrap_or_else(|e| {
    eprintln!("Error: {}", e);
    Vec::new()
});
```

### Error Handling Method Comparison

| Method | What happens on error | When to use |
|--------|----------------------|-------------|
| `?` | Returns Err to caller (doesn't crash) | Inside functions that return Result |
| `.unwrap()` | 💥 **Crashes program** (panics) | Quick prototypes, when you're 100% sure it won't fail |
| `.unwrap_or(default)` | Uses fallback value | When you have a sensible default |
| `.unwrap_or_else(\|e\| { ... })` | Runs closure to compute fallback | When you need custom error handling logic |
| `.unwrap_or_default()` | Uses type's default value | When Default trait is implemented |
| `match` | You decide what happens | When you need full control over error handling |

### Why Separate Error-Prone Code into Helper Functions?

**Problem:** The `?` operator requires the function to return `Result<T, E>`

**Solution:** Move error-prone code to helper functions that return `Result`:

```rust
// Helper functions handle errors
fn load_todos() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("file.json")?;  // ✅ ? works here
    let list = serde_json::from_str(&data)?;
    Ok(list)  // Return success
}

fn save_todos(list: &Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(list)?;  // ✅ ? works here
    fs::write("file.json", json)?;
    Ok(())  // Return success (unit type wrapped in Ok)
}

// Main stays simple
fn main() {
    let cli = Cli::parse();
    let mut list = load_todos().unwrap();

    match cli.command {
        Add { description } => {
            list.push(Todo { description, completed: false });
            save_todos(&list).unwrap();
        }
        // ...
    }
}
```

### Best Practice for This Todo App

For a CLI app, the cleanest approach is to let main return Result:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut list = load_todos()?;  // Propagate errors

    match cli.command {
        Add { description } => {
            list.push(Todo { description, completed: false });
            save_todos(&list)?;  // Propagate errors
        }
        Remove { index } => {
            if index < list.len() {
                list.remove(index);
                save_todos(&list)?;
            }
        }
        List => {
            for (i, todo) in list.iter().enumerate() {
                println!("{}: {} [{}]", i, todo.description,
                    if todo.completed { "✓" } else { " " });
            }
        }
        Complete { index } => {
            if index < list.len() {
                list[index].completed = true;
                save_todos(&list)?;
            }
        }
    }

    Ok(())
}
```

This way, any error automatically exits with a clear error message instead of a panic.

---

## Key Takeaways

1. **Result doesn't return both data and error** - it's an enum that's either `Ok(data)` or `Err(error)`
2. **`?` is modern syntax** for error propagation (shorthand for match + return)
3. **`?` returns errors** to the caller, it doesn't crash
4. **`.unwrap()` crashes** the program if there's an error (use sparingly!)
5. **Helper functions with Result** keep main clean and enable use of `?`
6. **You still see lots of `match`** in older code - it's the underlying mechanism
7. **Use Cargo** for all project management - it handles dependencies, builds, and testing
8. **Follow standard project structure** - makes your code familiar to other Rust developers
