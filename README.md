# Terminal Call Tracker (TCT)

**TCT (Terminal Call Tracker)** is a versatile, Rust-based CLI tool that acts as a wrapper for any terminal command, enabling you to **track input and output** across different runs. Originally designed to add local history to [`tgpt`](https://github.com/cheusov/tgpt), TCT has evolved into a general-purpose tool ideal for **debugging**, **interactive CLI workflows**, or **experimenting with variable inputs** across command-line programs.

## 🔧 What It Does

* Wraps any terminal command and logs:

  * The **exact command** entered
  * The **full terminal output**
* Saves each run to a **timestamped text file**
* Lets you **browse and switch** between saved sessions via [`fzf`](https://github.com/junegunn/fzf)
* Creates a **local history** that’s searchable and reusable
* Ideal for use cases like:

  * Iterative prompt testing with tools like `tgpt`
  * Debugging shell scripts or CLI programs
  * Comparing outputs with different command arguments

## 🚀 Features

* 📝 Save each command and its output in a readable log file
* 🔍 Navigate saved sessions via `fzf`
* 🕹️ Works with any terminal command
* ⚡ Fast and compiled with Rust
* 💻 Minimal dependencies

## 📦 Installation

> ⚠️ Installation section under construction. Stay tuned!

## 🧪 Example Use Cases

* **ChatGPT Prompt Iteration**:

  * Test variations of a `tgpt` prompt, and review all outputs later.
* **Shell Script Debugging**:

  * Wrap your script calls and easily inspect the output over time.
* **API Call Testing**:

  * Track `curl` commands and responses as you refine headers or payloads.
* **CLI Argument Tweaking**:

  * Quickly change input flags and review the effects.

## 🗂️ Session Management

* All sessions are saved in a dedicated directory (e.g., `~/.tct/sessions`)
* Each file includes:

  * Timestamp
  * Full command
  * Full output
* You can open `fzf` to switch between or preview saved sessions

## 📌 Dependencies

* [`fzf`](https://github.com/junegunn/fzf) – for interactive session selection

## 📣 Why TCT?

TCT was born from a simple need: to have persistent local history for `tgpt`. But it quickly became a powerful general-purpose utility for **anyone who iterates over terminal commands** and wants visibility, structure, and traceability in their command-line workflows.

---

Would you like a visual diagram or example session included too?
