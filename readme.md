# Learning Rust — Workspace Setup

This short guide shows how to create a simple Rust workspace and add member projects.

✅ Step 1 — Create Root Folder

Create your main learning folder and enter it:

```bash
mkdir Learning_Rust
cd Learning_Rust
```

✅ Step 2 — Create Workspace `Cargo.toml` (Important)

Create a root `Cargo.toml` that declares the directory as a workspace:

```bash
touch Cargo.toml
```

Open `Cargo.toml` and write:

```toml
[workspace]
members = []
```

This tells Cargo:

- The folder is a workspace.
- Projects inside it will be workspace members.

✅ Step 3 — Create Projects Inside Workspace

Create projects the normal way (examples below create binary crates):

```bash
cargo new chapter-1
cargo new chapter-2
cargo new mini-project
```

Your structure becomes:

```
Learning_Rust/
	Cargo.toml        <-- workspace
	chapter-1/
		Cargo.toml
		src/main.rs
	chapter-2/
		Cargo.toml
		src/main.rs
	mini-project/
		Cargo.toml
		src/main.rs
```

If you already have folders named like `chapter-01`, either rename them or use those exact names in the workspace members list (see next step).

✅ Step 4 — Register Them in Workspace

Edit the root `Cargo.toml` and list the members:

```toml
[workspace]
members = [
	 "chapter-1",
	 "chapter-2",
	 "mini-project",
]
```

Notes & Tips

- If you created folders with different names (e.g., `chapter-01`), include those exact paths in `members`.
- To initialize an existing folder as a crate, use `cargo init --bin <folder>`.
- Build everything at once with: `cargo build --workspace`.
- Build a single member: `cargo build -p chapter-1`.

Next steps

- Add dependencies per-crate in their `Cargo.toml` files.
- Consider adding a `README.md` at the workspace root describing the learning plan.
