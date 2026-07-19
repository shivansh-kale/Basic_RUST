
# Module 2 — Ownership, Borrowing & References (Deep Dive)

> **Goal:** Understand the single most important concept in Rust. If you understand ownership, you understand why Rust is memory-safe without a garbage collector.

---

# Why does Rust even have Ownership?

Most programming languages solve memory management in one of three ways.

| Language | Strategy | Drawback |
|-----------|----------|----------|
| C | Manual (`malloc/free`) | Memory leaks, double free, dangling pointers |
| Java / Python | Garbage Collector | Runtime overhead, unpredictable pauses |
| Rust | Ownership | Compile-time restrictions instead of runtime cost |

Rust asks a simple question:

> **Who owns this value?**

Only the owner is responsible for cleaning up that memory.

---

# Stack vs Heap

Understanding ownership starts with memory.

## Stack

- Fixed-size values
- Extremely fast
- Automatically cleaned up
- LIFO (Last In First Out)

Examples:

```rust
let x = 10;
let y = true;
```

## Heap

Used for data whose size is unknown or can change.

```rust
let name = String::from("Rust");
```

What happens?

```
Stack
+--------------------+
| name               | --------+
+--------------------+         |
                               v
Heap
+------------------------------+
| R | u | s | t |
+------------------------------+
```

The stack stores metadata (pointer, length, capacity).
The actual characters live on the heap.

---

# Ownership Rules

Rust has only **three rules**.

## Rule 1

Every value has exactly **one owner**.

```rust
let s = String::from("Hello");
```

Owner = `s`

---

## Rule 2

A value can have only one owner at a time.

When ownership changes:

```rust
let s1 = String::from("Hello");

let s2 = s1;
```

Now

```
Before

s1 -----> Heap

After

s2 -----> Heap
s1  X
```

`s1` is invalid.

Why?

Because if both tried to free the same memory:

```
free(memory)
free(memory)
```

This is a **double free** bug.

Rust prevents it during compilation.

---

# Move Semantics

Unlike Python or Java, assignment for heap values is usually a **move**.

```rust
let name = String::from("Rust");
let another = name;
```

Ownership moves.

Trying

```rust
println!("{}", name);
```

produces

```
borrow of moved value
```

The compiler protects you before execution.

---

# Copy Types

Not every assignment moves ownership.

Primitive types implement `Copy`.

```rust
let x = 10;
let y = x;

println!("{}", x);
println!("{}", y);
```

Both work.

Why?

Because copying 4 bytes is cheaper than transferring ownership.

Common Copy types:

- integers
- floats
- bool
- char
- tuples of Copy values

---

# Clone

Need two independent heap values?

Use `clone()`.

```rust
let a = String::from("Rust");
let b = a.clone();
```

Now

```
a ---> Heap #1

b ---> Heap #2
```

Different memory.

Avoid unnecessary cloning because it allocates memory.

---

# Scope and Drop

```rust
{
    let s = String::from("Hello");
}
```

When scope ends

```
drop(s)
```

Memory is released automatically.

You never call `free()`.

Rust inserts cleanup code.

---

# Borrowing

Sometimes you don't want ownership.

You only want temporary access.

Example:

```rust
fn print_name(name:&String){
    println!("{}",name);
}
```

Calling

```rust
let s = String::from("Rust");

print_name(&s);

println!("{}",s);
```

works because ownership never changed.

Think of borrowing like borrowing a library book.

You read it.

You don't own it.

---

# Immutable References

```rust
let s = String::from("Rust");

let r1 = &s;
let r2 = &s;
let r3 = &s;
```

Allowed.

Many readers.

Zero writers.

---

# Mutable References

```rust
let mut s = String::from("Rust");

let r = &mut s;

r.push_str(" Language");
```

Allowed because only one mutable reference exists.

Rule:

One writer OR many readers.

Never both.

---

# Why?

Imagine

```
Reader ----\
            ---> Memory
Writer ----/
```

Writer changes memory while reader is reading.

Race condition.

Rust prevents this.

---

# Illegal Example

```rust
let mut s = String::from("Rust");

let r1 = &s;
let r2 = &mut s;
```

Compiler error.

Cannot have immutable and mutable references simultaneously.

---

# Dangling References

Bad in C

```c
char* func(){
    char name[]="Rust";
    return name;
}
```

Memory disappears.

Pointer still exists.

Danger.

Rust forbids it.

```rust
fn dangle()->&String{
    let s = String::from("Rust");
    &s
}
```

Compiler rejects it because `s` dies at the end of the function.

---

# References vs Ownership

| Operation | Ownership transferred? |
|------------|------------------------|
| `let b=a;` (`String`) | Yes |
| `let b=&a;` | No |
| `clone()` | No (creates new copy) |
| `Copy` types | No |

---

# Ownership in Functions

Passing ownership

```rust
fn consume(text:String){
    println!("{}",text);
}

fn main(){
    let s=String::from("Hello");
    consume(s);

    // s is invalid here
}
```

Borrowing

```rust
fn print(text:&String){
    println!("{}",text);
}

fn main(){

    let s=String::from("Hello");

    print(&s);

    println!("{}",s);
}
```

Returning ownership

```rust
fn create()->String{

    String::from("Rust")
}
```

Ownership moves to the caller.

---

# Solana Connection

Ownership appears everywhere in Solana programs.

Examples:

- Accounts are usually borrowed.
- Account data is mutably borrowed before modification.
- Program instructions pass references instead of copying accounts.
- Anchor heavily uses borrowing (`Account<'info, T>`).

If ownership feels confusing now, don't worry—understanding this module will make Anchor's syntax much easier later.

---

# Complete Example

```rust
// ===============================
// Ownership & Borrowing Demo
// ===============================

fn print_message(msg: &String) {
    // Borrow only.
    println!("Message: {}", msg);
}

fn append_world(msg: &mut String) {
    // Mutable borrow.
    msg.push_str(" World");
}

fn main() {

    // Heap allocated String.
    let mut greeting = String::from("Hello");

    // Immutable borrow.
    print_message(&greeting);

    // greeting is still valid because ownership never moved.
    println!("Still owned by main: {}", greeting);

    // Mutable borrow.
    append_world(&mut greeting);

    println!("Updated: {}", greeting);

    // Clone creates a completely new String.
    let greeting2 = greeting.clone();

    println!("{}", greeting);
    println!("{}", greeting2);

    // Copy type example.
    let x = 10;
    let y = x;

    println!("x = {}", x);
    println!("y = {}", y);
}
```

---

# Common Beginner Mistakes

1. Assuming assignment copies a `String`.
2. Using `clone()` everywhere.
3. Returning references to local variables.
4. Mixing mutable and immutable references.
5. Thinking `&String` owns the data.

---

# Mental Model

Imagine ownership as a car.

- Owner → Car registration.
- Borrower → Drives the car temporarily.
- Clone → Buys another identical car.
- Move → Transfers ownership papers.
- Drop → Car is scrapped when owner leaves scope.

Keep this analogy in mind while learning Solana, because accounts and account data follow very similar ownership and borrowing patterns.
