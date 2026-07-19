
# Module 3 — Structs & Methods (Deep Dive)

> Goal: Learn how Rust models real-world data. Structs are the backbone of most Rust applications and are heavily used in Solana accounts.

---

# Why Structs?

Primitive types (`i32`, `bool`, `char`) represent one value.

Real applications need to group related information.

Instead of

```rust
let name = "Alice";
let age = 20;
let balance = 1500;
```

we create

```rust
struct User {
    name: String,
    age: u8,
    balance: u64,
}
```

Now one variable represents one user.

---

# Defining a Struct

```rust
struct User {
    name: String,
    age: u8,
    balance: u64,
}
```

Every field has a name and a type.

---

# Creating an Instance

```rust
let user = User {
    name: String::from("Alice"),
    age: 20,
    balance: 1000,
};
```

Rust initializes fields by name, not position.

---

# Accessing Fields

```rust
println!("{}", user.name);
println!("{}", user.balance);
```

---

# Mutable Structs

```rust
let mut user = User {
    name: String::from("Bob"),
    age: 21,
    balance: 500,
};

user.balance += 200;
```

The whole binding must be mutable.

---

# Struct Update Syntax

```rust
let user2 = User {
    name: String::from("Charlie"),
    ..user
};
```

Fields not explicitly mentioned are moved/copied from `user`.

Ownership rules still apply.

---

# Tuple Structs

Useful when names add little value.

```rust
struct Color(u8,u8,u8);

let red = Color(255,0,0);

println!("{}", red.0);
```

---

# Unit Structs

```rust
struct Logger;
```

No fields.

Useful as markers or traits.

---

# Associated Functions

Functions inside an `impl` block.

```rust
impl User {

    fn new(name:String, age:u8)->Self{

        Self{
            name,
            age,
            balance:0
        }

    }

}
```

Called using

```rust
let u = User::new(String::from("Alice"),20);
```

---

# Methods

Methods receive `self`.

```rust
impl User {

    fn deposit(&mut self, amount:u64){

        self.balance += amount;

    }

    fn display(&self){

        println!("{} {}",self.name,self.balance);

    }

}
```

Three forms:

- `self` (takes ownership)
- `&self` (read-only borrow)
- `&mut self` (mutable borrow)

---

# When to Use Each?

| Receiver | Meaning |
|-----------|---------|
| self | Consume object |
| &self | Read only |
| &mut self | Modify |

---

# Derive Traits

```rust
#[derive(Debug)]
struct User{
    name:String,
    age:u8,
}
```

Now

```rust
println!("{:?}", user);
```

works.

Useful derives:

- Debug
- Clone
- Copy (small types)
- Default
- PartialEq

---

# Solana Connection

Almost every Solana account is represented as a struct.

```rust
pub struct UserAccount {
    pub authority: Pubkey,
    pub balance: u64,
}
```

Anchor serializes/deserializes structs automatically.

---

# Complete Example

```rust
#[derive(Debug)]

struct BankAccount{

    owner:String,
    balance:u64,

}

impl BankAccount{

    fn new(owner:String)->Self{

        Self{
            owner,
            balance:0
        }

    }

    fn deposit(&mut self, amount:u64){

        self.balance += amount;

    }

    fn withdraw(&mut self, amount:u64){

        if self.balance>=amount{
            self.balance-=amount;
        }

    }

    fn show(&self){

        println!("{:?}",self);

    }

}

fn main(){

    let mut account=BankAccount::new(String::from("Shivansh"));

    account.deposit(500);

    account.withdraw(100);

    account.show();

}
```

---

# Best Practices

- Group related data.
- Prefer methods over free functions.
- Use `&self` whenever mutation is unnecessary.
- Keep structs focused on one responsibility.
