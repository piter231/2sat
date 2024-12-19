
---

# 🧩 2SAT: A WebAssembly-Based 2-SAT Solver 🚀

**2SAT** is a blazing-fast **2-SAT Solver** powered by **Rust** and compiled to **WebAssembly (Wasm)**. This interactive tool allows you to solve complex 2-SAT problems directly in your browser, with an intuitive interface and cutting-edge performance. 💻✨

---

## 🌟 Features

- 🧠 **Fast Problem Solving**: Leverages the power of Rust and WebAssembly for lightning-fast 2-SAT computations.
- 🖱️ **User-Friendly Interface**: Input your clauses with ease, and get instant results.
- 🎨 **Beautiful Design**: A sleek and modern UI for a seamless experience.
- 🌐 **Cross-Platform**: Runs in any modern web browser with WebAssembly support.
- 📦 **Compact & Lightweight**: Delivered as a tiny Wasm bundle with no external dependencies.

---

## 📖 What is 2-SAT?

**2-SAT (2-Satisfiability)** is a problem in computational complexity theory, where you determine if a set of boolean variables can be assigned in a way that satisfies all given clauses. Each clause is a disjunction of two literals (e.g., `(x1 ∨ ¬x2)`).

This project implements an efficient algorithm to solve 2-SAT problems using **strongly connected components (SCC)** in a directed graph.

---

## 🚀 Getting Started

Follow these steps to run the 2SAT solver locally on your machine:

### 1️⃣ Prerequisites
Ensure you have the following installed:
- 🦀 [Rust](https://www.rust-lang.org/)
- 📦 [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- 🌐 A local HTTP server (e.g., Python, Node.js, or any static file server).

---

### 2️⃣ Build the Project
Clone the repository and build the WebAssembly module:

```bash
git clone https://github.com/piter231/2sat.git
cd 2sat
wasm-pack build --target web
```

---

### 3️⃣ Serve Locally
Use a simple HTTP server to serve the files. For example:

```bash
python -m http.server
```

Open your browser and navigate to `http://0.0.0.0:8000`.

---

## 🖼️ Usage

1. **Enter Clauses**: In the input box, write each clause on a new line, formatted as:
   ```
   variable1 negated1 variable2 negated2
   ```
   - Example: `0 false 1 true` represents `(x0 ∨ ¬x1)`.

2. **Solve**: Click the `Solve` button to find the solution.

3. **View Output**: The result will display whether the problem is satisfiable and the variable assignments.

---

## 🛠️ Technical Details

- **Algorithm**: Based on strongly connected components (SCC) using Kosaraju's algorithm.
- **Languages**: Written in **Rust**, compiled to **WebAssembly**.
- **UI Framework**: Pure HTML/CSS with a touch of modern styling.

---

## 💻 Development

### Install Dependencies
Ensure you have Rust and wasm-bindgen set up:

```bash
cargo install wasm-bindgen-cli
```

### Run Tests
Run the unit tests to verify functionality:

```bash
cargo test
```

---

## 📚 Example Input & Output

### Input:
```
0 false 1 true
0 true 2 false
1 true 2 true
```

### Output:
```
Satisfiable!
Assignment: [true, false, true]
```

---

## 🤝 Contributing

We welcome contributions! Feel free to:
- Open issues for bugs or feature requests.
- Submit pull requests for improvements.

---

## 🛡️ License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 🎉 Acknowledgments

- Built with ❤️ using [Rust](https://www.rust-lang.org/) and [WebAssembly](https://webassembly.org/).
- Inspired by the elegance of graph algorithms.

---


💡 **Pro Tip**: Share this tool with your friends and solve boolean puzzles together! 🧠✨

--- 

Let me know if you'd like to personalize this further or add any additional sections!