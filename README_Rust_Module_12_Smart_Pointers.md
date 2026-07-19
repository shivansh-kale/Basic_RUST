
# Module 12 — Smart Pointers (Box, Rc, Arc, RefCell)

> Goal: Understand why Rust has smart pointers and learn only the parts most useful for reading Rust libraries and Solana-related code.

---

# Why Smart Pointers?

A normal reference (`&T`) only **borrows** data.

A smart pointer is a type that **owns data** while providing additional capabilities.

Examples:

- Heap allocation (`Box`)
- Shared ownership (`Rc`)
- Thread-safe shared ownership (`Arc`)
- Interior mutability (`RefCell`)

---

# Box<T>

`Box<T>` stores a value on the heap.

```rust
let x = Box::new(10);
```

Memory:

```
Stack
+---------+
| Box ptr | -----> Heap
+---------+        +----+
                   | 10 |
                   +----+
```

Use cases:
- Large values
- Recursive data structures
- Trait objects (`Box<dyn Trait>`)

---

# Recursive Types

This won't compile:

```rust
enum List {
    Node(i32, List),
    Empty,
}
```

The compiler can't determine the size.

Correct:

```rust
enum List {
    Node(i32, Box<List>),
    Empty,
}
```

`Box` gives the enum a fixed size.

---

# Rc<T>

`Rc` = **Reference Counted**.

Allows multiple owners in a **single thread**.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("Rust"));
let b = Rc::clone(&a);
```

Both `a` and `b` point to the same data.

Reference count increases.

---

# Arc<T>

`Arc` = **Atomic Reference Counted**.

Like `Rc`, but thread-safe.

```rust
use std::sync::Arc;

let data = Arc::new(vec![1,2,3]);
```

Used heavily with threads.

---

# RefCell<T>

Normally:

- One mutable borrow
- OR many immutable borrows

`RefCell` moves borrow checking from compile time to runtime.

```rust
use std::cell::RefCell;

let value = RefCell::new(5);

*value.borrow_mut() += 1;
```

If borrowing rules are violated, the program panics.

---

# Rc<RefCell<T>>

Very common combination.

```
Rc
 │
 ▼
RefCell
 │
 ▼
Data
```

Provides:

- Shared ownership
- Mutable access

---

# When Should You Use Them?

| Type | Use |
|------|-----|
| Box | Heap allocation |
| Rc | Shared ownership (single thread) |
| Arc | Shared ownership (multiple threads) |
| RefCell | Runtime mutability |

---

# Solana Connection

In normal Solana program development with Anchor:

- `Box<Account<'info, T>>` may be used for large accounts.
- `Rc` and `Arc` are uncommon inside on-chain programs.
- `RefCell` appears in some SDKs and internal Rust libraries.

---

# Complete Example

```rust
use std::rc::Rc;

fn main() {

    let message = Rc::new(String::from("Hello"));

    let a = Rc::clone(&message);
    let b = Rc::clone(&message);

    println!("{}", a);
    println!("{}", b);

    println!("Owners = {}", Rc::strong_count(&message));

    let number = Box::new(100);

    println!("{}", number);
}
```

---

# Best Practices

- Prefer normal ownership first.
- Use `Box` when recursive types or heap allocation are required.
- Use `Rc` only for single-threaded shared ownership.
- Use `Arc` for multi-threading.
- Avoid `RefCell` unless compile-time borrowing is too restrictive.
