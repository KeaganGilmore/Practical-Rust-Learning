# Practical Rust Learning

This repository contains a Rust script designed to help students understand the basics of Rust practically. The script demonstrates various advanced Rust features, including ownership, borrowing, lifetimes, traits, generics, and concurrency.

## Features

1. **Traits and Implementations**:
    - `Summarizable` trait with a method `summary`.
    - `Book` and `Article` structs implementing the `Summarizable` trait.

2. **Generics**:
    - `print_summary` is a generic function that works with any type implementing `Summarizable`.

3. **Ownership and Borrowing**:
    - Demonstrated in the `ownership_and_borrowing` function.
    - Shows how ownership is moved and how borrowing works.

4. **Concurrency**:
    - Demonstrated in the `concurrency_example` function.
    - Uses `Arc` (Atomic Reference Counting) and `Mutex` for thread-safe shared state.
    - Spawns multiple threads to modify a shared `HashMap`.

## How to Run

1. **Clone the repository**:
   ```sh
   git clone https://github.com/yourusername/Practical.git
   cd Practical
   ```

2. **Build and run the project**:
   ```sh
   cargo run
   ```

## Dependencies

- `libc = "0.2.112"`

## License

This project is licensed under the MIT License.

---

Keagan Gilmore  
keagangilmore@gmail.com