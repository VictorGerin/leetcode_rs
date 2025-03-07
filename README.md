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
git clone [your-repository-url]
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
cargo run --bin two-sum
```

## ⛏️ Built Using

- [Rust](https://www.rust-lang.org/) - Programming Language
- Cargo - Dependency Management

## Project Structure

```
.
├── src/           # Source directory
│   ├── parser/    # Input parsing utilities
│   └── solutions/ # LeetCode problem solutions
├── tests/         # Test cases
└── Cargo.toml     # Project dependencies and configuration
```

## ✍️ Authors

- Your Name - Initial work

## 📝 License

This project is licensed under the MIT License.
