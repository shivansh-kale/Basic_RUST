
# Module 5 — Collections (Vec, HashMap, Slices & Iteration)

> **Goal:** Learn how Rust stores groups of values efficiently. Collections are used constantly in Solana programs for instruction data, account lists, transaction processing, and application state.

---

# Why Collections?

Primitive types store **one value**.

```rust
let age = 20;
```

Real applications need to store **many values**.

Examples:

- List of users
- List of transactions
- NFT metadata
- Account balances
- Validators
- Program instructions

Rust provides collections for this purpose.

The two you'll use most are:

- `Vec<T>`
- `HashMap<K, V>`

---

# Collection Categories

| Collection | Purpose |
|------------|----------|
| Vec | Ordered list |
| HashMap | Key → Value lookup |
| Slice | Borrow part of a collection |

---

# Vec<T>

A `Vec<T>` (Vector) is a **growable array**.

Unlike normal arrays, vectors can increase or decrease in size.

```rust
let numbers = vec![1,2,3];
```

Memory

```
Stack
+------------------+
| ptr len cap      |
+------------------+
        |
        v

Heap
+--------------------+
|1|2|3|
+--------------------+
```

The vector itself stores only:

- Pointer
- Length
- Capacity

Actual elements live on the heap.

---

# Creating Vectors

```rust
let v = Vec::<i32>::new();

let numbers = vec![10,20,30];
```

---

# Adding Elements

```rust
let mut numbers = Vec::new();

numbers.push(10);
numbers.push(20);
numbers.push(30);
```

---

# Removing Elements

```rust
numbers.pop();
```

Returns

```rust
Option<T>
```

because the vector may already be empty.

---

# Accessing Elements

Method 1

```rust
println!("{}", numbers[0]);
```

Panics if index doesn't exist.

Method 2 (Recommended)

```rust
match numbers.get(0){

    Some(value)=>println!("{}",value),

    None=>println!("Index not found"),

}
```

---

# Iteration

Read only

```rust
for value in &numbers{

    println!("{}",value);

}
```

Mutable

```rust
for value in &mut numbers{

    *value += 1;

}
```

Ownership

```rust
for value in numbers{

    println!("{}",value);

}
```

This consumes the vector.

---

# Capacity

Length

```
Current number of elements.
```

Capacity

```
Allocated space.
```

Example

```
Length = 3

Capacity = 8
```

Rust allocates extra memory to reduce frequent reallocations.

Useful methods

```rust
numbers.len();

numbers.capacity();
```

---

# Slices

A slice borrows part of a collection.

```rust
let values = vec![10,20,30,40];

let slice = &values[1..3];
```

Result

```
20
30
```

No copying occurs.

The slice only references existing memory.

---

# Strings are Slices Too

```rust
let name = String::from("Solana");

let part = &name[0..3];
```

`part`

```
Sol
```

Again, no copy.

---

# HashMap

Stores

```
Key

↓

Value
```

Example

```
Alice -> 100

Bob -> 200
```

Import

```rust
use std::collections::HashMap;
```

---

# Creating

```rust
let mut balances = HashMap::new();
```

---

# Insert

```rust
balances.insert("Alice",100);

balances.insert("Bob",250);
```

---

# Read

```rust
match balances.get("Alice"){

    Some(balance)=>println!("{}",balance),

    None=>println!("Not found"),

}
```

---

# Update

```rust
balances.insert("Alice",500);
```

Existing value is replaced.

---

# Iterating

```rust
for (user,balance) in &balances{

    println!("{} {}",user,balance);

}
```

---

# Ownership Inside Collections

Collections own their elements.

```rust
let name = String::from("Alice");

let mut users = Vec::new();

users.push(name);
```

Ownership moves into the vector.

This fails

```rust
println!("{}",name);
```

because `users` is now the owner.

---

# Solana Connection

Collections appear everywhere.

Examples

- List of account infos
- Transaction instructions
- Validator lists
- Token holders
- NFT metadata
- Instruction serialization

You'll frequently see

```rust
Vec<AccountInfo<'info>>
```

and

```rust
Vec<u8>
```

for serialized instruction data.

---

# Complete Example

```rust
use std::collections::HashMap;

fn main(){

    // -------------------------
    // Vector
    // -------------------------

    let mut scores = Vec::new();

    scores.push(90);
    scores.push(95);
    scores.push(99);

    println!("Length {}",scores.len());

    for score in &scores{

        println!("{}",score);

    }

    // -------------------------
    // Slice
    // -------------------------

    let first_two = &scores[0..2];

    println!("{:?}",first_two);

    // -------------------------
    // HashMap
    // -------------------------

    let mut balances = HashMap::new();

    balances.insert("Alice",1000);

    balances.insert("Bob",2500);

    match balances.get("Alice"){

        Some(balance)=>println!("Alice {}",balance),

        None=>println!("Missing"),

    }

    for (user,balance) in &balances{

        println!("{} {}",user,balance);

    }

}
```

---

# Common Beginner Mistakes

1. Using indexing (`[]`) without checking bounds.
2. Forgetting that `push()` moves ownership.
3. Consuming a vector accidentally with `for value in vec`.
4. Confusing length with capacity.
5. Using a `HashMap` when ordering matters.

---

# Best Practices

- Prefer `get()` over indexing when values may be absent.
- Iterate using references whenever possible.
- Use slices instead of copying data.
- Reserve capacity for large vectors if the size is known.
- Think about ownership whenever inserting heap-allocated values (`String`, `Vec`, structs) into collections.
