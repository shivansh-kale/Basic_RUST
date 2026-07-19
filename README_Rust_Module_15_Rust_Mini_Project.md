
# Module 15 — Rust Mini Project Before Solana

> Goal: Build one small Rust project that combines almost every concept you've learned before moving to Anchor.

---

# Project: Banking CLI

Features:

- Create account
- Deposit
- Withdraw
- Transfer
- View balance
- Exit

---

# Concepts Used

✅ Structs

✅ Enums

✅ Traits

✅ Generics

✅ Collections

✅ Result

✅ Modules

✅ Cargo

---

# Suggested Folder Structure

```text
bank/
│
├── Cargo.toml
└── src/
    ├── main.rs
    ├── account.rs
    ├── bank.rs
    ├── errors.rs
    └── utils.rs
```

---

# Account Struct

```rust
pub struct Account{
    pub id:u32,
    pub owner:String,
    pub balance:u64,
}
```

---

# Methods

```rust
deposit()

withdraw()

transfer()
```

Each returns

```rust
Result<(),BankError>
```

---

# Store Accounts

```rust
HashMap<u32,Account>
```

---

# Error Enum

```rust
enum BankError{
    AccountNotFound,
    InsufficientFunds,
}
```

---

# CLI Loop

```
1 Create
2 Deposit
3 Withdraw
4 Transfer
5 View
6 Exit
```

Use:

```
loop

match

Result

HashMap
```

---

# Learning Outcome

After this project you will have practiced:

- Ownership
- Borrowing
- Structs
- Methods
- Enums
- Collections
- Error handling
- Modules
- Cargo
- Traits
- Generics

Everything required for beginner Solana.

---

# Next Step

Once this project feels comfortable, begin:

1. Install Solana CLI
2. Install Anchor
3. Create `anchor init`
4. Build Counter Program
5. Build CRUD Program
6. Build Vault
7. Build Escrow
8. Build Token Staking

---

# Final Rust Checklist

- Variables ✓
- Ownership ✓
- Borrowing ✓
- Structs ✓
- Enums ✓
- Collections ✓
- Traits ✓
- Generics ✓
- Error Handling ✓
- Lifetimes ✓
- Cargo ✓
- Closures ✓
- Smart Pointers ✓
- Macros ✓

You now have the Rust foundation needed to confidently start Solana development.
