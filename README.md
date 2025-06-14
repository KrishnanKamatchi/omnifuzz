# ⚡ omnifuzz

**omnifuzz** is a high-performance, cross-platform terminal fuzzy finder written in Rust — combining the power of live search, GUI-like TUI, and real-time preview in one ultra-slick CLI app.

---

## 🚀 Features

- 🔍 **Fuzzy search**: Instantly find files, folders, lines, or words
- 🧭 **Type filter**: Search by type (file, dir, exact, partial, etc.)
- 🖥️ **Terminal UI**: Built with [`ratatui`](https://crates.io/crates/ratatui) for rich layouts
- 🧠 **Smart preview**: View file contents before opening
- 🎯 **Keyboard control**: Intuitive Vim-style navigation
- 🧩 **Modular design**: Easily extendable: `theme`, `hooks`, `preview`, etc.
- ⚡ **Ultra fast**: Powered by `walkdir`, `fuzzy-matcher`, and optimized rendering

---

## 🧰 Installation

### ✅ Prerequisites

- Rust ≥ 1.70
- Git

### 📦 Build & Install

```bash
git clone https://github.com/yourname/omnifuzz.git
cd omnifuzz
cargo install --path .
