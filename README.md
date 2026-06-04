# ternary-automata

Cellular automata with ternary states — rules, simulation, pattern detection, and entropy evolution.

## Why This Exists

Binary cellular automata (like Wolfram's Rule 110) are well-studied, but ternary CAs open up a richer space: 3²⁷ ≈ 7.6 trillion possible rules vs. 2⁸ = 256 for binary. With three states per cell, you can model systems that have an absorbing middle state, three-way competition, or threshold dynamics. This crate provides ternary CA rules via Wolfram numbering, simulation with configurable boundary conditions, cycle detection, and entropy tracking — everything you need to explore the vast landscape of ternary cellular automata.

## Core Concepts

- **`TernaryRule`** — A CA rule mapping 3-cell neighborhoods (left, center, right) to output states {0, 1, 2}. With 27 possible neighborhoods and 3 possible outputs, rules are indexed by Wolfram numbers (base-3 integers from 0 to 3²⁷−1).
- **`CASimulator`** — Runs a CA rule on a 1D state array with configurable boundary conditions (fixed, periodic, or zero).
- **Boundary conditions** — `Fixed(v)` (constant boundary), `Periodic` (wrap-around), `Zero` (0-padded).
- **Wolfram numbering** — Base-3 encoding of the lookup table: neighborhood (left, center, right) encodes to index `l·9 + c·3 + r`.

## Quick Start

```toml
# Cargo.toml
[dependencies]
ternary-automata = "0.1"
```

```rust
use ternary_automata::*;

fn main() {
    // Use the built-in majority rule
    let rule = TernaryRule::majority();

    // Simulate with periodic boundaries
    let initial = vec![0, 1, 2, 0, 1, 2, 1, 1];
    let mut sim = CASimulator::new(rule, initial, Boundary::Periodic);

    // Run for 10 steps and print each generation
    let history = sim.run(10);
    for (i, state) in history.iter().enumerate() {
        println!("{:3}: {}", i, state_to_visual(state));
    }

    // Detect cycles
    let mut sim2 = CASimulator::new(TernaryRule::xor_rule(), vec![0, 1, 2], Boundary::Periodic);
    if let Some((preperiod, period)) = sim2.find_cycle(1000) {
        println!("Cycle: preperiod={}, period={}", preperiod, period);
    }

    // Track entropy evolution
    let mut sim3 = CASimulator::new(TernaryRule::rule_110_ternary(), vec![0, 1, 2, 0, 1], Boundary::Periodic);
    let entropy = sim3.entropy_evolution(20);
    println!("Entropy evolution: {:?}", entropy);
}
```

## API Overview

### Built-in Rules
- `TernaryRule::identity()` — Center cell passes through unchanged
- `TernaryRule::zero()` — Everything maps to 0
- `TernaryRule::majority()` — Output is the majority value in the neighborhood
- `TernaryRule::rule_110_ternary()` — Ternary analog of Wolfram's Rule 110: `(l + c + r) mod 3`
- `TernaryRule::xor_rule()` — Same as rule_110_ternary
- `TernaryRule::left_shift()` — Output = left neighbor value

### Custom Rules
- `TernaryRule::from_wolfram(number)` — Create from a Wolfram number (base-3 encoded)
- `TernaryRule::from_table(table)` — Create from a 27-entry lookup table
- `rule.wolfram_number()` — Get the Wolfram number
- `rule.table()` — Access the raw lookup table

### Simulation (`CASimulator`)
- `CASimulator::new(rule, initial_state, boundary)` — Create simulator
- `sim.step()` / `sim.step_n(n)` — Advance one or n generations
- `sim.run(steps)` — Collect all states over n steps
- `sim.state()` — Current state
- `sim.find_cycle(max_steps)` — Detect a cycle: returns `(preperiod, period)`
- `sim.entropy()` — Shannon entropy of current state
- `sim.entropy_evolution(steps)` — Track entropy over time

### Utilities
- `detect_pattern(history)` — Check if the last state repeats a previous one
- `state_to_string(state)` — Convert to "012" string
- `state_to_visual(state)` — Convert to block characters (░▒▓)
- `state_counts(state)` — Count occurrences of each state
- `hamming_distance(a, b)` — Count differing positions

## How It Works

**Rule encoding** uses a 27-entry lookup table indexed by `left·9 + center·3 + right`, where each of left, center, right is in {0, 1, 2}. The Wolfram number is the base-3 interpretation of this table read as a 27-digit number.

**Simulation** applies the rule to each cell simultaneously: for each position, the rule looks up (left, center, right) in its table. Boundary cells handle edges according to the configured boundary condition.

**Cycle detection** hashes each state and checks for repetition. If state S appears at steps i and j (i < j), the cycle has preperiod i and period j − i.

**Entropy** computes Shannon entropy over the state distribution: H = −Σ p(s) log₂ p(s), where p(s) is the fraction of cells in state s.

## Use Cases

1. **Complex systems research** — Explore the vast ternary rule space (7.6 trillion rules) for emergent behavior, self-organization, and computational universality.
2. **Ternary logic simulation** — Model ternary logic circuits as cellular automata for verification and analysis.
3. **Pattern formation** — Study how simple local rules produce global patterns in three-state systems (reaction-diffusion analogs).
4. **Generative art** — Use visually interesting ternary CA rules with the block-character renderer to create evolving patterns.

## Ecosystem

- [`ternary-regex`](https://github.com/user/ternary-regex) — Pattern matching on ternary sequences
- [`ternary-streaming`](https://github.com/user/ternary-streaming) — Streaming processing for ternary signals
- [`ternary-graph`](https://github.com/user/ternary-graph) — Graph algorithms on ternary-weighted edges

## License

MIT
