# Learn Rust
This repo is a result of learning RUST. I aim to start with basic terminologies and
build on it. 

My motivation to learn Rust is understand the TFHE-rs produced by ZAMA.

Sure! Rust is a systems programming language focused on safety, speed, and concurrency. Let's start with the basics and gradually move to more advanced topics.

### 1. **Introduction to Rust**

Rust is designed to be a safe language, meaning it prevents many common bugs that occur in languages like C or C++. Rust achieves memory safety without needing a garbage collector. It enforces safe memory usage at compile time.

### 2. **Setting Up**

1. **Install Rust**:
   - Rustup: The recommended way to install Rust is through `rustup`, a toolchain installer. Run the following command in your terminal:

     ```sh
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```

   - Follow the onscreen instructions to install Rust. Once installed, you can check the version with:

     ```sh
     rustc --version
     ```
2. **Hello, World! Program**:
   
       - Create a new file named main.rs:
        ```rust
        fn main() {
           println!("Hello, world!");
       }
       ```
       - Compile and run the program:
       ```sh
              rustc main.rs
              ./main
              
       ```
       

### 3. **Advanced Topics**

Once you're comfortable with the basics, you can explore more advanced topics:

- **Concurrency**: Rust has powerful concurrency primitives such as threads, async/await, and channels.
- **Traits**: Similar to interfaces in other languages, traits define shared behavior.
- **Macros**: Rust's metaprogramming feature allows you to write code that writes code.
- **Cargo**: Rust's package manager and build system help manage dependencies and project configuration.

