
# Module 11 — Closures & Iterators

> Goal: Learn two of Rust's most expressive features. Although they may look advanced, you'll encounter them often in Rust libraries and Solana tooling.

---

# What is a Closure?

A closure is an anonymous function that can capture variables from its surrounding environment.

Regular function

```rust
fn add(a:i32,b:i32)->i32{
    a+b
}
```

Closure

```rust
let add = |a:i32,b:i32| a+b;
```

Use

```rust
println!("{}",add(2,3));
```

---

# Capturing Variables

```rust
let tax = 18;

let total = |price:i32| price + tax;
```

The closure automatically captures `tax`.

---

# Mutable Capture

```rust
let mut count = 0;

let mut increment = ||{

    count += 1;

};
```

Closures can borrow immutably, borrow mutably, or take ownership.

---

# move Closures

```rust
let text = String::from("Rust");

let print = move ||{

    println!("{}",text);

};
```

Ownership moves into the closure.

Useful with threads and async tasks.

---

# Iterators

Instead of manually indexing collections

```rust
for value in &numbers{

    println!("{}",value);

}
```

Rust internally uses iterators.

---

# Creating Iterators

```rust
let values = vec![1,2,3];

let iter = values.iter();
```

Common methods

- iter()
- iter_mut()
- into_iter()

---

# map()

Transform values.

```rust
let doubled:Vec<_> = values
    .iter()
    .map(|x| x*2)
    .collect();
```

---

# filter()

```rust
let even:Vec<_> = values
    .iter()
    .filter(|x| **x % 2 == 0)
    .collect();
```

---

# collect()

Converts an iterator into another collection.

```rust
let v:Vec<_> = iter.collect();
```

---

# Chaining

```rust
let result:Vec<_> = values
    .iter()
    .filter(|x| **x>2)
    .map(|x| x*10)
    .collect();
```

Very common Rust style.

---

# Lazy Evaluation

Iterators don't execute immediately.

Nothing happens until:

- collect()
- for loop
- sum()
- count()

This makes iterator chains efficient.

---

# Solana Connection

Closures are common in:

- tests
- SDKs
- helper libraries

Iterators are everywhere:

- account lists
- instruction arrays
- transaction processing
- filtering program accounts

---

# Complete Example

```rust
fn main(){

    let numbers = vec![1,2,3,4,5];

    let squared:Vec<_> = numbers
        .iter()
        .filter(|x| **x % 2 == 1)
        .map(|x| x*x)
        .collect();

    println!("{:?}",squared);

    let greeting = String::from("Solana");

    let print = ||{
        println!("{}",greeting);
    };

    print();

}
```

---

# Common Mistakes

- Confusing `iter()` with `into_iter()`.
- Forgetting iterator chains are lazy.
- Moving values accidentally with `into_iter()`.
- Overusing closures when a normal function is clearer.

---

# Best Practices

- Prefer iterators over manual indexing.
- Use closures for short, localized logic.
- Reach for `map`, `filter`, `fold`, and `collect` before writing complex loops.
- Learn to recognize iterator chains—they appear throughout modern Rust code.
