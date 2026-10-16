# CLI File Search & Text Analyzer

A lightweight, efficient command-line text analyzer written in Rust. This utility scans a specified text file line-by-line, identifies lines matching a given search pattern, and displays matches along with line numbers and summary statistics.

---

## 🛠️ Project Structure

```text
file_analyzer/
├── Cargo.toml          # Project metadata and configuration
├── README.md            # Project documentation
├── test.txt            # Sample file for testing searches
└── src/
    ├── main.rs         # Application entry point & file processing logic
    ├── config.rs       # Command-line argument parsing and configuration
    └── error.rs        # Custom error types and conversion implementations
```

---

## 🚀 How to Run

1. Open your terminal in the project directory.
2. Build and run the application using `cargo run`, passing your search pattern and target file path as arguments:

```bash
cargo run -- "<pattern>" <file_path>
```

### Example:
```bash
cargo run -- "Rust" test.txt
```

---

## 💡 Rust Techniques Implemented

This project incorporates **3 core Rust techniques** from the course requirements:

### 1. `TryFrom` / `TryInto` Traits (`src/config.rs`)
The `Config` struct implements `TryFrom<Vec<String>>` to parse and validate command-line arguments safely. If fewer arguments than expected are provided, it cleanly returns a custom error rather than panicking.

```rust
impl TryFrom<Vec<String>> for Config {
    type Error = AppError;

    fn try_from(args: Vec<String>) -> Result<Self, Self::Error> {
        if args.len() < 3 {
            return Err(AppError::MissingArguments);
        }
        Ok(Config {
            pattern: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
```

### 2. Custom Error Handling & `Result` (`src/error.rs`)
An `AppError` enum is defined to handle both argument validation failures and file I/O errors cleanly. It implements `Display` for friendly console output and `From<std::io::Error>` to enable idiomatic error propagation using the `?` operator.

### 3. Closures & Higher-Order Functions (`src/main.rs`)
File content streaming and pattern matching are implemented using an iterator chain featuring closures and higher-order functions (`.map_while()`, `.enumerate()`, `.filter()`, and `.for_each()`):

```rust
reader
    .lines()
    .map_while(Result::ok)
    .enumerate()
    .filter(|(_, line)| line.contains(&config.pattern))
    .for_each(|(line_num, line)| {
        total_matches += 1;
        println!("Line {}: {}", line_num + 1, line.trim());
    });
```