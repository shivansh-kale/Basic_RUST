
# Module 7 — Generics (Deep Dive)

> Goal: Write reusable code that works with many data types while keeping compile-time performance.

---

# Why Generics?

Without generics

```rust
fn add_i32(a:i32,b:i32)->i32{}
fn add_f64(a:f64,b:f64)->f64{}
```

Duplicate logic.

Generics allow one implementation.

---

# Generic Function

```rust
fn identity<T>(value:T)->T{
    value
}
```

`T` is a placeholder type.

---

# Generic Struct

```rust
struct Point<T>{

    x:T,
    y:T,

}
```

Usage

```rust
let p = Point{x:10,y:20};

let q = Point{x:2.5,y:8.1};
```

---

# Multiple Generic Types

```rust
struct Pair<T,U>{

    first:T,
    second:U,

}
```

---

# Generic Enums

`Option` is generic.

```rust
Option<i32>

Option<String>

Option<User>
```

Same enum.

Different contained type.

---

# Trait Bounds

Sometimes generic code needs specific abilities.

```rust
fn print<T:std::fmt::Debug>(value:T){

    println!("{:?}",value);

}
```

Only types implementing `Debug` are accepted.

---

# where Clause

Cleaner for many bounds.

```rust
fn process<T,U>(a:T,b:U)
where
    T:Clone,
    U:std::fmt::Debug
{

}
```

---

# Monomorphization

Rust does **not** use runtime generics.

Instead, the compiler generates specialized versions.

Example

```rust
identity(10)

identity("Rust")
```

Compiler produces

```
identity_i32()

identity_str()
```

Advantages

- Zero runtime overhead
- Highly optimized
- Type safety

Tradeoff

- Larger executable

---

# Generic Methods

```rust
struct BoxValue<T>{

    value:T,

}

impl<T> BoxValue<T>{

    fn get(&self)->&T{

        &self.value

    }

}
```

---

# Combining Traits and Generics

```rust
trait Summary{

    fn summarize(&self);

}

fn notify<T:Summary>(item:&T){

    item.summarize();

}
```

This is extremely common in Rust.

---

# Solana Connection

Generics appear frequently.

Examples

```rust
Account<'info,T>

Context<T>

Program<T>

Signer<'info>
```

Anchor uses generics extensively to provide compile-time guarantees.

---

# Complete Example

```rust
use std::fmt::Debug;

#[derive(Debug)]

struct Wallet{

    balance:u64,

}

fn show<T:Debug>(item:T){

    println!("{:?}",item);

}

struct Holder<T>{

    value:T,

}

impl<T> Holder<T>{

    fn get(&self)->&T{

        &self.value

    }

}

fn main(){

    let wallet=Wallet{balance:500};

    show(wallet);

    let number=Holder{value:100};

    println!("{}",number.get());

}
```

---

# Common Mistakes

- Thinking generics are runtime polymorphism.
- Forgetting trait bounds.
- Overusing generic parameters.
- Confusing generics with traits.

---

# Best Practices

- Start with concrete types.
- Generalize only when needed.
- Prefer trait bounds over code duplication.
- Use `where` clauses for readability.
- Remember that Rust generics are compiled into optimized concrete code.
