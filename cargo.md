# Cargo Commands in Rust

- To create a new Rust project, use the following command:
  ```bash
  cargo new <project_name>
  ```
- To build a Rust project, use the following command:
  ```bash
  cargo build
  ```
- To build and run a Rust project in one step, use the following command:
  ```bash
  cargo run
  ```
- To build a Rust project without producing a binary (useful for checking errors), use the following command:
  ```bash
  cargo check
  ```
- Build documentation for a project
  ```bash
  cargo doc
  ```
- Publish a library to crates.io
  ```bash
  cargo publish
  ```
- Cargo stores the result of the build in the target/debug directory by default, instead of saving it in the same directory as our code.
