# Rusty Wassembly Game

Micro scale game prototyping with Rust programming language and WebAssembly (WASM).

## Concept

For fun and imaginary profit!

## State of this project

### Current state

Currently this project only implements a simple game of life heavily influenced by the book `https://rustwasm.github.io/docs/book/` which was used for learning WebAssembly basics. The example code was used as an inspiration, but all JavaScript functionality was implemented in Rust.

### Next step

The next step is to implement user interface utilizing event listeners.

### Final goal

The ultimate goal is to slowly transform this project into a simple 2d platformer game, one functionality at a time.

## Building this project

### Prerequisites

- Rust
- Node.js
- wasm-pack
  - https://rustwasm.github.io/wasm-pack/

### Setup

```
# Update to newest Rust (optional)
$ rustup update

# Generate wasm files (created to `./pkg/` folder)
$ wasm-pack build

# Install development web server
$ npm install

# Run development web server
$ npm run start

# Open browser and acccess localhost:8080
```
