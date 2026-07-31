# Besaz

[![CI](https://img.shields.io/github/actions/workflow/status/ManiProjs/besaz/release.yml?style=flat-square)](https://github.com/ManiProjs/besaz/actions)
[![License](https://img.shields.io/github/license/ManiProjs/besaz?style=flat-square)](LICENSE)
[![Rust](https://img.shields.io/badge/made%20with-Rust-orange?style=flat-square)](https://www.rust-lang.org)

A modern, simple, and extensible task runner written in Rust.

Besaz lets you define your own project workflows using a `Besazfile`.

```bash
besaz build
```

The `build` command is not built into Besaz. It is a task created by you.

---

## Features

- 🚀 Simple task-based workflow
- 📝 Human-readable `Besazfile` configuration
- ⚡ Fast native execution
- 🎨 Colored terminal output
- 🔧 Single or multiple commands per task
- 💡 Fully user-defined commands
- 🦀 Written in Rust
- 📦 Single portable binary

---

## Installation

###

### From source

Requires Rust and Cargo.

```bash
git clone https://github.com/ManiProjs/besaz.git
cd besaz

cargo install --path .
```

Verify installation:

```bash
besaz --version
```

---

## Quick Start

Create a file named `Besazfile` in your project:

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

Tasks can contain multiple commands:

```toml
[tasks.release]
description = "Prepare a release"

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

## Listing Tasks

Run Besaz without arguments:

```bash
besaz
```

Example output:

```text
Available tasks:

  build     Build the project
  release   Prepare a release
  test      Run tests
```

Descriptions are optional:

```toml
[tasks.clean]
run = "cargo clean"
```

---

## Configuration

Simple task:

```toml
[tasks.build]
run = "make"
```

Workflow example:

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

## How It Works

Besaz keeps the build system simple:

```
+----------------+
| User defines   |
| tasks          |
+-------+--------+
        |
        v
+----------------+
| Besaz executes |
| commands       |
+----------------+
```

Besaz does not replace your existing tools.

You can use:

- Cargo
- Make
- CMake
- Shell scripts
- Any command-line tool

Besaz simply organizes them into reusable workflows.

---

## Why Besaz?

Many projects need a small automation layer, but full build systems can be unnecessary.

Besaz provides:

- A simple configuration format
- A consistent command interface
- A project-local workflow definition

No magic. No hidden behavior.

---

## Supported Platforms

Official releases are available for:

- Linux x86_64
- Linux ARM64
- macOS Intel
- macOS Apple Silicon
- Windows x86_64

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

**Besaz (بساز)** means **"build"**, **"create"**, or **"make"** in Persian.

The idea behind the name:

> Define. Build. Create.

---

## License

MIT