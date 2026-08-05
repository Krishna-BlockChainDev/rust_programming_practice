# 🦀 Rust Programming - Setup Guide

## Prerequisites & Installation

### Step 1: Install Rust (via rustup)

Rust is installed via `rustup`, the official Rust toolchain installer.

#### On Linux / macOS:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions. Choose option **1 (default installation)**.

After installation, reload your shell:

```bash
source $HOME/.cargo/env
```

#### On Windows:

Download and run: https://www.rust-lang.org/tools/install  
(installs `rustup-init.exe`)

---

### Step 2: Verify Installation

```bash
rustc --version
cargo --version
rustup --version
```

You should see output like:

```
rustc 1.78.0 (...)
cargo 1.78.0 (...)
rustup 1.27.0 (...)
```

---

### Step 3: Update Rust (anytime)

```bash
rustup update
```

---

## How to Run Each Day's Exercises

Each day has its own folder (e.g., `Day-1/`, `Day-2/`, ...) with a Rust file inside.

### Option A: Run using `cargo` (Recommended)

Each day folder is a mini Cargo project. Navigate into the folder and run:

```bash
cd Day-1
cargo run
```

To run with release optimization:

```bash
cargo run --release
```

### Option B: Compile & Run using `rustc` directly

```bash
cd Day-1/src
rustc main.rs
./main
```

---

## Folder Structure

```
RUST_PROGRAMMINGS/
├── SETUP_GUIDE.md          ← You are here
├── CURRICULUM.md           ← Full 60-day plan overview
├── Day-1/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs         ← Exercise file with comments
├── Day-2/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── ...
```

---

## Creating a New Cargo Project (for reference)

```bash
cargo new my_project
cd my_project
cargo run
```

---

## Useful Cargo Commands

| Command            | Description                       |
| ------------------ | --------------------------------- |
| `cargo run`        | Compile and run the project       |
| `cargo build`      | Compile without running           |
| `cargo check`      | Check for errors without building |
| `cargo test`       | Run tests                         |
| `cargo doc --open` | Generate and open documentation   |
| `cargo fmt`        | Format code                       |
| `cargo clippy`     | Linting / suggestions             |

---

## Recommended VS Code Extensions

1. **rust-analyzer** — Official Rust language support (autocomplete, errors)
2. **CodeLLDB** — Debugger for Rust
3. **Even Better TOML** — Cargo.toml support
4. **Error Lens** — Inline error display

Install via VS Code Extensions panel or:

```bash
code --install-extension rust-lang.rust-analyzer
code --install-extension vadimcn.vscode-lldb
```

---

## Online Resources

| Resource              | URL                                        |
| --------------------- | ------------------------------------------ |
| The Rust Book (Free)  | https://doc.rust-lang.org/book/            |
| Rust by Example       | https://doc.rust-lang.org/rust-by-example/ |
| Rustlings (Exercises) | https://github.com/rust-lang/rustlings     |
| Playground (Online)   | https://play.rust-lang.org/                |
| Standard Library Docs | https://doc.rust-lang.org/std/             |
| Crates.io (packages)  | https://crates.io/                         |

---

## Daily Practice Tips

- ⏰ Dedicate **30–45 minutes** daily — consistency beats intensity
- 📖 Read the comments in each `main.rs` before coding
- ✏️ Type the code manually — don't copy-paste (muscle memory!)
- 🔁 Revisit previous days when stuck
- 🧪 Experiment — modify the exercises and observe results
- 📝 Keep notes in a journal or comments in your code

---

## Troubleshooting

### "command not found: cargo"

```bash
source $HOME/.cargo/env
# or add to ~/.bashrc or ~/.zshrc:
export PATH="$HOME/.cargo/bin:$PATH"
```

### "error[E0...]: ..." in terminal

Read the error message carefully — Rust errors are very descriptive!  
Use `cargo check` for faster feedback during development.

### Stuck on a concept?

Run: `rustup doc --book` to open The Rust Book offline.

---

Happy Coding! 🦀🚀
