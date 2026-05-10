# CVRPTW Solver

An interactive visualizer for the **Capacitated Vehicle Routing Problem with Time Windows (CVRPTW)**, built with Rust and [Macroquad](https://macroquad.rs/).

Routes are optimized in real time using **Simulated Annealing** — watch vehicles find better paths as the temperature cools.

---

## Problem

Given a depot and a set of customers, find the minimum-distance set of vehicle routes such that:

- Every customer is visited exactly once
- Each vehicle starts and ends at the depot
- Vehicle **capacity** is not exceeded (`100` units per vehicle)
- Each customer is served within their **time window** `[ready, due]`
- All vehicles return to the depot within the global time window (`2000`)

---

## Algorithm — Simulated Annealing

At each step the solver picks a random customer from one route and tries to insert it into another. The move is accepted if it:

- Reduces total distance, **or**
- Passes the probabilistic acceptance check: `e^(-Δ/T) > random(0,1)`

Temperature cools by a factor of `0.99999` each iteration, gradually shifting from exploration to exploitation. Empty routes are pruned after each accepted move.

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
| `R` | Reset with a new random problem (30 customers) |
| `Escape` | Exit the simulation |

---

## Visualization

Each vehicle route is drawn in a distinct color. The white square at the center is the depot; grey circles are customers. The HUD shows the current number of vehicles, total route distance, and SA temperature.

---

## Parameters

| Constant | Value | Description |
|---|---|---|
| `VEHICLE_CAPACITY` | `100.0` | Max demand load per vehicle |
| `DEPOT_TIME_WINDOW` | `2000.0` | Latest return time to depot |
| `BORDER` | `50.0` | Screen edge padding for customer placement |
| Customer demand | `10–30` | Randomly sampled per customer |
| Customer time window | `[0–500, 600–2000]` | Randomly sampled ready/due times |
| Service time | `20.0` | Fixed time spent at each customer |

---

## Built With

- [Rust](https://www.rust-lang.org/)
- [Macroquad](https://macroquad.rs/) — cross-platform graphics
- [rand](https://docs.rs/rand) — random number generation
