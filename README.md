# TCT (Terminal Chat Tool)

TCT is a terminal-based chat tool that allows users to interact with ChatGPT via the command line. The tool supports saving chat history, browsing previous sessions, and using an interactive fuzzy finder (`fzf`) to easily navigate between stored conversations.

## Features

- Start a new chat with ChatGPT.
- Save and load previous chat sessions.
- Browse saved chat files interactively using `fzf`.
- Minimal dependencies, written in Zig for fast, compiled execution.

## Prerequisites

Before running TCT, make sure you have the following installed:

- [tgpt](https://github.com/cheusov/tgpt) (for interacting with ChatGPT via the terminal)
- [fzf](https://github.com/junegunn/fzf) (for interactive fuzzy file selection)

### Installation

1. **Install tgpt**:
   - Clone the repository:
     ```bash
     git clone https://github.com/cheusov/tgpt.git
     cd tgpt
     go install
     ```
   - Verify installation:
     ```bash
     tgpt --help
     ```

2. **Install fzf**:
   - For Linux (Ubuntu/Debian):
     ```bash
     sudo apt-get install fzf
     ```
   - For macOS (Homebrew):
     ```bash
     brew install fzf
     ```
   - For Windows (via WSL):
     ```bash
     sudo apt-get install fzf
     ```

3. **Clone the TCT project**:
   ```bash
   git clone https://github.com/yourusername/tct.git
   cd tct
