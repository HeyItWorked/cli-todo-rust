use clap::{Parser, Subcommand};
use std::fs;
use serde_json;
use serde::{Deserialize, Serialize}; 
use std::path::Path;

/// Data model representing a single todo item.
/// Derives Serialize/Deserialize for JSON persistence.
#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    description: String,
    completed: bool
}

/// Main CLI structure that holds subcommands.
/// The Parser derive macro enables automatic CLI argument parsing via clap.
#[derive(Parser)]
struct Cli {
    /// Available subcommands (Add, Remove, List, Complete).
    /// The command field is automatically populated by clap based on user input.
    #[command(subcommand)]
    command: Commands,
}

/// Enum defining all available CLI subcommands.
/// Each variant maps to a user-facing command (e.g., "todo add", "todo list").
#[derive(Subcommand)]
enum Commands {
    /// Add a new todo with the given description
    Add {
        description: String
    },
    /// Remove a todo by its index (0-based)
    Remove {
        index: usize
    },
    /// List all todos with their completion status
    List,
    /// Mark a todo as completed by its index (0-based)
    /// Note: usize is Rust's natural indexing type for arrays/vectors
    Complete {
        index: usize
    }
}

/// Load todos from JSON file, creating an empty file if none exists.
/// 
/// Returns a Result containing either:
/// - Ok(Vec<Todo>): Successfully loaded todos from file
/// - Err: File system or deserialization error
fn load_data() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let folder_name = "storage";
    let file_path = format!("{}/todo-file.json", folder_name);

    let list = if Path::new(&file_path).exists() {
        // File exists - read and deserialize
        let data = fs::read_to_string(&file_path)?;
        serde_json::from_str(&data)?
    } else {
        // File doesn't exist - create storage directory and empty JSON file
        fs::create_dir_all(folder_name)?;
        let empty = Vec::<Todo>::new();
        let json = serde_json::to_string(&empty)?;
        fs::write(&file_path, json)?;
        empty
    };

    Ok(list)
}

/// Save todos to JSON file with pretty formatting.
/// 
/// Uses serde_json::to_string_pretty for human-readable output.
/// Takes a reference to avoid taking ownership of the todo list.
fn save_todos(list: &Vec<Todo>) -> Result<(), Box<dyn std::error::Error>> {
    let folder_name = "storage";
    let file_path = format!("{}/todo-file.json", folder_name);

    // Serialize with indentation for readability
    let json = serde_json::to_string_pretty(list)?;
    fs::write(&file_path, json)?;
    Ok(())
}

fn main() {
    // Parse command-line arguments into Cli struct
    let cli: Cli = Cli::parse();
    
    // Load existing todos or start with empty list if file doesn't exist
    let mut list: Vec<Todo> = load_data().unwrap_or_default();

    // Execute the appropriate command based on user input
    match cli.command {
        Commands::Add { description } => {
            list.push(Todo { description, completed: false });
            let _ = save_todos(&list);
        }
        
        Commands::Remove { index } => {
            if index < list.len() {
                let removed = list.remove(index);
                println!("Removed: {:?}", removed);
                let _ = save_todos(&list);
            } else {
                eprintln!("Error: Task {} doesn't exist", index);
            }
        }
        
        Commands::List => {
            for (i, todo) in list.iter().enumerate() {
                let status = if todo.completed { "[x]" } else { "[ ]" };
                println!("{}: {} {}", i, todo.description, status);
            }
        }
        
        Commands::Complete { index } => {
            // get_mut() returns Option<&mut Todo> for safe mutable access
            if let Some(todo) = list.get_mut(index) {
                todo.completed = true;
                println!("Task '{}' marked as complete!", todo.description);
                let _ = save_todos(&list);
            } else {
                eprintln!("Error: Task {} doesn't exist", index);
            }
        }
    }
}
