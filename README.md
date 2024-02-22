# Pictionary

Pictionary is a simple, interactive console game library written in Rust. It allows players to guess words selected randomly from a predefined list. This project aims to provide a basic understanding of Rust programming concepts including handling user input, random number generation, and basic Rust syntax. It's lightweight, easy to use, and perfect for those new to Rust or looking for a fun coding exercise.

## Features

- Random word selection from a predefined list.
- Console-based user interaction for guessing words.
- Simple and intuitive API for integrating into Rust projects.

## Getting Started

To use Pictionary in your project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
pictionary = "0.1.0"
```

Replace "0.1.0" with the latest version of Pictionary.

## Usage
Here's a simple example on how to use the Pictionary library to run a game:

```rust
use pictionary::play_pictionary;

fn main() {
    println!("Welcome to Pictionary!");
    play_pictionary();
}
```
## Development
To contribute to Pictionary or run it locally, clone the repository and build the project:

```sh
git clone https://github.com/yourgithub/pictionary.git
cd pictionary
cargo build
```
Run tests to ensure everything is working as expected:

```sh

cargo test
```

## Contributing
Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.