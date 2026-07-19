# 🦀 Module 1 — Rust Basics

> **Goal:** Learn only the Rust fundamentals required before starting Solana development.
>
> **Estimated Time:** 2–3 Hours
>
> You do **NOT** need to master Rust. This module only covers the concepts you'll repeatedly use while building Solana programs.

---

# 📚 Table of Contents

- Variables
- Mutable Variables (`mut`)
- Constants (`const`)
- Primitive Data Types
- Strings
- String Formatting
- Functions
- Comments

---

# 1️⃣ Variables

## 📖 Theory

A **variable** is simply a name that stores a value.

Unlike Python or JavaScript, **variables in Rust are immutable by default.**

That means once a value is assigned, it **cannot be changed** unless you explicitly allow it.

Think of it as writing with a **pen** instead of a pencil.

```
x = 10
```

Once written,

```
x = 20
```

is not allowed.

Rust does this to prevent accidental bugs.

---

### Syntax

```rust
let variable_name = value;
```

Example

```rust
let age = 20;
```

Here,

- `let` creates a variable.
- `age` is the variable name.
- `20` is the stored value.

---

### Why Immutable by Default?

Rust focuses heavily on **memory safety**.

If values cannot change unexpectedly,

- bugs decrease
- code becomes predictable
- compiler can optimize better

---

# 2️⃣ Mutable Variables (`mut`)

## 📖 Theory

Sometimes we **do** want to change a variable.

Examples

- Bank balance
- Score
- Counter
- User age
- Number of transactions

For this, Rust provides the `mut` keyword.

`mut` stands for **mutable**.

A mutable variable can change after it is created.

---

### Syntax

```rust
let mut variable = value;
```

Example

```rust
let mut score = 0;
```

Now,

```rust
score = 10;
```

is perfectly valid.

---

### When to Use `mut`

✅ Counters

```text
0 → 1 → 2 → 3
```

✅ Inventory

```
Stock = 20

↓

Stock = 19

↓

Stock = 18
```

✅ Wallet Balance

```
100

↓

150

↓

90
```

---

# 3️⃣ Constants (`const`)

## 📖 Theory

A constant stores a value that **must never change**.

Unlike variables,

- constants cannot become mutable
- constants must have a data type
- constants are written in CAPITAL LETTERS

---

### Syntax

```rust
const PI: f64 = 3.141592;
```

Notice

```
const
Name
:
Type
=
Value
;
```

---

### Examples

```text
Maximum Supply

Transaction Fee

API Version

Seconds in One Day
```

Good examples of constants.

---

### Difference Between Variable and Constant

| Variable | Constant |
|-----------|-----------|
| Created using `let` | Created using `const` |
| Can be mutable | Never mutable |
| Type may be inferred | Type required |
| Value may change | Value never changes |

---

# 4️⃣ Primitive Data Types

## 📖 Theory

Primitive types are Rust's built-in basic data types.

Think of them as the building blocks of every program.

---

## Integer

Whole numbers.

Examples

```
1
45
100
-20
```

Common integer types

| Type | Size |
|------|------|
| i8 | 8 bits |
| i16 | 16 bits |
| i32 | 32 bits |
| i64 | 64 bits |
| u8 | Unsigned |
| u64 | Unsigned |

---

### Signed vs Unsigned

Signed

```
-5
0
10
```

Unsigned

```
0
1
2
100
```

No negative values.

---

## Floating Point

Decimal numbers.

```text
3.14

99.99

0.25
```

Types

```
f32

f64
```

---

## Boolean

Stores only

```
true

false
```

---

## Character

Stores one Unicode character.

```rust
'A'

'B'

'🔥'

'₹'
```

Uses single quotes.

---

# 5️⃣ Strings

## 📖 Theory

Rust has two commonly used string types.

---

## String Slice (`&str`)

Fixed.

Usually used for text that never changes.

Example

```rust
let name = "Shivansh";
```

---

## String

Growable.

Can be modified.

```rust
let mut city = String::from("Pune");
```

You can add more text.

---

### Which One Should You Use?

If text never changes

```
&str
```

If text changes

```
String
```

---

# 6️⃣ String Formatting

## 📖 Theory

Rust prints output using the `println!()` macro.

Instead of concatenation,

Rust uses placeholders.

---

Example

```text
Hello Shivansh
```

is written as

```rust
println!("Hello {}", name);
```

The `{}` is replaced by the variable.

---

Multiple variables

```rust
println!("{} is {}", name, age);
```

---

Formatting is safer than manually joining strings.

---

# 7️⃣ Functions

## 📖 Theory

A function is a reusable block of code.

Instead of writing the same code multiple times,

we write it once and call it whenever needed.

---

Example

Without function

```
Add

Add

Add

Add
```

Repeated code.

With function

```
Call add()

Call add()

Call add()
```

Cleaner.

---

### Syntax

```rust
fn function_name(parameters) -> return_type
```

---

Rust functions return the last expression.

Notice there is **no semicolon**.

```rust
a + b
```

This returns the value.

If you write

```rust
a + b;
```

Nothing is returned.

---

# 8️⃣ Comments

## 📖 Theory

Comments are ignored by the compiler.

They are only for humans.

They explain

- why code exists
- what a function does
- future notes

---

Single line

```rust
// This is a comment
```

---

Multiple lines

```rust
/*
Comment

Comment

Comment
*/
```

---

# 💻 Complete Code Example

```rust
// ==============================================
// Module 1 : Rust Basics
// Complete Example
// ==============================================

// -------------------------------
// Constant
// Constants never change.
// Data type is compulsory.
// -------------------------------

const PI: f64 = 3.141592;

// -------------------------------
// Function
// Takes two integers and returns
// their sum.
// Notice there is NO semicolon
// after (a + b)
// -------------------------------

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// -------------------------------
// Main Function
// Every Rust program starts here.
// -------------------------------

fn main() {

    // ---------------------------------
    // Immutable Variable
    // Cannot be changed later.
    // ---------------------------------

    let age = 20;

    // ---------------------------------
    // Mutable Variable
    // Can be modified because of 'mut'
    // ---------------------------------

    let mut balance = 100;

    // Increase balance

    balance += 50;

    // ---------------------------------
    // Primitive Data Types
    // ---------------------------------

    let marks: i32 = 95;

    let temperature: f64 = 36.7;

    let is_student: bool = true;

    let grade: char = 'A';

    // ---------------------------------
    // String Slice
    // Fixed text
    // ---------------------------------

    let name = "Shivansh";

    // ---------------------------------
    // Growable String
    // Can be modified
    // ---------------------------------

    let mut city = String::from("Pune");

    city.push_str(" City");

    // ---------------------------------
    // Calling Function
    // ---------------------------------

    let result = add(10, 20);

    // ---------------------------------
    // Printing Values
    // {} is replaced with variables.
    // ---------------------------------

    println!("Name          : {}", name);

    println!("Age           : {}", age);

    println!("Balance       : {}", balance);

    println!("Marks         : {}", marks);

    println!("Temperature   : {}", temperature);

    println!("Student       : {}", is_student);

    println!("Grade         : {}", grade);

    println!("City          : {}", city);

    println!("Addition      : {}", result);

    println!("PI            : {}", PI);

}
```

---

# 🎯 Module Summary

After completing this module, you should understand:

- ✅ Creating variables
- ✅ Using mutable variables (`mut`)
- ✅ Creating constants (`const`)
- ✅ Rust primitive data types
- ✅ Difference between `String` and `&str`
- ✅ Printing with `println!()`
- ✅ Creating and calling functions
- ✅ Writing comments
- ✅ Reading simple Rust programs

---
