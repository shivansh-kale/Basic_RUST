
# Module 13 — Macros (Declarative & Procedural)

> Goal: Understand how Rust generates code automatically. This is extremely important because Solana's Anchor framework relies heavily on macros.

---

# What is a Macro?

A macro generates Rust code **before compilation**.

Think of it as:

```
Macro
   │
   ▼
Generated Rust Code
   │
   ▼
Compiler
```

Unlike functions, macros can work with Rust syntax itself.

---

# Why Macros?

Without macros:

- Lots of repetitive code
- Boilerplate
- Manual implementations

Macros automate this.

---

# Function vs Macro

Function

```rust
add(2,3);
```

Macro

```rust
println!("Hello");
```

Notice the `!`.

---

# Common Built-in Macros

```rust
println!()
eprintln!()
format!()
vec![]
assert!()
dbg!()
```

Example

```rust
let numbers = vec![1,2,3];
```

The `vec!` macro expands into code that builds a vector.

---

# Declarative Macros

Created using `macro_rules!`.

```rust
macro_rules! greet {

    () => {

        println!("Hello Rust");

    };

}
```

Use

```rust
greet!();
```

---

# Macros with Parameters

```rust
macro_rules! square {

    ($x:expr) => {

        $x * $x

    };

}

fn main() {

    println!("{}", square!(5));

}
```

---

# Derive Macros

```rust
#[derive(Debug, Clone)]
struct User {
    name:String,
}
```

The compiler automatically generates implementations.

Common derives:

- Debug
- Clone
- Copy
- Default
- PartialEq
- Eq

---

# Attribute Macros

These modify items such as functions and structs.

Example:

```rust
#[test]
fn my_test(){}
```

---

# Procedural Macros

These are more advanced.

They receive Rust syntax as input and generate new syntax.

Anchor uses them extensively.

---

# Anchor Macros

Examples:

```rust
#[program]

#[derive(Accounts)]

#[account]

#[error_code]
```

These macros generate hundreds of lines of Rust code for:

- Serialization
- Validation
- Account loading
- Security checks
- Error handling

---

# Complete Example

```rust
macro_rules! multiply {

    ($a:expr,$b:expr) => {

        $a * $b

    };

}

fn main(){

    let answer = multiply!(10,20);

    println!("{}",answer);

}
```

---

# Best Practices

- Use functions unless syntax transformation is required.
- Don't overuse custom macros.
- Learn to read macro-generated APIs.
- Understand Anchor macros before writing Solana programs—they hide much of the boilerplate.
