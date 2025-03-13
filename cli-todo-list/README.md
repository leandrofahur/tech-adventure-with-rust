# 🟢 Beginner Project — CLI Todo Application

## ✅ Project Setup

**Step 1: Initialize the Rust Project**

```
cargo new cli-todo-list
cd cli-todo-list
```

**Step 2: Add Required Dependencies**

```
[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

Then build to install dependencies:

```
cargo build
```

## 📁 Project Structure

```
cli-todo-list/
├── Cargo.toml
├── tasks.json
└── src/
    ├── main.rs
    ├── task.rs
    └── utils.rs
```
