
# Module 14 — Rust for Solana & Anchor

> Goal: Connect everything you've learned in Rust to real Solana development.

---

# Why Rust for Solana?

A Solana program is simply a Rust program compiled to eBPF.

Your Rust knowledge now maps directly to blockchain concepts.

| Rust | Solana |
|------|---------|
| Struct | Account Data |
| Enum | Instructions |
| Trait | Serialization / Framework APIs |
| Result | ProgramResult |
| Lifetime | Account References |
| Module | Program Organization |

---

# High-Level Flow

```
Client
   │
   ▼
Instruction Data
   │
   ▼
Transaction
   │
   ▼
Validator
   │
   ▼
Your Rust Program
   │
   ▼
Accounts Updated
```

---

# Accounts

Unlike traditional applications, Solana stores state inside **accounts**.

```rust
pub struct UserAccount{
    pub authority: Pubkey,
    pub balance: u64,
}
```

Accounts are serialized into bytes and stored on-chain.

---

# Borsh Serialization

Rust structs cannot be stored directly.

```
Rust Struct
      │
      ▼
Serialize
      │
      ▼
Bytes
      │
      ▼
Blockchain Storage
```

Later

```
Bytes
  │
  ▼
Deserialize
  │
  ▼
Rust Struct
```

Anchor performs this automatically.

---

# Program Derived Addresses (PDAs)

PDAs are deterministic addresses generated from:

- Seeds
- Program ID

Why?

- No private key
- Program-controlled state
- Predictable addresses

---

# Context<'info, T>

Every Anchor instruction receives a Context.

```rust
pub fn deposit(
    ctx: Context<Deposit>,
    amount: u64,
) -> Result<()> {

    Ok(())
}
```

Context contains:

- Accounts
- Signers
- System program
- Program metadata

---

# Account<'info, T>

```rust
pub account: Account<'info, UserAccount>
```

Meaning:

- Borrow account information
- Deserialize bytes into `UserAccount`
- Validate ownership

---

# #[derive(Accounts)]

This macro generates validation code.

Instead of manually checking:

- signer
- owner
- mutability
- PDA seeds

Anchor generates the code.

---

# Instruction Enums

Without Anchor

```rust
enum Instruction {
    Initialize,
    Deposit,
    Withdraw,
}
```

Incoming bytes are matched to instruction variants.

---

# Cross Program Invocation (CPI)

Programs can call other programs.

```
Program A
    │
    ▼
Program B
```

Example:

- Token Program
- System Program
- Associated Token Program

---

# Error Handling

Anchor

```rust
#[error_code]
pub enum ErrorCode {
    Unauthorized,
    InvalidAmount,
}
```

Returns custom error codes to clients.

---

# Typical Anchor Project

```text
programs/
tests/
app/
Anchor.toml
Cargo.toml
```

---

# Complete Instruction Flow

```
Wallet
   │
   ▼
Transaction
   │
   ▼
Instruction Data
   │
   ▼
Anchor
   │
   ▼
Deserialize Accounts
   │
   ▼
Validate Accounts
   │
   ▼
Execute Rust Logic
   │
   ▼
Serialize Updated Accounts
```

---

# Mapping Everything You've Learned

- Ownership → Account borrowing
- Lifetimes → `'info`
- Traits → Serialization
- Generics → `Context<T>`
- Structs → Account state
- Enums → Instructions
- Results → Program errors

---

# Best Practices

- Let Anchor handle boilerplate.
- Understand what macros generate.
- Keep business logic separate from validation.
- Think in terms of accounts instead of database rows.
