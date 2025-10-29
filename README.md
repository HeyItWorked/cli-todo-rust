# Rust CLI Todo App

A simple command-line todo application built with Rust. Learn Rust fundamentals through building a practical CLI tool with file persistence.

## Setup

```bash
# Clone or create the project
cd cli-todo-rust

# Add dependencies
cargo add clap --features derive
cargo add serde --features derive
cargo add serde_json
```

## Usage

All commands use **0-based indexing** (tasks are numbered 0, 1, 2, 3...).

```bash
# Add a task
cargo run -- add "Buy groceries"
cargo run -- add "Walk the dog"
cargo run -- add "Finish homework"

# List all tasks
cargo run -- list
# Output:
# 0: Buy groceries [ ]
# 1: Walk the dog [ ]
# 2: Finish homework [ ]

# Mark a task as complete
cargo run -- complete 0
# Output: Task Buy groceries is completed!

# List again to see completed status
cargo run -- list
# Output:
# 0: Buy groceries [x]
# 1: Walk the dog [ ]
# 2: Finish homework [ ]

# Remove a task by index
cargo run -- remove 1
# Output: Removed: Todo { description: "Walk the dog", completed: false }
```

## Features

- ✅ **Add todos** - Create new tasks with descriptions
- ✅ **Remove todos** - Delete tasks by index (0-based)
- ✅ **List todos** - Display all tasks with completion status
- ✅ **Complete todos** - Mark tasks as done without removing them
- ✅ **JSON persistence** - Data saved to `storage/todo-file.json`
- ✅ **Error handling** - Graceful handling of missing files and invalid indices
- ✅ **Visual indicators** - `[ ]` for incomplete, `[x]` for completed

## Project Structure

```
cli-todo-rust/
├── src/
│   └── main.rs          # Main application code
├── storage/
│   └── todo-file.json   # JSON file storage (created automatically)
├── Cargo.toml           # Project dependencies
├── README.md            # This file
└── LEARNING_NOTES.md    # Rust learning notes on error handling
```

## Build

```bash
# Development build
cargo build

# Optimized release build
cargo build --release

# Run the release binary directly
./target/release/cli-todo-rust list
```

## Architecture

```
┌─────────────────────────────────────┐
│  PROGRAM START                      │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  Parse CLI arguments                │
│  (clap with derive macros)          │
│  → Command: Add/Remove/List/Complete│
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  load_data()                        │
│  → Check if storage/todo-file.json  │
│    exists                           │
│  → If yes: deserialize JSON         │
│  → If no: create empty Vec + file   │
│  → Return Result<Vec<Todo>, Error>  │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  Match on command:                  │
│                                     │
│  Add → Push Todo to vec             │
│  Remove → vec.remove(index)         │
│  List → Print all with status       │
│  Complete → Set completed = true    │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  save_todos(&vec)                   │
│  → Serialize vec to pretty JSON     │
│  → Write to storage/todo-file.json  │
│  → Return Result<(), Error>         │
└─────────────────────────────────────┘
           ↓
┌─────────────────────────────────────┐
│  Print feedback to user             │
│  PROGRAM EXIT                       │
└─────────────────────────────────────┘
```

## Data Model

```rust
// Todo struct (data model)
struct Todo {
    description: String,
    completed: bool
}

// Commands enum (CLI interface)
enum Commands {
    Add { description: String },
    Remove { index: usize },
    List,
    Complete { index: usize }
}
```

## Storage Format

Todos are stored in `storage/todo-file.json` as a JSON array:

```json
[
  {
    "description": "Buy groceries",
    "completed": true
  },
  {
    "description": "Walk the dog",
    "completed": false
  }
]
```

## Learning Resources

- See `LEARNING_NOTES.md` for detailed Rust error handling concepts
- Covers: Result types, `?` operator, error propagation, unwrap variants

## Dependencies

- **clap 4.x** - Command-line argument parser with derive macros
- **serde 1.x** - Serialization/deserialization framework
- **serde_json 1.x** - JSON support for serde

## Recap

This project is a tutorial toy project demonstrating:
- Basic Rust syntax and ownership
- CLI argument parsing with clap
- File I/O and error handling
- JSON serialization with serde
- Pattern matching and enums
- Result types and error propagation