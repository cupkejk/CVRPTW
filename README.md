# TSP in Rust

A visualization of various algorithms for solving the Traveling Salesperson Problem (TSP), built with Rust and the [Macroquad](https://macroquad.rs/) game engine.

## Features

- **Simulated Annealing (SA)**: A probabilistic technique for approximating the global optimum of a given function.
- **2-Opt**: A simple local search algorithm that removes "crosses" in the tour by swapping edges.
- **Held-Karp (Classical)**: An exact dynamic programming algorithm that finds the optimal solution (capped at 23 cities due to exponential complexity).
- **Interactive Visualization**: Watch the algorithms improve the tour in real-time.
- **Benchmarking Mode**: Compare the performance and accuracy of different algorithms.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)

### Running the Visualization

To start the interactive visualization:

```bash
cargo run --release
```

You can also specify the initial number of cities:

```bash
cargo run --release -- --cities 15
```

### Controls

- **`A`**: Start Simulated Annealing
- **`T`**: Start 2-Opt optimization
- **`C`**: Start Classical (Held-Karp) exact solver
- **`H`**: Generate new random cities and reset
- **`R`**: Randomize the current tour
- **`N`**: Add a new city at a random position
- **`S`**: Stop the currently running algorithm
- **`Esc`**: Exit the application

### Benchmarking

To run benchmarks and compare algorithm performance:

```bash
cargo run --release -- --test [max_cities]
```

Example: `cargo run --release -- --test 12`

## Implementation Details

- **Language**: Rust
- **Graphics**: Macroquad
- **Algorithms**:
  - `update_sa`: Reverses segments of the tour and accepts changes based on distance improvement or a temperature-based probability.
  - `update_two_opt`: Iteratively improves the tour by swapping edges if it results in a shorter path.
  - `update_classical`: Implements the Held-Karp dynamic programming approach with $O(2^n n^2)$ complexity.
