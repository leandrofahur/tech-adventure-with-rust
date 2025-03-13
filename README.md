# 🦀 A Roadmap of my Tech Adventure With Rust 🚀

Welcome to this structured Rust learning path, designed to progressively level up your Rust skills from beginner to advanced through practical project implementation.

The projects are carefully chosen to ensure incremental complexity, leveraging fundamental software design principles: [**YAGNI**](https://www.geeksforgeeks.org/what-is-yagni-principle-you-arent-gonna-need-it/), [**KISS**](https://www.geeksforgeeks.org/kiss-principle-in-software-development/), and [**DRY**](https://www.geeksforgeeks.org/dont-repeat-yourselfdry-in-software-development/).

---

## 📌 Projects Overview

### 🟢 Beginner Project — CLI Todo Application

**Objective:**
Build a simple command-line Todo application, storing data locally.

**Learning Outcomes:**

- Rust fundamentals: Ownership, Borrowing, Types
- File operations and JSON serialization (`serde`)
- CLI argument parsing (`clap`)
- Basic error handling

**Features:**

- Add, update, list, remove, complete tasks
- Persist tasks in local storage

**Design Patterns:**

- Facade, Builder

---

### 🟡 Intermediate Project — REST API with Actix-web & Diesel

**Objective:**
Create a RESTful CRUD API to manage notes with SQLite/PostgreSQL.

**Learning Outcomes:**

- Web frameworks (Actix-web/Axum)
- ORM integration (Diesel)
- JSON serialization/deserialization
- Integration testing and error handling

**Features:**

- CRUD endpoints, filtering, pagination
- Robust error handling
- Integration tests for endpoints

**Design Patterns:**

- Repository, DTO

---

### 🔴 Advanced Project — Real-time Chat Application

**Objective:**
Develop a real-time scalable chat backend using WebSockets and Tokio.

**Learning Outcomes:**

- Async programming (`tokio`)
- WebSocket protocols (`tokio-tungstenite`)
- Concurrent programming
- Advanced traits, lifetimes, state management

**Features:**

- Multiple chat rooms and sessions
- Real-time messaging with broadcasting
- Scalability for multiple concurrent connections

**Design Patterns:**

- Observer, State

---

## 🗂️ Project Directory Template

Follow this template for organized and clean projects:

```
project/
├── Cargo.toml
├── src/
│   ├── errors.rs
│   ├── utils/
│   ├── models/
│   ├── handlers/
│   └── main.rs
├── tests/
└── README.md
```

---

## 📖 Recommended Resources

- [Official Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Awesome Rust](https://github.com/rust-unofficial/awesome-rust)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)
