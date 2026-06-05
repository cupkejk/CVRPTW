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
At each step the solver randomly selects one of four neighborhood operators:
- **Intra-route 2-opt**: Reverses a segment within a route to untangle crossings.
- **Inter-route Relocate**: Moves a customer from one route to another.
- **Inter-route Swap**: Swaps a customer in one route with a customer in another.
- **Intra-route Swap**: Swaps two customers within the same route.

The algorithm heavily incentivizes minimizing the number of vehicles by applying a `VEHICLE_COST` penalty per active route during relocation. Moves are accepted if they improve the objective score or pass the probabilistic acceptance check: `e^(-Δ/T) > random(0,1)`.

The temperature cools by a factor of `0.99` every 100 iterations, creating plateaus that allow the search to explore local neighborhoods before cooling down. Empty routes are pruned dynamically.

### 2. Branch and Bound (Exact)
Systematically explores the search tree of all valid route permutations. It uses the best route found so far (seeded from the initial one-customer-per-route solution) as an upper bound to prune branches that cannot yield a better solution. The bounding condition accounts for both accumulated distance and the `VEHICLE_COST` penalty per open vehicle, matching the objective used by SA.

Progress is printed to stdout every 5,000,000 iterations. Note that this algorithm is computationally expensive and runs on the main thread, which may cause the window to freeze for larger problem sizes.

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
| `S` | Soft Reset (reset SA temperature and routes, keep customers) |
| `R` | Hard Reset (new random problem, 15 customers) |
| `Escape` | Exit the simulation |

---

## Visualization
Each vehicle route is drawn in a distinct color. The white square at the center is the depot; grey circles are customers. The HUD shows the current number of vehicles, total route distance, and SA temperature. When running the exact solver, a "Working..." overlay is displayed.

The window opens at **900 × 1000** pixels.

---

## Parameters

| Constant | Value | Description |
|---|---|---|
| `VEHICLE_CAPACITY` | `100.0` | Max demand load per vehicle |
| `DEPOT_TIME_WINDOW` | `4000.0` | Latest return time to depot |
| `VEHICLE_COST` | `2000.0` | Penalty cost per active vehicle in SA and B&B |
| `BORDER` | `50.0` | Screen edge padding for customer placement |
| Customer demand | `10–30` | Randomly sampled per customer |
| Customer time window | `[0–500, (dist_from_depot + 100)–4000]` | Randomly sampled ready/due times |
| Service time | `20.0` | Fixed time spent at each customer |
| Default customers | `15` | Starting problem size on launch or hard reset |

---

## Testing / Benchmarking
Setting the `TESTING` constant to `true` runs a headless benchmark instead of opening the interactive window. It instantiates problems of size 0–16 customers, solves each with B&B and then with SA (after a soft reset), and prints the vehicle count, total distance, and wall-clock time for every instance to stdout.

---

## Built With
- [Rust](https://www.rust-lang.org/)
- [Macroquad](https://macroquad.rs/) — cross-platform graphics
- [rand](https://docs.rs/rand) — random number generation