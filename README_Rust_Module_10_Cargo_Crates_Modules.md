
# Module 10 — Cargo, Crates, Modules & Packages

> Goal: Understand how Rust projects are organized. This is essential before working with Anchor or any real Solana project.

---

# Cargo

Cargo is Rust's official build system and package manager.

It helps you:

- Create projects
- Compile code
- Manage dependencies
- Run tests
- Build release binaries
- Generate documentation

Common commands:

```bash
cargo new my_project
cargo run
cargo build
cargo build --release
cargo check
cargo test
cargo doc --open
cargo fmt
cargo clippy
```

---

# Project Structure

```text
my_project/
│
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   └── lib.rs
└── target/
```

### Cargo.toml

Project metadata.

```toml
[package]
name = "wallet"
version = "0.1.0"
edition = "2024"

[dependencies]
serde = "1.0"
```

### Cargo.lock

Stores exact dependency versions.

Applications usually commit this file.
Libraries often don't.

---

# Crates

A **crate** is the smallest compilation unit.

Two types:

- Binary crate (`main.rs`)
- Library crate (`lib.rs`)

Binary crates produce executables.

Library crates expose reusable functionality.

---

# Packages

A package contains one or more crates.

```
Package
│
├── Binary Crate
└── Library Crate
```

---

# Modules

Modules organize code.

```rust
mod math;

fn main() {
    println!("{}", math::add(2,3));
}
```

math.rs

```rust
pub fn add(a:i32,b:i32)->i32{
    a+b
}
```

---

# Visibility

Everything is private by default.

```rust
pub fn hello(){}

pub struct User{
    pub name:String,
}
```

Use `pub` only where necessary.

---

# use Keyword

Instead of

```rust
std::collections::HashMap
```

write

```rust
use std::collections::HashMap;
```

---

# External Crates

```toml
[dependencies]
rand="0.9"
```

```rust
use rand::Rng;
```

Cargo downloads dependencies automatically.

---

# Workspaces

Useful for monorepos.

```
workspace
│
├── app
├── sdk
└── shared
```

Anchor projects internally use Cargo workspaces.

---

# Solana Connection

Typical Anchor project

```text
programs/
tests/
migrations/
Anchor.toml
Cargo.toml
```

Each Solana program is a Rust crate.

---

# Complete Example

```rust
// src/math.rs

pub fn square(x:i32)->i32{
    x*x
}
```

```rust
// src/main.rs

mod math;

fn main(){

    println!("{}",math::square(10));

}
```

---

# Best Practices

- Keep modules small.
- Export only required APIs.
- Use workspaces for large projects.
- Run `cargo fmt` and `cargo clippy` regularly.
