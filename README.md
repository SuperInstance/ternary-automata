# Ternary Automata

**Ternary Automata** provides cellular automata with three states {0, 1, 2} — extending Stephen Wolfram's elementary CA framework to ternary logic with base-3 Wolfram numbering, pattern detection, cycle finding, and entropy evolution.

## Why It Matters

Cellular automata (CA) are the simplest known systems that produce complex emergent behavior from local rules. Wolfram's Rule 110 (binary) is Turing complete. Ternary CAs with 3^3 = 27 possible neighborhoods and 3^27 ≈ 7.6 trillion rules vastly expand the behavioral space. They model diffusion, pattern formation, and self-organization in physical systems — and map directly to ternary agent grids where each cell is an agent in one of three states {-1, 0, +1}.

## How It Works

### Rule Encoding

A ternary CA rule maps each of 27 possible neighborhoods to an output state:

```
Neighborhood index = left·9 + center·3 + right   (0..26)
Output = table[index] ∈ {0, 1, 2}
```

The **Wolfram number** encodes the rule table as a base-3 integer:

```
wolfram = Σ table[i] · 3^i   for i = 0..26
```

Maximum Wolfram number: 3^27 - 1 ≈ 7.6 × 10^12. Rule lookup: **O(1)** (array index).

### Simulation Step

```
next_cell(left, center, right) = rule.table[left·9 + center·3 + right]
```

One full step of width W: **O(W)** — each cell computes independently. Boundary conditions: periodic (wrap-around) or fixed (edge values).

### Pattern Detection

Detect fixed points, oscillators, and spaceships:

```
- Fixed point: config(t) == config(t+1)
- Oscillator period p: config(t) == config(t+p), p > 1
- Spaceship: config(t) shifted by Δ equals config(t+p)
```

Detection cost: **O(W · H)** for width W, history depth H.

### Entropy Evolution

Shannon entropy of the state distribution:

```
H(t) = -Σ p_i · log2(p_i)   where p_i = fraction of cells in state i
```

Maximum entropy: log2(3) ≈ 1.585 bits (uniform distribution). Entropy computation: **O(W)** per step.

### Cycle Detection

Floyd's cycle-finding algorithm on the configuration sequence:

```
Phase 1: tortoise and hare advance at different speeds until they meet
Phase 2: find cycle start and length
```

Cost: **O(μ + λ)** where μ = preperiod length, λ = cycle length. Space: **O(1)** (Floyd's) or **O(μ + λ)** (Brent's with history).

## Quick Start

```rust
use ternary_automata::TernaryRule;

let rule = TernaryRule::from_wolfram(3_434_567_890_123);
let initial = vec![0, 1, 2, 0, 1, 2, 0, 0, 0, 1];
let next = rule.step(&initial);

println!("Entropy: {:.3} bits", rule.entropy(&next));
```

## API

| Type | Description |
|------|-------------|
| `TernaryRule` | Rule table (27 entries) + Wolfram number |
| `TernaryRule::from_wolfram(n)` | Create rule from base-3 Wolfram number |
| `TernaryRule::from_table([u8; 27])` | Create from explicit lookup table |
| `step(config)` | Advance one time step |
| `apply_neighborhood(l, c, r)` | Single-cell lookup |
| `wolfram_number()` | Get the Wolfram number |
| `entropy(config)` | Shannon entropy of state distribution |

## Architecture Notes

Ternary Automata provides the spatial simulation substrate for agent grid models in SuperInstance. In γ + η = C, state 1 (+1) represents γ (growth — active agents), state 2 (-1) represents η (avoidance — inhibited agents), and state 0 represents the neutral equilibrium. Emergent patterns in CA dynamics model how agent strategies propagate spatially through the fleet.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md) for spatial dynamics architecture.

## References

1. Wolfram, S. (2002). *A New Kind of Science*. Wolfram Media.
2. Cook, M. (2004). "Universality in Elementary Cellular Automata." *Complex Systems*, 15(1), 1–40.
3. von Neumann, J. (1966). *Theory of Self-Reproducing Automata*. University of Illinois Press.

## License

MIT
