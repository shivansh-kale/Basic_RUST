# 🦀 Rust Cheat Sheet (Minimal Notes)

> A one-page reference covering the most important Rust concepts you'll use every day.

---

# 1. Variables

```rust
let x = 10;          // Immutable
let mut y = 20;      // Mutable

y += 5;
```

### Shadowing

```rust
let x = 5;
let x = x + 1;
```

---

# 2. Data Types

### Integers

```text
i8  i16  i32  i64  i128  isize
u8  u16  u32  u64  u128  usize
```

### Floating Point

```text
f32
f64
```

### Other Types

```text
bool
char
String
&str
```

---

# 3. Strings

### String Slice (Borrowed)

```rust
let s = "Hello";
```

### Owned String

```rust
let mut s = String::from("Hello");

s.push('!');
s.push_str(" Rust");
```

---

# 4. Printing

```rust
println!("Hello");

let x = 10;

println!("{}", x);

println!("{:?}", vec![1,2,3]);

println!("{:#?}", vec![1,2,3]);
```

---

# 5. Comments

```rust
// Single line comment

/*
Multi-line
comment
*/
```

---

# 6. Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(2, 3));
}
```

**Note:** The last line without a semicolon is automatically returned.

---

# 7. If / Else

```rust
if x > 5 {

} else if x == 5 {

} else {

}
```

---

# 8. Match

```rust
match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Other"),
}
```

---

# 9. Loops

### Infinite Loop

```rust
loop {

}
```

### While Loop

```rust
while x < 10 {

}
```

### For Loop

```rust
for i in 0..5 {

}
```

### Inclusive Range

```rust
0..=5
```

---

# 10. Arrays

```rust
let arr = [1, 2, 3];

println!("{}", arr[0]);
```

---

# 11. Tuples

```rust
let t = (10, "Rust", true);

println!("{}", t.0);
```

---

# 12. Vectors

```rust
let mut v = Vec::new();

v.push(10);
v.push(20);

println!("{:?}", v);
```

---

# 13. Ownership

Each value has **only one owner**.

```rust
let s1 = String::from("Rust");

let s2 = s1;

// s1 is no longer valid
```

---

# 14. Borrowing

### Immutable Borrow

```rust
let s = String::from("Rust");

let r = &s;
```

### Mutable Borrow

```rust
let mut s = String::from("Rust");

let r = &mut s;
```

### Rule

```text
Many immutable references

OR

One mutable reference

Never both at the same time.
```

---

# 15. References

```rust
fn print(s: &String) {

}

fn change(s: &mut String) {

}
```

---

# 16. Slices

### Array Slice

```rust
let arr = [1,2,3,4];

let part = &arr[1..3];
```

### String Slice

```rust
let s = "Hello";

let h = &s[0..2];
```

---

# 17. Struct

```rust
struct User {
    name: String,
    age: u32,
}

let user = User {
    name: String::from("John"),
    age: 20,
};
```

---

# 18. Methods

```rust
impl User {

    fn show(&self) {

    }

    fn birthday(&mut self) {

    }

}
```

---

# 19. Enum

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

### Enum With Data

```rust
enum Message {
    Text(String),
    Number(i32),
}
```

---

# 20. Option

Rust uses `Option` instead of `null`.

```rust
let x = Some(10);

let y = None;
```

Usage

```rust
match x {
    Some(v) => println!("{}", v),
    None => println!("None"),
}
```

---

# 21. Result

Used for error handling.

```rust
Result<T, E>
```

Example

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {

    if b == 0 {
        Err(String::from("Divide by zero"))
    } else {
        Ok(a / b)
    }

}
```

---

# 22. Error Propagation

```rust
use std::fs::File;

fn read() -> Result<(), std::io::Error> {

    let file = File::open("a.txt")?;

    Ok(())
}
```

`?` automatically returns the error if one occurs.

---

# 23. Pattern Matching

```rust
if let Some(x) = value {

    println!("{}", x);

}
```

---

# 24. Traits

Traits are similar to interfaces.

```rust
trait Animal {

    fn speak(&self);

}
```

Implementation

```rust
struct Dog;

impl Animal for Dog {

    fn speak(&self) {
        println!("Woof");
    }

}
```

---

# 25. Generics

```rust
fn largest<T: PartialOrd>(a: T, b: T) -> T {

    if a > b {
        a
    } else {
        b
    }

}
```

---

# 26. Lifetimes

Tell Rust how long references are valid.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {

    if x.len() > y.len() {
        x
    } else {
        y
    }

}
```

---

# 27. Closures

```rust
let add = |a, b| a + b;

println!("{}", add(2,3));
```

---

# 28. Iterators

```rust
let v = vec![1,2,3];

for x in v.iter() {
    println!("{}", x);
}
```

Map Example

```rust
let doubled = v.iter()
               .map(|x| x * 2)
               .collect::<Vec<_>>();
```

---

# 29. Modules

```rust
mod math;

use math::add;
```

---

# 30. Crates

```text
Binary Crate

Library Crate
```

Package manager:

```text
Cargo
```

---

# 31. Cargo Commands

```bash
cargo new project

cargo build

cargo run

cargo check

cargo test

cargo fmt

cargo clippy

cargo clean
```

---

# 32. Useful Macros

```rust
println!()

format!()

vec![]

panic!()

dbg!()

assert!()

assert_eq!()
```

---

# 33. Collections

```text
Vec

HashMap

HashSet

BTreeMap

BinaryHeap
```

---

# 34. Smart Pointers

```text
Box<T>

Rc<T>

Arc<T>

RefCell<T>

Mutex<T>
```

---

# 35. Concurrency

### Thread

```rust
std::thread
```

### Channel

```rust
std::sync::mpsc
```

### Mutex

```rust
Mutex<T>
```

### Shared Ownership

```rust
Arc<T>
```

---

# 36. Common Derives

```rust
#[derive(Debug)]

#[derive(Clone)]

#[derive(Copy)]

#[derive(Default)]

#[derive(PartialEq)]

#[derive(Eq)]
```

---

# 37. Common Traits

```text
Debug

Clone

Copy

Default

Display

PartialEq

Eq

Ord

Hash
```

---

# 38. Rust Project Structure

```text
project/

├── Cargo.toml
└── src/
    ├── main.rs
    ├── lib.rs
    └── mod.rs
```

---

# 39. Naming Convention

```text
Variables      → snake_case

Functions      → snake_case

Modules        → snake_case

Structs        → PascalCase

Enums          → PascalCase

Traits         → PascalCase

Constants      → UPPER_CASE
```

---

# 40. Rust Philosophy

- Memory Safety
- No Garbage Collector
- Ownership Model
- Zero-Cost Abstractions
- Fearless Concurrency
- Compile-Time Error Detection
- Explicit over Implicit

---

#  Rust Ownership Rules

1. Every value has exactly **one owner**.
2. When the owner goes out of scope, the value is dropped.
3. Moving transfers ownership.
4. Borrowing does **not** transfer ownership.
5. Multiple immutable references are allowed.
6. Only one mutable reference is allowed.
7. Mutable and immutable references cannot coexist.

---

#  Option vs Result

| Option | Result |
|---------|---------|
| Some(T) | Ok(T) |
| None | Err(E) |
| Used when value may not exist | Used when an operation may fail |

---

#  Common Collections

| Collection | Purpose |
|------------|---------|
| Vec | Dynamic Array |
| HashMap | Key-Value Store |
| HashSet | Unique Values |
| BTreeMap | Sorted Key-Value Store |
| BinaryHeap | Priority Queue |

---

#  Smart Pointers

| Pointer | Use |
|----------|-----|
| Box<T> | Heap Allocation |
| Rc<T> | Shared Ownership (Single Thread) |
| Arc<T> | Shared Ownership (Multi Thread) |
| RefCell<T> | Interior Mutability |
| Mutex<T> | Thread-Safe Mutability |

---

#  The 20% That Gives You 80% of Rust

Master these first:

1. Variables (`let`, `mut`)
2. Functions
3. Ownership
4. Borrowing (`&`, `&mut`)
5. References & Slices
6. Structs
7. Enums
8. Option
9. Result
10. Match
11. Traits
12. Generics
13. Iterators
14. Closures
15. Cargo

---

#  Recommended Learning Order

```
Variables
    ↓
Data Types
    ↓
Functions
    ↓
Control Flow
    ↓
Ownership
    ↓
Borrowing
    ↓
References
    ↓
Slices
    ↓
Structs
    ↓
Enums
    ↓
Option & Result
    ↓
Traits
    ↓
Generics
    ↓
Lifetimes
    ↓
Closures
    ↓
Iterators
    ↓
Modules
    ↓
Cargo
    ↓
Collections
    ↓
Smart Pointers
    ↓
Concurrency
```

---

#  Cargo Workflow

```text
cargo new my_project
        ↓
cargo build
        ↓
cargo run
        ↓
cargo check
        ↓
cargo fmt
        ↓
cargo clippy
        ↓
cargo test
```

---

#  Remember

> Rust is built around **Ownership + Borrowing + Lifetimes**.

Once you understand these three concepts, the rest of the language becomes much easier to learn.