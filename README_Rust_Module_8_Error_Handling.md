
# Module 8 — Error Handling (Deep Dive)

> Goal: Learn Rust's philosophy of handling errors explicitly instead of relying on exceptions.

---

# Why doesn't Rust have Exceptions?

Languages like Java or Python use exceptions.

```python
try:
    ...
except:
    ...
```

Rust chooses explicit error handling because:

- Errors become part of the function's type.
- The compiler forces you to handle failures.
- Programs are easier to reason about.

---

# Two Categories of Errors

## Recoverable Errors

Examples:

- File not found
- Network timeout
- Invalid user input

Represented by:

```rust
Result<T, E>
```

---

## Unrecoverable Errors

Examples:

- Index out of bounds
- Impossible program state
- Internal logic bugs

Represented by:

```rust
panic!()
```

---

# panic!

```rust
panic!("Something went wrong");
```

The current thread stops immediately.

Use panic only when continuing execution would be incorrect.

---

# Result<T, E>

Definition (simplified):

```rust
enum Result<T,E>{
    Ok(T),
    Err(E),
}
```

`T` = success value

`E` = error value

---

# Creating Results

```rust
fn divide(a:i32,b:i32)->Result<i32,String>{

    if b==0{
        Err(String::from("Division by zero"))
    }else{
        Ok(a/b)
    }

}
```

---

# Matching

```rust
match divide(10,2){

    Ok(value)=>println!("{}",value),

    Err(error)=>println!("{}",error),

}
```

Compiler ensures every case is handled.

---

# The ? Operator

Without `?`

```rust
let value = match divide(10,2){
    Ok(v)=>v,
    Err(e)=>return Err(e),
};
```

With `?`

```rust
let value = divide(10,2)?;
```

The error automatically propagates upward.

---

# unwrap()

```rust
let value = divide(10,2).unwrap();
```

Returns the value if successful.

Panics if an error exists.

Avoid in production unless failure is impossible.

---

# expect()

```rust
let value = divide(10,2)
    .expect("Division failed");
```

Better than unwrap because the panic message is meaningful.

---

# Custom Errors

```rust
#[derive(Debug)]

enum MathError{

    DivideByZero,

    NegativeInput,

}
```

Returning

```rust
Result<i32,MathError>
```

is much better than returning strings.

---

# Result Combinators

Useful methods:

```rust
map()

map_err()

unwrap_or()

unwrap_or_else()

and_then()
```

These reduce repetitive `match` statements.

---

# Solana Connection

Errors are extremely important.

Programs return

```rust
ProgramResult
```

Anchor uses

```rust
Result<()>
```

Custom errors

```rust
#[error_code]

pub enum ErrorCode{

    Unauthorized,

    InvalidAmount,

}
```

Every failed instruction returns an error code to the client.

---

# Complete Example

```rust
#[derive(Debug)]

enum BankError{

    InsufficientFunds,

}

fn withdraw(balance:u64,amount:u64)->Result<u64,BankError>{

    if amount>balance{

        Err(BankError::InsufficientFunds)

    }else{

        Ok(balance-amount)

    }

}

fn main(){

    match withdraw(100,50){

        Ok(new_balance)=>println!("Balance {}",new_balance),

        Err(error)=>println!("{:?}",error),

    }

}
```

---

# Best Practices

- Prefer Result over panic.
- Use `?` whenever possible.
- Create custom error enums.
- Reserve panic for unrecoverable programmer mistakes.
