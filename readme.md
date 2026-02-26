# Learning Rust — Workspace Setup

This short guide shows how this `Learning_Rust` workspace is structured and how the `Cargo.toml` files are wired up.

✅ Step 1 — Create Root Folder

Create your main learning folder and enter it:

```bash
mkdir Learning_Rust
cd Learning_Rust
```

✅ Step 2 — Workspace `Cargo.toml` (Root)

The root `Cargo.toml` declares this directory as a workspace and lists the three member crates:

```toml
[workspace]
resolver = "3"
members  = [
  "01-basic",
  "02-data_types_and_structures",
  "03-functions",
]
```

- The folder is a Cargo workspace.
- Each listed folder is a crate that lives inside the workspace.

✅ Step 3 — Current Workspace Layout

The workspace you are looking at is organized like this:

```
Learning_Rust/
  Cargo.toml                 <-- workspace (see members above)

  01-basic/
    Cargo.toml               <-- simple binary crate
    src/main.rs

  02-data_types_and_structures/
    Cargo.toml               <-- crate with many binaries
    01-scalar/
      main.rs
    02-compound/
      01-tuple.rs
      02-arrays.rs
      03-slices.rs
      04-strings.rs
    03-structs/
      01-structs.rs
      02-initialization_and_update.rs
      03-methods_and_associated_fn.rs
    04-enums/
      01-enums.rs
      02-enum_matching.rs
      03-methods.rs

  03-functions/
    Cargo.toml               <-- crate with three binaries
    01-functions.rs
    02-parameter_passing.rs
    03-return_values.rs
```

Inside `02-data_types_and_structures` and `03-functions`, the `Cargo.toml` files declare multiple `[[bin]]` targets so each file can be run as its own example.

✅ Step 4 — Useful Cargo Commands

- Build everything at once: `cargo build --workspace`
- Run a binary from `01-basic`: `cargo run -p chapter-01`
- Run a specific example from data types: `cargo run -p chapter-02 --bin tuples`
- Run a specific example from functions: `cargo run -p chapter-03 --bin return_values`

Next steps

- Add dependencies per-crate in their `Cargo.toml` files.
- Use this layout as a template if you want to add more learning crates (e.g., `04-control_flow`, `05-ownership`), and remember to register them under `[workspace].members` in the root `Cargo.toml`.
