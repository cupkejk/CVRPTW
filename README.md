# CVRPTW Solver

An interactive visualizer for the **Capacitated Vehicle Routing Problem with Time Windows (CVRPTW)**, built with Rust and [Macroquad](https://macroquad.rs/).

Routes are optimized in real time using **Simulated Annealing** (metaheuristic) or can be solved optimally using **Branch and Bound** (exact algorithm).

---

## Problem

Given a depot and a set of customers, find the minimum-distance set of vehicle routes such that:

- Every customer is visited exactly once
- Each vehicle starts and ends at the depot
- Vehicle **capacity** is not exceeded (`100` units per vehicle)
- Each customer is served within their **time window** `[ready, due]`
- All vehicles return to the depot within the global time window (`4000`)

---

## Algorithms

### 1. Simulated Annealing (Heuristic)
At each step the solver picks a random customer from one route and tries to insert it into another. The move is accepted if it:
- Reduces total distance, **or**
- Passes the probabilistic acceptance check: `e^(-Δ/T) > random(0,1)`

Temperature cools by a factor of `0.99999` each iteration, gradually shifting from exploration to exploitation. Empty routes are pruned after each accepted move.

### 2. Branch and Bound (Exact)
Systematically explores the search tree of all valid route permutations. It uses the best route found so far as an upper bound to prune branches that cannot yield a better solution. Note that this algorithm is computationally expensive and runs on the main thread, capping at 5,000,000 iterations to prevent indefinite freezing.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)

### Run

```bash
cargo run --release
```

---

## Controls

| Key | Action |
|---|---|
| `Space` (hold) | Run 500 SA iterations per frame |
| `B` | Run Exact Algorithm (Branch and Bound) |
| `S` | Soft Reset (Reset SA temp and routes, keep cities) |
| `R` | Hard Reset (New random problem, 15 customers) |
| `Escape` | Exit the simulation |

---

## Visualization

Each vehicle route is drawn in a distinct color. The white square at the center is the depot; grey circles are customers. The HUD shows the current number of vehicles, total route distance, and SA temperature. When running the exact solver, a "Working..." overlay is displayed.

---

## Parameters

| Constant | Value | Description |
|---|---|---|
| `VEHICLE_CAPACITY` | `100.0` | Max demand load per vehicle |
| `DEPOT_TIME_WINDOW` | `4000.0` | Latest return time to depot |
| `BORDER` | `50.0` | Screen edge padding for customer placement |
| Customer demand | `10–30` | Randomly sampled per customer |
| Customer time window | `[0–500, 600–4000]` | Randomly sampled ready/due times |
| Service time | `20.0` | Fixed time spent at each customer |

---

## Built With

- [Rust](https://www.rust-lang.org/)
- [Macroquad](https://macroquad.rs/) — cross-platform graphics
- [rand](https://docs.rs/rand) — random number generation
