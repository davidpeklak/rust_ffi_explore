# Rust FFI Explore - Async Programming Learning Project

## Project Overview

This is a **Rust learning project** that explores asynchronous programming concepts, particularly focused on building custom implementations of futures, reactors, executors, and event loops from scratch. The project is heavily inspired by the book ["Futures Explained in 200 Lines of Rust"](https://cfsamson.github.io/books-futures-explained) (which is unfortunately not available anymore) and serves as a hands-on exploration of low-level async programming concepts.

## Architecture & Components

The project is organized as a **Rust workspace** with 7 different modules, each demonstrating different aspects of async programming:

### 1. **`poll/`** - Low-level Event Polling
- Implements a custom polling mechanism using Linux `epoll` system calls
- Provides FFI bindings to Linux system calls for file I/O and event polling
- Contains a `Poll` struct that wraps epoll functionality
- Demonstrates basic event-driven programming with named pipes

### 2. **`reactor_executor/`** - Basic Reactor Pattern
- Implements a simple reactor-executor pattern using channels
- Shows how to separate concerns between event detection (reactor) and action execution (executor)
- Uses `std::sync::mpsc` channels for communication between reactor and executor
- Demonstrates callback-based event handling

### 3. **`future/`** - Custom Future Implementation
- Contains a full async runtime implementation with:
  - **`future.rs`**: Custom `ReadNChars` future that reads N characters from a file
  - **`executor.rs`**: Task executor that manages and runs futures
  - **`reactor.rs`**: Event reactor that handles I/O events and wakes futures
  - **`waker.rs`**: Custom waker implementation for notifying pending futures
- Implements the standard Rust `Future` trait
- Shows how async/await works under the hood

### 4. **`generator/`** - Generator Exploration
- Demonstrates the relationship between generators and futures
- Shows how state machines work in async programming
- Includes examples of self-referential data structures and their challenges
- Explores memory safety issues with generators

### 5. **`pinning/`** - Memory Pinning Concepts
- Explores Rust's `Pin` type and why it's necessary for async programming
- Demonstrates self-referential structs and their memory safety challenges
- Shows how pinning prevents memory from being moved

### 6. **`waker/`** - Manual Trait Object Construction
- Deep dive into how trait objects work in Rust
- Shows manual construction of fat pointers and vtables
- Demonstrates the low-level implementation details behind dynamic dispatch
- Educational example of unsafe Rust and memory layout

### 7. **`old_world/`** - Earlier Implementation
- Contains an earlier version of the async implementation
- Shows evolution of the codebase and different approaches

## Key Learning Topics Covered

1. **Event-driven Programming**: Using epoll for efficient I/O multiplexing
2. **Reactor Pattern**: Separating event detection from event handling
3. **Custom Async Runtimes**: Building futures, executors, and reactors from scratch
4. **Memory Safety**: Understanding pinning, self-referential structs, and unsafe code
5. **Trait Objects**: Low-level understanding of dynamic dispatch in Rust
6. **State Machines**: How async functions are compiled to state machines
7. **FFI (Foreign Function Interface)**: Interfacing with C system calls

## Educational Value

This project is a resource for understanding:
- How async/await works internally in Rust
- The relationship between futures, executors, and reactors
- Why certain async programming concepts (like pinning) are necessary
- Low-level systems programming and event handling
- The progression from callback-based to future-based async programming

The project demonstrates a complete journey from basic polling to a fully functional async runtime, making it valuable for developers wanting to understand the fundamentals of asynchronous programming in Rust.

---

# Usage
- Clone the project.
- Create two named pipes named pipe1 and pipe2:
```shell
mkfifo pipe1
mkfifo pipe2
```
- Run the project
```shell
cargo run
```
- In a second terminal, write into the named pipes, e.g.
```shell
echo foobar > pipe1
echo foobar > pipe2
```
- Keep writing into the named pipes. When aou have written >= 100 characters into one of the pipes, observe a message on the console of the running process:
```shell
Future1 finished with...
```
or
```shell
Future2 finished with...
```
# Literature
- [Futures Explained in 200 Lines of Rust](https://cfsamson.github.io/books-futures-explained)

