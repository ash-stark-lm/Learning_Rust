```rust
fn takes_ownership(s: String)
fn makes_copy(some_integer: i32)
```

Both:

- Take one parameter
- Print it
- End → parameter goes out of scope

So why different behavior?

---

# 🔥 The Real Difference Is NOT The Function

It’s the **type**.

---

## 🧠 1️⃣ `String` → Move Happens

```rust
let s = String::from("hello");
takes_ownership(s);
```

`String` does **not** implement `Copy`.

When you pass it to a function:

- Ownership is **moved**
- `main` no longer owns `s`
- `takes_ownership` now owns it
- When function ends → `s` is dropped (memory freed)

### 📦 Why?

Because `String` stores data on the **heap**:

```
Stack:
  s → (ptr, len, capacity)

Heap:
  "hello"
```

Rust prevents double-free.
So it transfers ownership instead of copying.

---

## 🧠 2️⃣ `i32` → Copy Happens

```rust
let x = 5;
makes_copy(x);
```

`i32` **implements the `Copy` trait**.

That means:

- Passing it creates a **bitwise copy**
- No ownership transfer
- Both variables are valid

### Why?

`i32`:

- Lives completely on the **stack**
- Fixed size
- No heap memory
- Cheap to copy

---

# ⚡ Core Rule

### If type implements `Copy` → it is copied

### If type does NOT implement `Copy` → it is moved

---

# 📌 Types That Implement `Copy`

- `i32`, `i64`
- `u32`
- `f32`, `f64`
- `bool`
- `char`
- Tuples of copy types

---

# 📌 Types That Do NOT Implement `Copy`

- `String`
- `Vec<T>`
- `HashMap`
- Any heap-allocated data
- Structs containing non-Copy types

---

# 🎯 Why Rust Designed It This Way?

If `String` were automatically copied:

```rust
let s1 = String::from("hello");
let s2 = s1;   // imagine this copied
```

Both would point to same heap memory.

Then:

- s1 drops → frees memory
- s2 drops → tries to free again ❌
- 💥 Double free crash

Rust prevents this by enforcing **move semantics**.

---

# 🚀 If You WANT Copy Behavior for String

You must explicitly clone:

```rust
let s = String::from("hello");
takes_ownership(s.clone());
println!("{}", s); // still valid
```

Now you have two separate heap allocations.

---

# 🧩 Simple Mental Model

| Type     | Memory       | Trait   | What Happens |
| -------- | ------------ | ------- | ------------ |
| `i32`    | Stack        | Copy    | Copied       |
| `String` | Heap + Stack | No Copy | Moved        |

---

# 🏆 One-Line Answer

The functions behave differently **not because of the functions**,
but because `String` moves ownership and `i32` implements `Copy`.

---

If you want, I can now explain:

- How `Copy` and `Clone` differ
- How ownership looks in memory diagram
- How to write your own struct with `Copy`
- Or give a deep low-level memory explanation

What level do you want? 😄
