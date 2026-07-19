
# Module 4 — Enums, Option & Result (Deep Dive)

> Goal: Learn how Rust represents **one value that can be in multiple states**. Enums are one of Rust's most powerful features.

---

# Why Enums?

Structs answer:

> What properties does this object have?

Enums answer:

> What state is this value currently in?

Example:

Traffic Light

```rust
enum TrafficLight{
    Red,
    Yellow,
    Green,
}
```

Only one variant can exist at a time.

---

# Enums with Data

```rust
enum Shape{

    Circle(f64),

    Rectangle{
        width:f64,
        height:f64,
    },

}
```

Each variant may store different data.

---

# Pattern Matching

```rust
match light{

    TrafficLight::Red=>println!("Stop"),

    TrafficLight::Yellow=>println!("Slow"),

    TrafficLight::Green=>println!("Go"),

}
```

Rust forces exhaustive handling.

---

# Option<T>

Instead of `null`, Rust uses

```rust
enum Option<T>{

    Some(T),

    None,

}
```

Example

```rust
let age = Some(20);

let salary:Option<i32> = None;
```

Avoids null pointer bugs.

---

# Matching Option

```rust
match age{

    Some(value)=>println!("{}",value),

    None=>println!("No value"),

}
```

---

# if let

Convenient syntax.

```rust
if let Some(value)=age{

    println!("{}",value);

}
```

---

# Result<T,E>

Represents success or failure.

```rust
enum Result<T,E>{

    Ok(T),

    Err(E),

}
```

Example

```rust
fn divide(a:i32,b:i32)->Result<i32,String>{

    if b==0{

        Err(String::from("Cannot divide by zero"))

    }else{

        Ok(a/b)

    }

}
```

---

# Handling Result

```rust
match divide(10,2){

    Ok(answer)=>println!("{}",answer),

    Err(error)=>println!("{}",error),

}
```

---

# The ? Operator

Instead of

```rust
match result{

    Ok(v)=>v,

    Err(e)=>return Err(e),

}
```

simply write

```rust
let value = result?;
```

It propagates errors automatically.

---

# Solana Connection

Enums are everywhere.

Examples

- Instruction types
- Program errors
- Transaction status
- Anchor error handling
- Option accounts

Instruction enum

```rust
enum Instruction{

    Initialize,

    Deposit,

    Withdraw,

    Close,

}
```

---

# Complete Example

```rust
enum Payment{

    Cash,

    Card(String),

    UPI(String),

}

fn process(payment:Payment){

    match payment{

        Payment::Cash=>println!("Cash"),

        Payment::Card(number)=>println!("Card {}",number),

        Payment::UPI(id)=>println!("UPI {}",id),

    }

}

fn main(){

    process(Payment::Cash);

    process(Payment::UPI(String::from("alice@upi")));

}
```

---

# Common Mistakes

- Thinking enums are like C enums (Rust enums can store data).
- Ignoring `None`.
- Calling `.unwrap()` everywhere.
- Forgetting to handle every `match` branch.

---

# Best Practices

- Prefer `Option` over sentinel values.
- Prefer `Result` over panic for recoverable errors.
- Use enums to model application states explicitly.
