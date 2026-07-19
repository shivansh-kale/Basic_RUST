
# Module 6 — Traits (Deep Dive)

> Goal: Understand how Rust defines shared behavior without classical inheritance. Traits are similar to interfaces in Java/C#, but are more powerful and central to Rust.

---

# Why Traits?

Imagine we have:

```rust
struct Dog;
struct Cat;
struct Bird;
```

Each can make a sound.

Instead of duplicating methods, define a common behavior.

```rust
trait Animal {
    fn sound(&self);
}
```

A **trait** describes **what a type can do**, not what it is.

---

# Implementing a Trait

```rust
struct Dog;

impl Animal for Dog {
    fn sound(&self){
        println!("Woof");
    }
}
```

Now any `Dog` has the behavior defined by `Animal`.

---

# Multiple Implementations

```rust
struct Cat;

impl Animal for Cat{
    fn sound(&self){
        println!("Meow");
    }
}
```

The same trait can be implemented by many types.

---

# Default Implementations

```rust
trait Animal{

    fn sleep(&self){
        println!("Sleeping...");
    }

    fn sound(&self);
}
```

`sleep()` is inherited automatically unless overridden.

---

# Trait Bounds

Suppose we want a function that accepts **anything capable of speaking**.

```rust
fn make_sound<T: Animal>(animal:&T){
    animal.sound();
}
```

Equivalent syntax

```rust
fn make_sound(animal:&impl Animal){
    animal.sound();
}
```

Use `impl Trait` for simple APIs.

---

# Multiple Trait Bounds

```rust
fn display<T: Animal + Clone>(value:T){

}
```

or

```rust
fn display<T>(value:T)
where
    T: Animal + Clone
{

}
```

`where` improves readability.

---

# Associated Types

Traits can define placeholder types.

```rust
trait Iterator{

    type Item;

    fn next(&mut self)->Option<Self::Item>;

}
```

This avoids passing generic parameters repeatedly.

---

# Static Dispatch

Generic functions:

```rust
fn run<T: Animal>(value:T){}
```

Compiler generates optimized code for every concrete type.

Pros:
- Fast
- No runtime lookup

Cons:
- Larger binary

---

# Dynamic Dispatch

```rust
fn run(value:&dyn Animal){
    value.sound();
}
```

Uses a vtable (virtual table).

Pros:
- One implementation
- Flexible

Cons:
- Tiny runtime cost

---

# Derivable Traits

Rust provides many useful traits.

```rust
#[derive(Debug,Clone,PartialEq)]
struct User{
    name:String
}
```

Common derives:

- Debug
- Clone
- Copy
- Default
- PartialEq
- Eq
- Hash

---

# Traits in the Standard Library

Examples:

- Display
- Debug
- Clone
- Copy
- Iterator
- From
- Into
- Default

Many methods only exist because these traits are implemented.

---

# Solana Connection

Traits are everywhere.

Examples:

- AnchorSerialize
- AnchorDeserialize
- AccountSerialize
- AccountDeserialize
- BorshSerialize
- BorshDeserialize

When Anchor loads an account, these traits perform serialization automatically.

---

# Complete Example

```rust
trait Shape{

    fn area(&self)->f64;

    fn describe(&self){
        println!("Calculating area...");
    }

}

struct Circle{

    radius:f64,

}

impl Shape for Circle{

    fn area(&self)->f64{

        3.14159*self.radius*self.radius

    }

}

fn print_area(shape:&impl Shape){

    shape.describe();

    println!("{}",shape.area());

}

fn main(){

    let circle=Circle{radius:5.0};

    print_area(&circle);

}
```

---

# Best Practices

- Use traits to define behavior.
- Prefer `impl Trait` for simple function parameters.
- Use trait bounds for generic algorithms.
- Derive common traits whenever appropriate.
- Think of traits as capabilities rather than inheritance.
