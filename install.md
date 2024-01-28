# Rust Programming Language Guide

This repository contains a comprehensive guide for learning the Rust programming language. The guide is based on the official documentation provided by the Rust programming language team.

**[Rust Programming Language Book](https://doc.rust-lang.org/book/)**

## Rust Installation Guide

### Linux Installation

If you're using a Linux distribution, you can install Rust using the following `curl` command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

If you're using a Linux distribution with the `apt` package manager (e.g., Ubuntu, Debian), you may need to install C++ build tools:

1. Open a terminal.

2. Run the following command to install C++ build tools:

    ```bash
    sudo apt install build-essential
    ```

   This step is necessary for building certain Rust packages that rely on C++ components.