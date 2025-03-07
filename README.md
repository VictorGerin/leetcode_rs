# LeetCode Workspace in Rust

A collection of LeetCode problem solutions implemented in Rust, featuring a custom parser and solution framework.

## 🧐 About

This project contains solutions to various LeetCode problems implemented in Rust. It includes a custom parser that helps handle LeetCode's input formats and test cases, making it easier to test and verify solutions locally.

## 🏁 Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/VictorGerin/leetcode_rs
cd leetcode-workspace
```

2. Build the project:
```bash
cargo build
```

## 🎈 Usage

To run a specific solution:

```bash
cargo run --bin [problem-name]
```

For example:
```bash
cargo run --bin sum_two_numbers
```

## ⛏️ Built Using

- [Rust](https://www.rust-lang.org/) - Programming Language
- Cargo - Dependency Management

## Project Structure

```
.
├── lib/                           # Shared library code
│   ├── src/
│   │   ├── parser/               # Input parsing utilities
│   │   ├── data_structures/      # Common data structures
│   │   └── lib.rs               # Library root
│   └── Cargo.toml               # Library dependencies
├── [problem-name]/               # Individual problem solutions
│   ├── src/                     # Problem-specific code
│   └── Cargo.toml              # Problem-specific dependencies
├── Cargo.toml                    # Workspace configuration
└── Cargo.lock                    # Dependency lock file
```

Each LeetCode problem is organized as a separate package in the workspace, allowing for isolated development and testing. The shared library (`lib`) contains common utilities and data structures used across different problems.

## ✍️ Authors

- Victor Lacerda - Initial work

## 📝 License

This project is licensed under the MIT License.
