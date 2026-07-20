# Besaz

A modern, simple, and extensible build system written in Rust.

Besaz lets you define your own project workflows using a `Besazfile`.

```bash
besaz build
```

The `build` command is not built into Besaz. It is a task defined by the user.

---

## Features

- 🚀 Simple task-based workflow
- 📝 `Besazfile` configuration
- ⚡ Fast execution
- 🎨 Colored terminal output
- 🔧 Single or multiple commands per task
- 💡 User-defined commands
- 🦀 Written in Rust

---

## Installation

### From source

```bash
git clone https://github.com/yourusername/besaz.git
cd besaz
cargo install --path .
```

Check installation:

```bash
besaz --version
```

---

## Quick Start

Create a `Besazfile` in your project:

```toml
[tasks.build]
description = "Build the project"
run = "cargo build --release"

[tasks.test]
description = "Run tests"
run = "cargo test"
```

Run a task:

```bash
besaz build
```

Output:

```text
Running cargo build --release
```

---

## Multiple Commands

A task can execute multiple commands:

```toml
[tasks.release]
description = "Create a release"

run = [
    "cargo fmt",
    "cargo test",
    "cargo build --release"
]
```

Running:

```bash
besaz release
```

Executes:

```text
cargo fmt
cargo test
cargo build --release
```

in order.

---

## Task List

Running Besaz without arguments shows available tasks:

```bash
besaz
```

Example:

```text
Available tasks:

  build     Build the project
  release   Create a release
  test      Run tests
```

Descriptions are optional:

```toml
[tasks.clean]
run = "cargo clean"
```

---

## Configuration

A simple task:

```toml
[tasks.build]
run = "make"
```

A more complex workflow:

```toml
[tasks.deploy]
description = "Deploy application"

run = [
    "./prepare.sh",
    "./build.sh",
    "./deploy.sh"
]
```

---

## Design Philosophy

Besaz follows a simple idea:

```
User defines tasks
        |
        v
Besaz executes them
```

Besaz does not force a build system.

You decide how your project is built.

---

## Roadmap

- [x] User-defined tasks
- [x] Multiple commands
- [x] Task descriptions
- [ ] Task dependencies
- [ ] Working directory support
- [ ] Environment variables
- [ ] Parallel execution
- [ ] File watching
- [ ] Build caching
- [ ] Plugin system

---

## Name

**Besaz (بساز)** means **"build"** or **"make"** in Persian.

It represents the purpose of the project:

> Define. Build. Create.

---

## License

MIT